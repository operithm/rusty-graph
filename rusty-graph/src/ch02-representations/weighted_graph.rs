use std::collections::HashMap;

#[derive(Debug)]
struct WeightedGraph {
    adjacency_list: HashMap<usize, Vec<(usize, i32)>>,
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.adjacency_list.entry(u).or_insert(vec![]).push((v, weight));
        // For undirected weighted graph, add the reverse edge too
        self.adjacency_list.entry(v).or_insert(vec![]).push((u, weight));
    }
}

fn main() {
    let mut graph = WeightedGraph::new();
    graph.add_edge(0, 1, 4);
    graph.add_edge(0, 2, 1);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 3, 5);

    println!("{:?}", graph);
}