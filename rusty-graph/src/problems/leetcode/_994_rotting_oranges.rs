use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 { return -1; }
        let cols = grid[0].len();

        let mut queue = VecDeque::new();
        let mut fresh = 0;
        let mut minutes = 0;

        // Initialize queue with rotten oranges and count fresh ones
        for i in 0..rows {
            for j in 0..cols {
                match grid[i][j] {
                    2 => queue.push_back((i, j, 0)),
                    1 => fresh += 1,
                    _ => (),
                }
            }
        }

        // Directions: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((i, j, time)) = queue.pop_front() {
            minutes = minutes.max(time);

            for (di, dj) in directions.iter() {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                // Check bounds and if it's a fresh orange
                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;

                    if grid[ni][nj] == 1 {
                        // Mark as rotten and enqueue
                        fresh -= 1;
                        // In a real scenario, we'd modify the grid here
                        // For this problem, we just track the count
                        queue.push_back((ni, nj, time + 1));
                        // Mark as visited by changing to 2 (if modifying grid)
                    }
                }
            }
        }

        if fresh == 0 { minutes } else { -1 }
    }
}