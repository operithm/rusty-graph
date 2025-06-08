#[derive(Debug)]
struct MatrixGraph {
    matrix: Vec<Vec<usize>>,
    size: usize,
}

impl MatrixGraph {
    fn new(size: usize) -> Self {
        MatrixGraph {
            matrix: vec![vec![0; size]; size],
            size,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.size && v < self.size {
            self.matrix[u][v] = 1;
            // For undirected graph, add this line:
            self.matrix[v][u] = 1;
        }
    }
}

fn main() {
    let mut graph = MatrixGraph::new(4);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);

    println!("{:?}", graph.matrix);
}