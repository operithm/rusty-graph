use std::collections::HashMap;

#[derive(Debug)]
struct DirectedGraph {
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list.entry(from).or_insert(vec![]).push(to);
    }
}

fn main() {
    let mut graph = DirectedGraph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.add_edge(2, 3);

    println!("{:?}", graph);
}