use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

/// Enum to represent edge direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    Directed,
    Undirected,
}

/// Trait for edge weight
pub trait Weight: Clone + Debug + PartialOrd + Default {}
impl<T: Clone + Debug + PartialOrd + Default> Weight for T {}

/// Generic Graph structure
#[derive(Debug)]
pub struct Graph<V, E = ()>
where
    V: Eq + Hash + Clone + Debug,
    E: Weight,
{
    vertices: HashSet<V>,
    edges: Vec<(V, V, E)>,
    direction: EdgeDirection,
    representation: GraphRepresentation<V, E>,
}

/// Different graph representations
#[derive(Debug)]
enum GraphRepresentation<V, E>
where
    V: Eq + Hash + Clone + Debug,
    E: Weight,
{
    AdjacencyList(HashMap<V, Vec<(V, E)>>),
    AdjacencyMatrix(Vec<Vec<Option<E>>>),
    EdgeList,
}

impl<V, E> Graph<V, E>
where
    V: Eq + Hash + Clone + Debug,
    E: Weight,
{
    /// Create a new graph with specified parameters
    pub fn new(
        direction: EdgeDirection,
        representation: GraphRepresentation<V, E>,
    ) -> Self {
        Graph {
            vertices: HashSet::new(),
            edges: Vec::new(),
            direction,
            representation,
        }
    }

    /// Add a vertex to the graph
    pub fn add_vertex(&mut self, vertex: V) {
        self.vertices.insert(vertex);
        self.update_representation();
    }

    /// Add an edge between two vertices with optional weight
    pub fn add_edge(&mut self, from: V, to: V, weight: E) {
        self.vertices.insert(from.clone());
        self.vertices.insert(to.clone());
        self.edges.push((from.clone(), to.clone(), weight.clone()));

        if self.direction == EdgeDirection::Undirected {
            self.edges.push((to, from, weight));
        }

        self.update_representation();
    }

    /// Update the internal representation based on edges
    fn update_representation(&mut self) {
        match &mut self.representation {
            GraphRepresentation::AdjacencyList(map) => {
                *map = HashMap::new();
                for (from, to, weight) in &self.edges {
                    map.entry(from.clone())
                        .or_insert_with(Vec::new)
                        .push((to.clone(), weight.clone()));
                }
            }
            GraphRepresentation::AdjacencyMatrix(matrix) => {
                let vertices: Vec<V> = self.vertices.iter().cloned().collect();
                let size = vertices.len();
                *matrix = vec![vec![None; size]; size];

                for (from, to, weight) in &self.edges {
                    let from_idx = vertices.iter().position(|v| v == from).unwrap();
                    let to_idx = vertices.iter().position(|v| v == to).unwrap();
                    matrix[from_idx][to_idx] = Some(weight.clone());
                }
            }
            GraphRepresentation::EdgeList => {
                // Edge list is already maintained in self.edges
            }
        }
    }

    /// Get neighbors of a vertex
    pub fn neighbors(&self, vertex: &V) -> Vec<(&V, &E)> {
        match &self.representation {
            GraphRepresentation::AdjacencyList(map) => {
                map.get(vertex)
                    .map(|neighbors| neighbors.iter().map(|(v, w)| (v, w)).collect())
                    .unwrap_or_else(Vec::new)
            }
            GraphRepresentation::AdjacencyMatrix(matrix) => {
                let vertices: Vec<&V> = self.vertices.iter().collect();
                if let Some(from_idx) = vertices.iter().position(|&v| v == vertex) {
                    vertices.iter()
                        .enumerate()
                        .filter_map(|(to_idx, v)| {
                            matrix[from_idx][to_idx].as_ref().map(|w| (*v, w))
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
            GraphRepresentation::EdgeList => {
                self.edges.iter()
                    .filter(|(from, _, _)| from == vertex)
                    .map(|(_, to, weight)| (to, weight))
                    .collect()
            }
        }
    }

    /// Check if the graph contains a vertex
    pub fn contains_vertex(&self, vertex: &V) -> bool {
        self.vertices.contains(vertex)
    }

    /// Check if an edge exists between two vertices
    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        match &self.representation {
            GraphRepresentation::AdjacencyList(map) => {
                map.get(from).map_or(false, |neighbors| {
                    neighbors.iter().any(|(v, _)| v == to)
                })
            }
            GraphRepresentation::AdjacencyMatrix(matrix) => {
                let vertices: Vec<&V> = self.vertices.iter().collect();
                if let (Some(from_idx), Some(to_idx)) = (
                    vertices.iter().position(|&v| v == from),
                    vertices.iter().position(|&v| v == to),
                ) {
                    matrix[from_idx][to_idx].is_some()
                } else {
                    false
                }
            }
            GraphRepresentation::EdgeList => {
                self.edges.iter().any(|(f, t, _)| f == from && t == to)
            }
        }
    }

    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// Builder pattern for Graph
pub struct GraphBuilder<V, E>
where
    V: Eq + Hash + Clone + Debug,
    E: Weight,
{
    direction: EdgeDirection,
    representation: GraphRepresentation<V, E>,
    vertices: Vec<V>,
    edges: Vec<(V, V, E)>,
}

impl<V, E> GraphBuilder<V, E>
where
    V: Eq + Hash + Clone + Debug,
    E: Weight,
{
    pub fn new() -> Self {
        Self {
            direction: EdgeDirection::Undirected,
            representation: GraphRepresentation::AdjacencyList(HashMap::new()),
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn directed(mut self) -> Self {
        self.direction = EdgeDirection::Directed;
        self
    }

    pub fn undirected(mut self) -> Self {
        self.direction = EdgeDirection::Undirected;
        self
    }

    pub fn with_adjacency_list(mut self) -> Self {
        self.representation = GraphRepresentation::AdjacencyList(HashMap::new());
        self
    }

    pub fn with_adjacency_matrix(mut self) -> Self {
        self.representation = GraphRepresentation::AdjacencyMatrix(Vec::new());
        self
    }

    pub fn with_edge_list(mut self) -> Self {
        self.representation = GraphRepresentation::EdgeList;
        self
    }

    pub fn add_vertex(mut self, vertex: V) -> Self {
        self.vertices.push(vertex);
        self
    }

    pub fn add_edge(mut self, from: V, to: V, weight: E) -> Self {
        self.edges.push((from, to, weight));
        self
    }

    pub fn build(self) -> Graph<V, E> {
        let mut graph = Graph::new(self.direction, self.representation);

        for vertex in self.vertices {
            graph.add_vertex(vertex);
        }

        for (from, to, weight) in self.edges {
            graph.add_edge(from, to, weight);
        }

        graph
    }
}