impl Tree {
    /// Naive LCA implementation using parent jumping
    /// Time: O(h) per query where h is tree height
    pub fn lca_naive(&self, mut u: usize, mut v: usize) -> usize {
        // Bring both nodes to the same depth
        while self.nodes[u].depth > self.nodes[v].depth {
            u = self.nodes[u].parent.unwrap();
        }
        while self.nodes[v].depth > self.nodes[u].depth {
            v = self.nodes[v].parent.unwrap();
        }

        // Now jump up until we find the common ancestor
        while u != v {
            u = self.nodes[u].parent.unwrap();
            v = self.nodes[v].parent.unwrap();
        }

        u
    }
}