use solution::Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() { return 0; }

        let mut grid = grid;
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(&mut grid, i, j);
                }
            }
        }

        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
            return;
        }

        grid[i][j] = '0';

        if i > 0 { Self::dfs(grid, i - 1, j); }
        if j > 0 { Self::dfs(grid, i, j - 1); }
        if i < grid.len() - 1 { Self::dfs(grid, i + 1, j); }
        if j < grid[0].len() - 1 { Self::dfs(grid, i, j + 1); }
    }
}