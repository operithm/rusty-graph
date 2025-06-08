#[derive(Debug)]
struct EdgeListGraph {
    edges: Vec<(usize, usize)>,
}

impl EdgeListGraph {
    fn new() -> Self {
        EdgeListGraph { edges: vec![] }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.push((u, v));
    }
}

fn main() {
    let mut graph = EdgeListGraph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);

    println!("{:?}", graph);
}