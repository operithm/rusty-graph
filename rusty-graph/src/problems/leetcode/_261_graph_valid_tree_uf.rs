use solution::{Solution, UnionFind};

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() != n - 1 {
            return false; // Must have exactly n-1 edges for a tree
        }

        let mut uf = UnionFind::new(n);
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            if !uf.union(x, y) {
                return false; // Cycle detected
            }
        }
        true
    }
}