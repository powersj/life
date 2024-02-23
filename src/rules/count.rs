use crate::GRID_SIZE;

pub fn count_four_neighbors(grid: &[Vec<i32>], x: usize, y: usize) -> i32 {
    let mut count = 0;
    if x>=1 && grid[x-1][y] == 1 {
        count += 1;
    }
    if x+1 < GRID_SIZE && grid[x+1][y] == 1 {
        count += 1;
    }
    if y>=1 && grid[x][y-1] == 1 {
        count += 1;
    }
    if y+1 < GRID_SIZE && grid[x][y+1] == 1 {
        count += 1;
    }

    count
}

pub fn count_eight_neighbors(grid: &[Vec<i32>], x: usize, y: usize) -> i32 {
    let mut count = count_four_neighbors(grid, x, y);

    if x>=1 && y>=1 && grid[x-1][y-1] == 1 {
        count += 1;
    }
    if x>=1 && y+1 < GRID_SIZE && grid[x-1][y+1] == 1 {
        count += 1;
    }
    if x+1 < GRID_SIZE && y>=1 && grid[x+1][y-1] == 1 {
        count += 1;
    }
    if x+1 < GRID_SIZE && y+1 < GRID_SIZE && grid[x+1][y+1] == 1 {
        count += 1;
    }

    count
}
