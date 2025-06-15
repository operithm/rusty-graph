use std::collections::VecDeque;
use solution::Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut queue = VecDeque::new();

        // Enqueue border 'O's
        for i in 0..rows {
            if board[i][0] == 'O' {
                queue.push_back((i, 0));
            }
            if board[i][cols - 1] == 'O' {
                queue.push_back((i, cols - 1));
            }
        }
        for j in 0..cols {
            if board[0][j] == 'O' {
                queue.push_back((0, j));
            }
            if board[rows - 1][j] == 'O' {
                queue.push_back((rows - 1, j));
            }
        }

        // BFS to mark connected 'O's
        while let Some((i, j)) = queue.pop_front() {
            if board[i][j] != 'O' {
                continue;
            }

            board[i][j] = 'T';

            if i > 0 { queue.push_back((i - 1, j)); }
            if j > 0 { queue.push_back((i, j - 1)); }
            if i < rows - 1 { queue.push_back((i + 1, j)); }
            if j < cols - 1 { queue.push_back((i, j + 1)); }
        }

        // Final processing
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    'O' => 'X',
                    'T' => 'O',
                    c => c,
                };
            }
        }
    }
}