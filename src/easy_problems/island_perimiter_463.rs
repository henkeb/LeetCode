// You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.
//
// Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
//
// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
struct Solution;
type Coord = (usize, usize);

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let x_len = grid[0].len();
        let y_len = grid.len();
        let mut perimiter: i32 = 0;

        let bound_check_calculation =
            |&(x, y): &Coord, &(dx, dy): &(isize, isize)| -> Option<(usize, usize)> {
                x.checked_add_signed(dx)
                    .zip(y.checked_add_signed(dy))
                    .filter(|(x, y)| *x < x_len && *y < y_len)
            };

        for (y, row) in grid.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 1 {
                    let neighbours: Vec<(isize, isize)> = [(0, -1), (-1, 0), (1, 0), (0, 1)]
                        .into_iter()
                        .filter(|&(dx, dy)| bound_check_calculation(&(x, y), &(dx, dy)).is_some())
                        .collect();
                    perimiter += (4 - neighbours.len()) as i32;
                    perimiter += neighbours
                        .into_iter()
                        .map(|(dx, dy)| {
                            match grid[(y as isize + dy) as usize][(x as isize + dx) as usize] {
                                1 => 0,
                                0 => 1,
                                _ => unreachable!(),
                            }
                        })
                        .sum::<i32>();
                }
            }
        }
        perimiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        )
    }

    #[test]
    fn grid_mini() {
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4)
    }

    #[test]
    fn grid_mini2() {
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4)
    }
}
