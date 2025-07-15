/// Binary Lifting LCA structure
pub struct BinaryLiftingLCA {
    up: Vec<Vec<usize>>,  // up[k][node] = 2^k-th ancestor
    depth: Vec<usize>,
    log_max: usize,
}

impl BinaryLiftingLCA {
    /// Preprocess in O(n log n) time
    pub fn new(tree: &Tree) -> Self {
        let n = tree.nodes.len();
        let log_max = (n as f64).log2().ceil() as usize + 1;
        let mut up = vec![vec![0; n]; log_max];
        let mut depth = vec![0; n];

        // Initialize first level (2^0 = 1)
        for node in 0..n {
            up[0][node] = tree.nodes[node].parent.unwrap_or(node);
            depth[node] = tree.nodes[node].depth;
        }

        // Build binary lifting table
        for k in 1..log_max {
            for node in 0..n {
                up[k][node] = up[k-1][up[k-1][node]];
            }
        }

        Self { up, depth, log_max }
    }

    /// Query LCA in O(log n) time
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        // Bring nodes to same depth
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // Jump u up to v's depth
        for k in (0..self.log_max).rev() {
            if self.depth[u] - (1 << k) >= self.depth[v] {
                u = self.up[k][u];
            }
        }

        if u == v {
            return u;
        }

        // Now jump both up together
        for k in (0..self.log_max).rev() {
            if self.up[k][u] != self.up[k][v] {
                u = self.up[k][u];
                v = self.up[k][v];
            }
        }

        self.up[0][u]
    }
}

struct BinaryLifter {
    up: Vec<Vec<Option<usize>>>, // up[depth][node]
    depth: Vec<usize>,
}

impl BinaryLifter {
    fn new(root: &BinaryTreeNode, node_count: usize) -> Self {
        let max_depth = (node_count as f64).log2().ceil() as usize;
        let mut lifter = BinaryLifter {
            up: vec![vec![None; node_count]; max_depth + 1],
            depth: vec![0; node_count],
        };
        lifter.dfs(root, None);
        lifter.preprocess();
        lifter
    }

    fn dfs(&mut self, node: &BinaryTreeNode, parent: Option<usize>) {
        let node_id = node.value;
        self.up[0][node_id] = parent;
        self.depth[node_id] = parent.map_or(0, |p| self.depth[p] + 1);

        if let Some(left) = &node.left {
            self.dfs(&left.borrow(), Some(node_id));
        }
        if let Some(right) = &node.right {
            self.dfs(&right.borrow(), Some(node_id));
        }
    }
}