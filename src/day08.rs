use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day08.txt", 1000)
}

#[allow(dead_code)]
pub fn part_2() -> f64 {
    read_from_v2("src/input/day08.txt")
}

fn distance(p1: &(f64, f64, f64), p2: &(f64, f64, f64)) -> f64 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2) + (p1.2 - p2.2).powi(2)
}


fn read_from(filepath: &str, iterations: usize) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let coordinates = sample.iter().map(|s| match s.split(",").collect::<Vec<&str>>().as_slice() {
        [x, y, z] => {
            (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap(), z.parse::<f64>().unwrap())
        },
        // The match is technically exhaustive for a [T; 3] array, 
        // so this line is technically unreachable but required because we're using a Vec
        _ => (0.0, 0.0, 0.0), // added for exhausitivity but should not be hit
    }).collect::<Vec<(f64,f64,f64)>>();
    let size = coordinates.len();
    let mut distances: Vec<(f64, ((f64,f64,f64), (f64,f64,f64)))> = vec![];
    // we build all the pair distances
    for i in 0..size-1 {
        for j in i+1..size-1 {
             let p1 = &coordinates[i];   
             let p2 = &coordinates[j]; 
             let distance= distance(p1, p2);
                distances.push((distance, (*p1, *p2)));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut circuits: Vec<Vec<(f64,f64,f64)>>= vec![];
    for i in 0..iterations-1 {
        let (_, (p1, p2)) = &distances[i];
        let snapshot = circuits.clone();
        let mut found_existing_circuit = false;
        let mut clean_up_index = -1;
        for (a,b) in vec![(p1, p2), (p2, p1)] {
            let circuit_group = circuits.iter_mut().find(|circuit| {
                circuit.contains(a)
            });
            match circuit_group {
                Some(circuit) => {
                    found_existing_circuit = true;
                    if circuit.contains(b) {
                        break;
                    }
                    // look if b is already in another circuit and merge them
                    let other_circuit_index = snapshot.iter().position(|c| c!=circuit && c.contains(b));
                    // remove other circuit
                    if let Some(index) = other_circuit_index {
                        let other_circuit = &snapshot[index];
                        clean_up_index = index as isize;
                        for p in other_circuit {
                            if !circuit.contains(p) {
                                circuit.push(*p);
                            }
                        }
                    } else {
                        circuit.push(*b);
                    }
                },
                None => {
                    // we can add (a, b) later if found_existing_circuit is still false
                }
            }
        }
        if clean_up_index > -1 {
            circuits.remove(clean_up_index as usize);
        }
        if !found_existing_circuit {
            let mut new_circuit = vec![];
            new_circuit.push(*p1);
            new_circuit.push(*p2);
            circuits.push(new_circuit);
        }
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits.iter().take(3).map(|v| v.len()).product::<usize>() as i64
}


fn read_from_v2(filepath: &str) -> f64 {
    let sample = helpers::read(filepath).unwrap();
    let coordinates = sample.iter().map(|s| match s.split(",").collect::<Vec<&str>>().as_slice() {
        [x, y, z] => (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap(), z.parse::<f64>().unwrap()),
        // The match is technically exhaustive for a [T; 3] array, 
        // so this line is technically unreachable but required because we're using a Vec
        _ => (0.0, 0.0, 0.0),
    }).collect::<Vec<(f64,f64,f64)>>();
    let size = coordinates.len();
    let mut distances: Vec<(f64, ((f64,f64,f64), (f64,f64,f64)))> = vec![];
    // we build all the pair distances
    for i in 0..size-1 {
        for j in i+1..size {
            let p1 = &coordinates[i];
            let p2 = &coordinates[j];
            let distance= distance(p1, p2);
            distances.push((distance, (*p1, *p2)));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut circuits: Vec<Vec<(f64,f64,f64)>>= vec![];
    let mut result = 0f64;
    for i in 0..distances.len() {
        let (_, (p1, p2)) = &distances[i];
        let snapshot = circuits.clone();
        let mut found_existing_circuit = false;
        let mut clean_up_index = -1;
        for (a,b) in vec![(p1, p2), (p2, p1)] {
            let circuit_group = circuits.iter_mut().find(|circuit| {
                circuit.contains(a)
            });
            match circuit_group {
                Some(circuit) => {
                    found_existing_circuit = true;
                    if circuit.contains(b) {
                        break;
                    }
                    // look if p2 is already in another circuit and merge them
                    let other_circuit_index = snapshot.iter().position(|c| c!=circuit && c.contains(p2));
                    // remove other circuit
                    if let Some(index) = other_circuit_index {
                        let other_circuit = &snapshot[index];
                        clean_up_index = index as isize;
                        for p in other_circuit {
                            if !circuit.contains(p) {
                                circuit.push(*p);
                            }
                        }
                    } else {
                        circuit.push(*b);
                    }
                },
                None => {
                    // we can add (a, b) later if found_existing_circuit is still false
                }
            }
        }
        if clean_up_index > -1 {
            circuits.remove(clean_up_index as usize);
        }
        if !found_existing_circuit {
            let mut new_circuit = vec![];
            new_circuit.push(*p1);
            new_circuit.push(*p2);
            circuits.push(new_circuit);
        }
        // once we have only one circuit with all points we can exit
        if circuits[0].len() == coordinates.len() {
            result = p1.0 * p2.0;
            break;
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 29406);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample08.txt", 10);
        assert_eq!(res, 40);
    }
    // 162,817,812 and 425,690,689
    // 162,817,812 and 431,825,988
    // 5,4,2

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample08.txt");
        assert_eq!(res, 25272f64);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 7499461416f64);
    }
}
