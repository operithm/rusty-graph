use solution::Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        // Mark border 'O's and their connected regions as temporary 'T'
        for i in 0..rows {
            Self::dfs(board, i, 0);
            Self::dfs(board, i, cols - 1);
        }
        for j in 0..cols {
            Self::dfs(board, 0, j);
            Self::dfs(board, rows - 1, j);
        }

        // Flip remaining 'O's to 'X' and restore 'T's to 'O'
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= board.len() || j >= board[0].len() || board[i][j] != 'O' {
            return;
        }

        board[i][j] = 'T'; // Temporary mark

        // Explore four directions
        if i > 0 { Self::dfs(board, i - 1, j); }
        if j > 0 { Self::dfs(board, i, j - 1); }
        if i < board.len() - 1 { Self::dfs(board, i + 1, j); }
        if j < board[0].len() - 1 { Self::dfs(board, i, j + 1); }
    }
}