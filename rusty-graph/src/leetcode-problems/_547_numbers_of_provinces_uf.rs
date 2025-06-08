struct Solution {}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            count: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
        self.count -= 1;
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    uf.union(i, j);
                }
            }
        }

        uf.count as i32
    }
}