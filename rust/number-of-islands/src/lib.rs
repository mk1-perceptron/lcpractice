/// Number of Islands
///
/// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is
/// surrounded by water and is formed by connecting adjacent lands hroizontally or vertically. You
/// may assume all four edges of the grid are all surrounded by water.
use std::collections::HashSet;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut island_count = 0;
    let mut found_land = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if is_land(*c) && !found_land.contains(&(i, j)) {
                find_adjacent((i, j), &grid, &mut found_land);
                island_count += 1;
            }
        }
    }

    island_count
}

fn is_land(c: char) -> bool {
    c == '1'
}

fn find_adjacent(
    index: (usize, usize),
    grid: &[Vec<char>],
    found_land: &mut HashSet<(usize, usize)>,
) {
    let y_border = grid.len() - 1;
    let x_border = grid[0].len() - 1;
    let mut cur = Some(index);
    let mut stack = Vec::new();
    found_land.insert(index);

    while let Some(i) = cur {
        if i.0 > 0 {
            let up = (i.0 - 1, i.1);
            if is_land(grid[up.0][up.1]) && !found_land.contains(&up) {
                stack.push(up);
                found_land.insert(up);
            }
        }
        if i.0 < y_border {
            let down = (i.0 + 1, i.1);
            if is_land(grid[down.0][down.1]) && !found_land.contains(&down) {
                stack.push(down);
                found_land.insert(down);
            }
        }
        if i.1 > 0 {
            let left = (i.0, i.1 - 1);
            if is_land(grid[left.0][left.1]) && !found_land.contains(&left) {
                stack.push(left);
                found_land.insert(left);
            }
        }

        if i.1 < x_border {
            let right = (i.0, i.1 + 1);
            if is_land(grid[right.0][right.1]) && !found_land.contains(&right) {
                stack.push(right);
                found_land.insert(right);
            }
        }

        cur = stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let result = num_islands(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let input = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let result = num_islands(input);

        assert_eq!(result, 3);
    }
}
