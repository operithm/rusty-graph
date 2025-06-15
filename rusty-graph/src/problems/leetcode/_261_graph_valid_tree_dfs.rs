use solution::Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() != n - 1 {
            return false;
        }

        let mut adj = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut visited = vec![false; n];
        // Check for cycle starting from node 0
        if Self::has_cycle(&adj, &mut visited, 0, usize::MAX) {
            return false;
        }

        // Check if all nodes are connected
        visited.iter().all(|&v| v)
    }

    fn has_cycle(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize, parent: usize) -> bool {
        visited[node] = true;
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                if Self::has_cycle(adj, visited, neighbor, node) {
                    return true;
                }
            } else if neighbor != parent {
                return true;
            }
        }
        false
    }

}