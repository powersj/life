use crate::GRID_SIZE;

use super::count;

pub fn update_grid(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let count = count::count_eight_neighbors(grid, x, y);

            // Any live cell with fewer than two live neighbors dies, as if by under-population.
            // Any live cell with two or three live neighbors lives on to the next generation.
            // Any live cell with more than three live neighbors dies, as if by over-population.
            // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
            if grid[x][y] == 1 && !(2..=3).contains(&count){
                new_grid[x][y] = 0;
            } else if grid[x][y] == 0 && count == 3 {
                new_grid[x][y] = 1;
            } else {
                new_grid[x][y] = grid[x][y];
            }
        }
    }

    new_grid
}
