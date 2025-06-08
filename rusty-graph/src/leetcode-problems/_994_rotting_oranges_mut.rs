struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 { return -1; }
        let cols = grid[0].len();

        let mut queue = VecDeque::new();
        let mut fresh = 0;

        // Initialize
        for i in 0..rows {
            for j in 0..cols {
                match grid[i][j] {
                    2 => queue.push_back((i, j, 0)),
                    1 => fresh += 1,
                    _ => (),
                }
            }
        }

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut minutes = 0;

        while let Some((i, j, time)) = queue.pop_front() {
            minutes = time;

            for (di, dj) in directions.iter() {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;

                    if grid[ni][nj] == 1 {
                        grid[ni][nj] = 2;
                        fresh -= 1;
                        queue.push_back((ni, nj, time + 1));
                    }
                }
            }
        }

        if fresh == 0 { minutes } else { -1 }
    }
}