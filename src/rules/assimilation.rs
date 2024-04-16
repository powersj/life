use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B345/S4567";
pub const URL: &str = "https://conwaylife.com/wiki/OCA:Assimilation";
pub const DESCRIPTION: &str = "With a high enough starting cell count this rule \
causes the cells to grow and grow, or spawn outward";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);
            if (grid[x][y] == 0 && (3..=5).contains(&count)) ||
                (grid[x][y] == 1 && (4..=7).contains(&count)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
