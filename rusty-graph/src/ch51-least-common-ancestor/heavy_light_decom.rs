/// Heavy-Light Decomposition for LCA and path queries
pub struct HLDLCA {
    parent: Vec<usize>,
    depth: Vec<usize>,
    head: Vec<usize>,  // Head of heavy path
    pos: Vec<usize>,   // Position in segment tree
    chain: Vec<usize>, // Chain ID
}

impl HLDLCA {
    /// Preprocess in O(n) time
    pub fn new(tree: &Tree) -> Self {
        let n = tree.nodes.len();
        let mut hld = Self {
            parent: vec![0; n],
            depth: vec![0; n],
            head: vec![0; n],
            pos: vec![0; n],
            chain: vec![0; n],
        };

        // First DFS to compute sizes
        let mut size = vec![1; n];
        let mut stack = vec![(tree.root, false)];
        while let Some((node, visited)) = stack.pop() {
            if visited {
                for &child in &tree.nodes[node].children {
                    size[node] += size[child];
                }
                continue;
            }
            stack.push((node, true));
            for &child in tree.nodes[node].children.iter().rev() {
                stack.push((child, false));
            }
        }

        // Second DFS to build HLD
        let mut current_chain = 0;
        let mut current_pos = 0;
        let mut stack = vec![(tree.root, tree.root)];
        while let Some((node, head)) = stack.pop() {
            hld.head[node] = head;
            hld.pos[node] = current_pos;
            current_pos += 1;
            hld.chain[node] = current_chain;

            if !tree.nodes[node].children.is_empty() {
                // Find heavy child
                let mut heavy_child = tree.nodes[node].children[0];
                for &child in &tree.nodes[node].children {
                    if size[child] > size[heavy_child] {
                        heavy_child = child;
                    }
                }

                // Continue current chain with heavy child
                stack.push((heavy_child, head));

                // Start new chains for other children
                for &child in &tree.nodes[node].children {
                    if child != heavy_child {
                        current_chain += 1;
                        stack.push((child, child));
                    }
                }
            }
        }

        hld
    }

    /// Query LCA in O(log n) time
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.chain[u] != self.chain[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                u = self.parent[self.head[u]];
            } else {
                v = self.parent[self.head[v]];
            }
        }

        if self.depth[u] < self.depth[v] { u } else { v }
    }
}