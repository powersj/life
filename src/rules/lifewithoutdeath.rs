use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B3/S012345678";
pub const URL: &str = "https://en.wikipedia.org/wiki/Life_without_Death";
pub const DESCRIPTION: &str = "No death, cells with three neighbors continues \
to grow until there is no where else to grow.";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if (grid[x][y] == 0 && count == 3) ||
                (grid[x][y] == 1 && (0..=8).contains(&count)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
