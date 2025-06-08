use std::collections::HashMap;

#[derive(Debug)]
struct UndirectedGraph {
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list.entry(u).or_insert(vec![]).push(v);
        self.adjacency_list.entry(v).or_insert(vec![]).push(u);
    }
}

fn main() {
    let mut graph = UndirectedGraph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);

    println!("{:?}", graph);
}