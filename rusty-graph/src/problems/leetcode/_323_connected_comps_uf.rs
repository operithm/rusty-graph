use solution::{Solution, UnionFind};

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            uf.union(x, y);
        }

        uf.count as i32
    }
}