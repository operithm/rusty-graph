struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut provinces = 0;

        for city in 0..n {
            if !visited[city] {
                provinces += 1;
                Self::dfs(&is_connected, &mut visited, city);
            }
        }

        provinces
    }

    fn dfs(is_connected: &[Vec<i32>], visited: &mut Vec<bool>, current: usize) {
        visited[current] = true;
        for (neighbor, &connected) in is_connected[current].iter().enumerate() {
            if connected == 1 && !visited[neighbor] {
                Self::dfs(is_connected, visited, neighbor);
            }
        }
    }

}