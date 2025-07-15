pub struct TarjanLCA {
    parent: Vec<usize>,      // For Union-Find
    ancestor: Vec<usize>,    // Marks visited ancestors
    visited: Vec<bool>,      // Tracks visited nodes
    queries: Vec<Vec<usize>>, // query[u] = list of v's to pair with u
    answers: Vec<Option<usize>>, // Stores LCA(u, v) results
}

impl TarjanLCA {
    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]); // Path compression
        }
        self.parent[u]
    }

    fn union(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u != root_v {
            self.parent[root_v] = root_u;
        }
    }

    fn tarjan_dfs(&mut self, u: usize, tree: &Tree) {
        self.ancestor[u] = u;
        for &child in &tree.nodes[u].children {
            self.tarjan_dfs(child, tree);
            self.union(u, child);
            self.ancestor[self.find(u)] = u;
        }
        self.visited[u] = true;

        // Answer all queries involving u
        for &v in &self.queries[u] {
            if self.visited[v] {
                let lca = self.ancestor[self.find(v)];
                self.answers.push((u, v, lca));
            }
        }
    }
}

pub fn process_queries(tree: &Tree, query_pairs: &[(usize, usize)]) -> Vec<(usize, usize, usize)> {
    let mut solver = TarjanLCA::new(tree.nodes.len());

    // Load queries (bidirectional)
    for &(u, v) in query_pairs {
        solver.queries[u].push(v);
        solver.queries[v].push(u);
    }

    solver.tarjan_dfs(tree.root, tree);
    solver.answers
}