use solution::Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' {
                    // Check if it's the first cell of the battleship
                    if (i == 0 || board[i-1][j] != 'X') &&
                        (j == 0 || board[i][j-1] != 'X') {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}