/// Euler Tour LCA using RMQ
pub struct EulerTourLCA {
    first_occurrence: Vec<usize>,
    euler_tour: Vec<usize>,
    depth: Vec<usize>,
    rmq: SparseTable,
}

impl EulerTourLCA {
    /// Preprocess in O(n) time (plus RMQ construction)
    pub fn new(tree: &Tree) -> Self {
        let n = tree.nodes.len();
        let mut euler_tour = Vec::with_capacity(2 * n);
        let mut first_occurrence = vec![0; n];
        let mut depth = vec![0; n];

        // Perform Euler Tour DFS
        let mut stack = vec![(tree.root, false)];
        while let Some((node, visited)) = stack.pop() {
            if visited {
                euler_tour.push(node);
                continue;
            }

            first_occurrence[node] = euler_tour.len();
            euler_tour.push(node);
            depth[node] = tree.nodes[node].depth;

            stack.push((node, true));
            for &child in tree.nodes[node].children.iter().rev() {
                stack.push((child, false));
            }
        }

        // Build RMQ on depths
        let depths: Vec<_> = euler_tour.iter().map(|&n| depth[n]).collect();
        let rmq = SparseTable::new(&depths);

        Self {
            first_occurrence,
            euler_tour,
            depth,
            rmq,
        }
    }

    /// Query LCA in O(1) time (with O(1) RMQ)
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let (l, r) = (
            self.first_occurrence[u],
            self.first_occurrence[v],
        );
        let (l, r) = if l < r { (l, r) } else { (r, l) };

        let idx = self.rmq.query(l, r);
        self.euler_tour[idx]
    }
}

/// Sparse Table for RMQ
struct SparseTable {
    table: Vec<Vec<usize>>,
    log: Vec<usize>,
}

impl SparseTable {
    fn new(data: &[usize]) -> Self {
        let n = data.len();
        let mut log = vec![0; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }

        let k = log[n] + 1;
        let mut table = vec![vec![0; n]; k];
        table[0] = data.to_vec();

        for j in 1..k {
            for i in 0..=n - (1 << j) {
                table[j][i] = std::cmp::min(
                    table[j-1][i],
                    table[j-1][i + (1 << (j-1))],
                );
            }
        }

        Self { table, log }
    }

    fn query(&self, l: usize, r: usize) -> usize {
        let len = r - l + 1;
        let k = self.log[len];
        std::cmp::min(
            self.table[k][l],
            self.table[k][r + 1 - (1 << k)],
        )
    }
}