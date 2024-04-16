use crate::GRID_SIZE;

use super::count;

pub const RULE: &str = "B35678/S5678";
pub const URL: &str = "https://conwaylife.com/wiki/OCA:Diamoeba";
pub const DESCRIPTION: &str = "This rule set tends to converge on larger shapes \
that look like amoeba cells. Where the outside of the larger shapes have outside \
legs that move and the inside stays alive.";

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if (grid[x][y] == 0 && (count == 3 || (5..=8).contains(&count))) ||
                (grid[x][y] == 1 && (5..=8).contains(&count)) {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
