use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B38/S238";
pub const URL: &str = "https://conwaylife.com/wiki/OCA:HoneyLife";
pub const DESCRIPTION: &str = "Like life, can create honeycomb like \
structures.";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if (grid[x][y] == 0 && (count == 3 || count == 8)) ||
                (grid[x][y] == 1 && ((2..=3).contains(&count) || count == 8)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
