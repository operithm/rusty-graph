struct Solution {}

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut board = board;
        let mut count = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' {
                    count += 1;
                    Self::dfs(&mut board, i, j);
                }
            }
        }
        count
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= board.len() || j >= board[0].len() || board[i][j] != 'X' {
            return;
        }

        board[i][j] = '.';

        // Explore right and down (since battleships are either horizontal or vertical)
        Self::dfs(board, i + 1, j);
        Self::dfs(board, i, j + 1);
    }
}