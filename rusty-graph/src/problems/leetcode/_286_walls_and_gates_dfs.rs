use solution::Solution;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        if rooms.is_empty() || rooms[0].is_empty() {
            return;
        }

        let rows = rooms.len();
        let cols = rooms[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if rooms[i][j] == 0 {
                    Self::dfs(rooms, i, j, 0);
                }
            }
        }
    }

    fn dfs(rooms: &mut Vec<Vec<i32>>, i: usize, j: usize, distance: i32) {
        if i >= rooms.len() || j >= rooms[0].len() || rooms[i][j] < distance {
            return;
        }

        rooms[i][j] = distance;

        if i > 0 { Self::dfs(rooms, i - 1, j, distance + 1); }
        if j > 0 { Self::dfs(rooms, i, j - 1, distance + 1); }
        if i < rooms.len() - 1 { Self::dfs(rooms, i + 1, j, distance + 1); }
        if j < rooms[0].len() - 1 { Self::dfs(rooms, i, j + 1, distance + 1); }
    }
}