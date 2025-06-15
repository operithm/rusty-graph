use solution::Solution;
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

struct Solution {}
impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return false; // Cycle detected
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
        true
    }
}

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