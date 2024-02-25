use crate::GRID_SIZE;

use super::count;

// https://conwaylife.com/wiki/OCA:Vote
// B5678/S45678
pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            if grid[x][y] == 0 && (5..=8).contains(&count) {
                new_grid[x][y] = 1;
            } else if grid[x][y] == 1 && (4..=8).contains(&count){
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = 0;
            }
        }
    }

    new_grid
}
