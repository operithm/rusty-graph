# **Appendix A: Rust Basics for Graph Algorithms**

This appendix covers essential Rust concepts for implementing graph algorithms efficiently.
*(Assumes basic Rust syntax knowledge; focuses on graph-relevant patterns.)*

------

## **1. Data Structures for Graphs**

### **1.1 Representing Nodes and Edges**

```
// Basic node with ID and data
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub data: T,  // e.g., "User", "City"
}

// Edge with weight
pub struct Edge<W> {
    pub from: usize,
    pub to: usize,
    pub weight: W,  // i32, f64, etc.
}
```



### **1.2 Adjacency List (Most Common)**

```
use std::collections::HashMap;

// Unweighted graph
type UnweightedGraph = HashMap<usize, Vec<usize>>;

// Weighted graph
type WeightedGraph = HashMap<usize, Vec<(usize, f64)>>;
```



### **1.3 Generic Graph Structure**

```
pub struct Graph<N, E> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<E>>,
}
```

------

## **2. Ownership & Borrowing in Graph Context**



### **2.1 Avoiding Ownership Issues**

```
// Correct: Borrowing nodes for traversal
fn dfs(graph: &HashMap<usize, Vec<usize>>, start: usize) {
    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];
    
    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            for neighbor in &graph[&node] {  // Borrow neighbors
                stack.push(*neighbor);
            }
        }
    }
}
```



### **2.2 Using `Rc<RefCell>` for Shared Mutability**

```
use std::rc::Rc;
use std::cell::RefCell;

// For complex graphs with shared references
type SharedNode = Rc<RefCell<Node>>;
type SharedEdge = Rc<RefCell<Edge>>;
```

------

## **3. Traits for Graph Algorithms**



### **3.1 Custom `Display` for Debugging**

```
impl<N: std::fmt::Debug> std::fmt::Display for Node<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Node {}: {:?}", self.id, self.data)
    }
}
```



### **3.2 Equality for Nodes**

```
impl<N: PartialEq> PartialEq for Node<N> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id  // Compare by ID only
    }
}
```

------



## **4. Performance-Critical Patterns**



### **4.1 Preallocating Memory**

```
let mut adjacency_list: Vec<Vec<usize>> = Vec::with_capacity(1000);
```



### **4.2 Using `VecDeque` for BFS**

```
use std::collections::VecDeque;

fn bfs(graph: &[Vec<usize>], start: usize) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
}
```



### **4.3 BitSet for Compact Visited Tracking**

```
use bit_vec::BitVec;  // Add to Cargo.toml

let mut visited = BitVec::from_elem(graph.len(), false);
```

------



## **5. Error Handling in Graphs**



### **5.1 Handling Missing Nodes**

```
fn get_node(graph: &Graph, id: usize) -> Option<&Node> {
    graph.nodes.iter().find(|n| n.id == id)
}
```



### **5.2 Cycle Detection Result**

```
#[derive(Debug)]
pub enum GraphError {
    CycleDetected,
    NodeNotFound,
}
```

------



## **6. Example: Generic Graph Traversal**



```
pub trait GraphTraversal {
    type Node;
    type Edge;

    fn neighbors(&self, node: &Self::Node) -> Vec<&Self::Node>;
}

impl<N, E> GraphTraversal for Graph<N, E> {
    type Node = Node<N>;
    type Edge = Edge<E>;

    fn neighbors(&self, node: &Self::Node) -> Vec<&Self::Node> {
        self.edges.iter()
            .filter(|e| e.from == node.id)
            .map(|e| &self.nodes[e.to])
            .collect()
    }
}
```

------



## **7. Testing Graph Code**



### **7.1 Unit Test Example**

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![2]);
        
        bfs(&graph, 0);
        // Assert expected traversal order
    }
}
```



### **7.2 Benchmarking with Criterion**

Cargo.toml

```
# Cargo.toml
[dev-dependencies]
criterion = "0.4"
```

```
use criterion::{black_box, criterion_group, criterion_main};

fn bench_dijkstra(c: &mut criterion::Criterion) {
    let graph = build_large_graph();
    c.bench_function("dijkstra", |b| {
        b.iter(|| dijkstra(black_box(&graph), 0))
    });
}

criterion_group!(benches, bench_dijkstra);
criterion_main!(benches);
```

------

## **8. Memory Layout Diagrams for Rust Graph Structures**



### 8.1 Adjacency List (`Vec<Vec<usize>>`)

```
Stack                          Heap
+-------------------+          +---+---+---+---+---+---+
| adjacency_list    | -------> |ptr|len|cap|ptr|len|cap| (Vec headers)
+-------------------+          +---+---+---+---+---+---+
                               |   |       |
                               v   v       v
                              +---+---+   +---+---+
                              | 1 | 2 |   | 2 |   | (Edge lists)
                              +---+---+   +---+---+
```

- **Pros**: Cache-friendly for traversal (edges stored contiguously).
- **Cons**: 24-byte overhead per `Vec` header (`ptr`, `len`, `cap`).



### 8.**2 Compressed Sparse Row (CSR)**

```
+-------------------+          +---+---+---+---+---+
| edges: Vec<usize> | -------> | 1 | 2 | 2 |   |...| (All edges)
+-------------------+          +---+---+---+---+---+
| offsets: Vec<usize>| ------> | 0 | 2 | 3 |...|   | (Node pointers)
+-------------------+          +---+---+---+---+---+
```

- **Pros**: No per-node overhead, ideal for static graphs.
- **Cons**: Expensive to modify (requires rebuilding offsets).



### 8.**3 `HashMap<usize, Vec<(usize, f64)>>` (Weighted)**

```
+-------------------+          +---------------+---------------+
| hashmap           | -------> | Bucket 0      | Bucket 1      |...
+-------------------+          +---------------+---------------+
                               | K: 1 | V: ptr | K: 2 | V: ptr |
                               +---------------+---------------+
                                   |                   |
                                   v                   v
                              +---+---+---+       +---+---+---+
                              |(2,0.5)|(3,1.2)|   |(1,0.8)|   |
                              +---+---+---+       +---+---+---+
```

- **Pros**: Flexible node IDs (non-contiguous).
- **Cons**: Hash collisions hurt performance; 8-byte overhead per `(usize, f64)`.



### 8.**4 Struct-of-Arrays (SoA)**

```
+-------------------+          +---+---+---+---+
| nodes: Vec<Node>  | -------> |N0|N1|N2|...|  (Contiguous)
+-------------------+          +---+---+---+---+
| edges: Vec<Edge>  | -------> |E0|E1|E2|...|  (Contiguous)
+-------------------+          +---+---+---+---+
```

- **Pros**: Optimal for SIMD/cache locality.
- **Cons**: Complex to manage node/edge relationships.

------



### 8.5 **Key Memory Optimization Tips**

1. **Use `Vec` for dense graphs** (better cache locality than `HashMap`).

2. **Preallocate** with `Vec::with_capacity()` to avoid reallocations.

3. **Consider `smallvec`** for nodes with few edges (≤8 edges/node):

   ```
   use smallvec::{SmallVec, smallvec};
   type CompactAdjList = Vec<SmallVec<[usize; 8]>>;  // Stores edges inline until 8
   ```

4. **For immutable graphs**, CSR or **`&'static [usize]`** reduce overhead.

------



### 8.6 **Visual Comparison**

| Structure         | Node Access | Edge Access | Memory Overhead | Mutability |
| :---------------- | :---------- | :---------- | :-------------- | :--------- |
| `Vec<Vec<usize>>` | O(1)        | O(1)        | 24B/node        | Easy       |
| CSR               | O(1)        | O(1)        | 8B/node         | Hard       |
| `HashMap`         | O(1)*       | O(1)        | 16B/node + hash | Moderate   |
| Struct-of-Arrays  | O(1)        | O(1)        | 0B/node         | Hard       |

*(*) Amortized, depends on hash collisions.*

------



### 8.7 **When to Use Which?**

- **Competitive Programming**: `Vec<Vec<usize>>` (simplicity).
- **Real-World Graphs (Sparse)**: `HashMap` or CSR.
- **High-Performance Computing**: Struct-of-Arrays + SIMD.

------



### 8.8 **Example: Benchmarking Memory Usage**

```
#[test]
fn test_memory_layout() {
    let adj_list = vec![vec![1, 2], vec![2], vec![]];
    let csr_edges = vec![1, 2, 2];
    let csr_offsets = vec![0, 2, 3];
    
    assert_eq!(std::mem::size_of_val(&adj_list), 24);
    assert_eq!(std::mem::size_of_val(&csr_edges), 24);
}
```

------

## **Key Takeaways**

1. **Use `&` references** for traversal to avoid cloning.
2. **Generic designs** make algorithms reusable across graph types.
3. **`VecDeque` and `BitVec`** optimize BFS/DFS performance.
4. **Traits** enable polymorphic graph operations.