use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        if rooms.is_empty() || rooms[0].is_empty() {
            return;
        }

        let rows = rooms.len();
        let cols = rooms[0].len();
        let mut queue = VecDeque::new();

        // Enqueue all gates (0s)
        for i in 0..rows {
            for j in 0..cols {
                if rooms[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        // Directions: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((i, j)) = queue.pop_front() {
            for (di, dj) in directions.iter() {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                // Check bounds and if it's an empty room
                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;

                    if rooms[ni][nj] == i32::MAX {
                        rooms[ni][nj] = rooms[i][j] + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
    }
}