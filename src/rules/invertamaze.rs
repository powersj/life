use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B028/S0124";
pub const URL: &str = "https://conwaylife.com/wiki/OCA:Invertamaze";
pub const DESCRIPTION: &str = "A strobing rule set, which produces a maze like \
structure.";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if (grid[x][y] == 0 && (count == 0 || count == 2 || count == 8)) ||
                (grid[x][y] == 1 && ((0..=2).contains(&count) || count == 4)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
