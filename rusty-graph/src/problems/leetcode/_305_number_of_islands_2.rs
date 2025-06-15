use solution::Solution;

impl Solution {
    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let m = m as usize;
        let n = n as usize;
        let size = m * n;
        let mut uf = UnionFind::new(size);
        let mut grid = vec![false; size];
        let mut result = Vec::with_capacity(positions.len());
        let mut count = 0;

        for pos in positions {
            let row = pos[0] as usize;
            let col = pos[1] as usize;
            let index = row * n + col;

            // Skip if already land
            if grid[index] {
                result.push(count);
                continue;
            }

            grid[index] = true;
            count += 1;

            // Check four neighbors
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                    let neighbor_index = new_row as usize * n + new_col as usize;
                    if grid[neighbor_index] {
                        if uf.union(index, neighbor_index) {
                            count -= 1;
                        }
                    }
                }
            }

            result.push(count);
        }

        result
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

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
            return false;
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