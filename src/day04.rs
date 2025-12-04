use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day04.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day04.txt")
}

fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let grid = sample
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    get_accessible_rolls(grid)
}

fn get_accessible_rolls(grid: Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid.get(0).unwrap().len() as isize;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '.' {
                continue;
            }
            let neighbor_offsets = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let mut adjacent_count = 0;
            for (delta_i, delta_j) in neighbor_offsets.iter() {
                let n_row = i as isize + delta_i;
                let n_col = j as isize + delta_j;
                // check bounds
                if n_row >= 0 && n_row < rows && n_col >= 0 && n_col < cols {
                    let n_row_usize = n_row as usize;
                    let n_col_usize = n_col as usize;
                    if grid[n_row_usize][n_col_usize] == '@' {
                        adjacent_count = adjacent_count + 1;
                    }
                }
            }
            if adjacent_count < 4 {
                count = count + 1;
            }
        }
    }
    count
}

fn remove_accessible_rolls(grid: Vec<Vec<char>>, total_removed: i64) -> i64 {
    let mut updated_grid: Vec<Vec<char>> = grid.iter()
        .map(|row| row.clone())
        .collect();
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid.get(0).unwrap().len() as isize;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != '@' {
                continue;
            }
            let neighbor_offsets = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let mut adjacent_count = 0;
            for (delta_i, delta_j) in neighbor_offsets.iter() {
                let n_row = i as isize + delta_i;
                let n_col = j as isize + delta_j;
                // check bounds
                if n_row >= 0 && n_row < rows && n_col >= 0 && n_col < cols {
                    let n_row_usize = n_row as usize;
                    let n_col_usize = n_col as usize;

                    if grid[n_row_usize][n_col_usize] == '@' {
                        adjacent_count = adjacent_count + 1;
                    }
                }
            }
            if adjacent_count < 4 {
                updated_grid[i][j] = 'X';
                count = count + 1;
            }
        }
    }
    if count == 0 {
        return total_removed
    }
    remove_accessible_rolls(updated_grid, total_removed + count)
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let grid = sample
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    remove_accessible_rolls(grid, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 1523);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample04.txt");
        assert_eq!(res, 13);
    }

    #[test]
    fn test_parse_row() {
        let res = get_accessible_rolls(vec![
            vec!['.', '.', '@', '@', '.', '@', '.'],
            vec!['.', '.', '@', '@', '@', '@', '.'],
            vec!['.', '.', '@', '@', '.', '@', '.'],
        ]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample04.txt");
        assert_eq!(res, 43);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 9290);
    }
}
