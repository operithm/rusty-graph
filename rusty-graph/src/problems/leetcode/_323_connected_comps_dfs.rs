use solution::Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];

        // Build adjacency list
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut visited = vec![false; n];
        let mut count = 0;

        for i in 0..n {
            if !visited[i] {
                count += 1;
                Self::dfs(&adj, &mut visited, i);
            }
        }

        count
    }

    fn dfs(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize) {
        visited[node] = true;
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                Self::dfs(adj, visited, neighbor);
            }
        }
    }
}