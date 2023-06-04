![test](https://github.com/code-sam/stacked_linear_algebra_graph/actions/workflows/test_main_branch.yml/badge.svg?branch=main)
# Stacked Linear Algebra Graph
An embedded and in-memory graph using sparse linear algebra.

## Capabilities

### Architecture
The Stacked Linear Algebra Graph implements a [directed graph](https://en.wikipedia.org/wiki/Directed_graph) using GrapBLAS sparse linear algebra.
The graph models vertices and adjacency matrices as GraphBLAS sparse vectors and matrices respectively. 
The graph operates on its vertex vectors and adjacency matrices using GraphBLAS operators.

### Data types
The graph stores Rust primitive numeric types in its vertices and edges:
- bool,
- i8
- i16
- i32
- i64
- u8
- u16
- u32
- u64
- f32,
- f64,
- isize
- usize

### Indexing
The graph has a dual indexing system - string keys for human understandability and numerical indices for efficiency. Each coordinate maps to both a user-defined unique string key and an unsigned integer index assigned by the graph. Integer indices may be reused by the graph after its key was deleted.

Upon creating a new vertex type, the graph creates a vertex vector for each supported primitive numeric type. Equivalently, upon creating a new edge type the graph creates a new adjacency matrix for each supported value type.

The numerical vertex indices, and their associated keys, reference the same coordinates in all vertex vectors and adjacency matrices. All vertex vectors and adjacency matrices thus have compatible sizes.

Each combination of vertex vector and adjacency matrix thus defines a separate graph. All graphs share the same coordinates.

### Linear algebra operations
Graph operators apply to any applicable combination of vertex vector and adjacency matrix.


### ACID
Cairn Knowledge Graph does currently not guarantee [ACID](https://en.wikipedia.org/wiki/ACID) database transaction properties.

### Persistence
The graph resides in-memory and does currently not exist in persistent storage.

## Minimum example
```rust
use cairn_knowledge_graph::error::GraphComputingError;
use cairn_knowledge_graph::graph::edge::DirectedEdgeDefinedByKeys;
use cairn_knowledge_graph::graph::graph::Graph;
use cairn_knowledge_graph::graph::vertex::{Vertex, VertexValue};
use cairn_knowledge_graph::operations::add_edge::AddEdge;
use cairn_knowledge_graph::operations::add_vertex::AddVertex;
use cairn_knowledge_graph::operations::selection::operators::and::AndOperator;
use cairn_knowledge_graph::operations::select_vertex::SelectVertex;
use cairn_knowledge_graph::operations::selection::vertex_selection::VertexSelection;

macro_rules! add_new_edge {
  ($from_vertex:ident, $edge_type:ident, $to_vertex:ident, $graph:ident) => {
     let edge = DirectedEdgeDefinedByKeys::new(
         $from_vertex.clone().into(),
         $edge_type.clone(),
         $to_vertex.clone().into(),
     );
     $graph.add_edge_and_edge_type_using_keys(edge.clone()).unwrap();
  };
}
 
fn main() {
    let initial_vertex_capacity = 10;
    let initial_edge_type_capacity = 10;
    let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

    let zero = Vertex::new(String::from("0"), 0u8.into());
    graph.add_or_replace_vertex(zero.clone()).unwrap();

    let one = Vertex::new(String::from("1"), 1u8.into());
    graph.add_or_replace_vertex(one.clone()).unwrap();

    let one_dot_one = Vertex::new(String::from("1.1"), 1.1f32.into());
    graph.add_or_replace_vertex(one_dot_one.clone()).unwrap();

    let negative_one = Vertex::new(String::from("-1"), (-1i8).into());
    graph.add_or_replace_vertex(negative_one.clone()).unwrap();

    let negative_one_dot_one = Vertex::new(String::from("-1.1"), (-1.1f32).into());
    graph
        .add_or_replace_vertex(negative_one_dot_one.clone())
        .unwrap();

    let integer = Vertex::new(String::from("integer"), String::from("integer").into());
    graph.add_or_replace_vertex(integer.clone()).unwrap();

    let real_number = Vertex::new(
        String::from("real_number"),
        String::from("real_number").into(),
    );
    graph.add_or_replace_vertex(real_number.clone()).unwrap();

    let positive = Vertex::new(String::from("positive"), String::from("positive").into());
    graph.add_or_replace_vertex(positive.clone()).unwrap();

    let negative = Vertex::new(String::from("negative"), String::from("negative").into());
    graph.add_or_replace_vertex(negative.clone()).unwrap();

    let sign = String::from("sign");
    let is_a = String::from("is_a");

    add_new_edge!(negative_one_dot_one, is_a, real_number, graph);
    add_new_edge!(negative_one_dot_one, sign, negative, graph);

    add_new_edge!(negative_one, is_a, real_number, graph);
    add_new_edge!(negative_one, is_a, integer, graph);
    add_new_edge!(negative_one, sign, negative, graph);

    add_new_edge!(zero, is_a, real_number, graph);
    add_new_edge!(zero, is_a, integer, graph);

    add_new_edge!(one, is_a, real_number, graph);
    add_new_edge!(one, is_a, integer, graph);
    add_new_edge!(one, sign, positive, graph);

    add_new_edge!(one_dot_one, is_a, real_number, graph);
    add_new_edge!(one_dot_one, sign, positive, graph);

    let negative_selection = graph
        .select_vertices_connected_to_vertex_by_key(sign, &"negative")
        .unwrap();
    let integer_selection = graph
        .select_vertices_connected_to_vertex_by_key(is_a, &"integer")
        .unwrap();

    let negative_integer_selection = negative_selection.and(&integer_selection).unwrap();
    let negative_integers = negative_integer_selection.vertex_values_ref().unwrap();

    assert_eq!(negative_integers.len(), 1);
    assert_eq!(negative_integers, vec!(&VertexValue::Integer8Bit(-1)));
 }
 ```

## Requirements
Please make sure to meet the requirements for building [graphblas_sparse_linear_algebra](https://crates.io/crates/graphblas_sparse_linear_algebra).

## Contributing
Awesome, contributions are welcome. stacked_linear_algebra_graph and your contribution may be relicensed and integrated into commercial software in the future. Therefore, you will be asked to agree to the [Contributor License Agreement](contributor-license-agreement.md) when you make a pull request.

 ## Licensing
stacked_linear_algebra_graph is licensed under [Creative Commons Attribution Non Commercial 4.0 International](https://creativecommons.org/licenses/by-nc/4.0/legalcode). For other licensing options, please contact Sam Dekker.

## Acknowledgements
Stacked Linear Algebra Graph is inspired by [LAGraph](https://github.com/GraphBLAS/LAGraph) and uses the same underlying GraphBLAS implementation from [Timothy A. Davis](https://github.com/DrTimothyAldenDavis/GraphBLAS).
