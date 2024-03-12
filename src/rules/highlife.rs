use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B36/S23";
pub const URL: &str = "https://conwaylife.com/wiki/OCA:HighLife";
pub const DESCRIPTION: &str = "";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if (grid[x][y] == 0 && (count == 3 || count == 6)) ||
                (grid[x][y] == 1 && (2..=3).contains(&count)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
