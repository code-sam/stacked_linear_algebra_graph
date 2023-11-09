![test](https://github.com/code-sam/stacked_linear_algebra_graph/actions/workflows/test_main_branch.yml/badge.svg?branch=main)
# Stacked Linear Algebra Graph
An embedded and in-memory graph using sparse linear algebra.

## Capabilities

### Architecture
The Stacked Linear Algebra Graph implements a [directed graph](https://en.wikipedia.org/wiki/Directed_graph) with a weight on each vertex and edge.
The graph models vertices and adjacency matrices as GraphBLAS sparse vectors and matrices respectively. 
The graph operates on its vertex vectors and adjacency matrices using GraphBLAS operators.

### Data types
The graph stores the following Rust primitive numeric types in its vertices and edges:
bool; i8; i16; i32; i64; u8; u16; u32; u64; f32; f64; isize; usize

### Indexing
The graph has a dual indexing system - string keys for human understandability and numerical indices for efficiency. Each coordinate maps to both a user-defined unique string key and an unsigned integer index assigned by the graph. Integer indices may be reused by the graph after its key was deleted.

The numerical vertex indices, and their associated keys, reference the same coordinates in all vertex vectors and adjacency matrices. All vertex vectors and adjacency matrices thus have compatible sizes.

Each combination of vertex vector and adjacency matrix thus defines a separate graph. All graphs share the same coordinates.

### Type casting
Each vertex vector and adjacency matrix has a single data datatype. The data type is set upon adding the vertex vector or adjacency matrix to the graph.

Operations involving different value types will use type casting according to ANSI C, with the following exceptions:
- +-Inf or integer values outside their maximum range are clipped
- NaN casts to zero

### Linear algebra operations
Graph operators apply to any applicable combination of vertex vector and adjacency matrix.

### Transactions
Cairn Knowledge Graph does not guarantee [ACID](https://en.wikipedia.org/wiki/ACID) database transaction properties.

### Persistence
The graph resides in-memory and does not exist in persistent storage.

## Minimum example
```rust
    use graphblas_sparse_linear_algebra::operators::binary_operator::{
        Assignment, Plus,
    };
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueEqualTo;
    
    use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use stacked_linear_algebra_graph::graph::edge::{
        DirectedEdgeCoordinateDefinedByIndices,
        WeightedDirectedEdgeDefinedByIndices,
    };
    
    use stacked_linear_algebra_graph::graph::graph::Graph;
    use stacked_linear_algebra_graph::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
    use stacked_linear_algebra_graph::operators::add::{
        AddEdge, AddEdgeType, AddVertexType, AddVertex
    };
    use stacked_linear_algebra_graph::operators::apply_operator::ApplyIndexUnaryOperatorToVertexVector;
    use stacked_linear_algebra_graph::operators::element_wise_multiplication
        ::BinaryOperatorElementWiseVertexVectorMultiplication;
    use stacked_linear_algebra_graph::operators::multiplication
        ::VertexVectorAdjacencyMatrixMultiplication;
    use stacked_linear_algebra_graph::operators::read::ReadVertexValue;

    fn main() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let numbers_vertex_type_key: &str = "numbers";
        let odd_number_sequence_edge_type_key: &str = "odd_number_sequence";

        let _vertex_type_1_index: usize =
            AddVertexType::<i32>::add_new_vertex_type(&mut graph, numbers_vertex_type_key).unwrap();

        // Add vertices
        let mut vertex_indices: Vec<usize> = Vec::new();
        for n in 0..12 {
            vertex_indices.push(
                graph
                    .add_new_key_defined_vertex(VertexDefinedByKey::new(
                        numbers_vertex_type_key,
                        format!("vertex_{}", n).as_str(),
                        &(n as u8),
                    ))
                    .unwrap(),
            );
        }

        let odd_number_sequence_edge_type_index = <Graph as AddEdgeType<i32>>::add_new_edge_type(
            &mut graph,
            odd_number_sequence_edge_type_key,
        )
        .unwrap();

        // Define a sequence of subsequent odd numbers
        for i in [1, 3, 5, 7, 9] {
            let edge = WeightedDirectedEdgeDefinedByIndices::new(
                DirectedEdgeCoordinateDefinedByIndices::new(
                    odd_number_sequence_edge_type_index,
                    vertex_indices[i],
                    vertex_indices[i + 2],
                ),
                true,
            );

            graph.add_new_edge_using_indices(edge).unwrap();
        }

        // Find the fourth number in the sequence, starting at 1
        let selected_vertices_key: &str = "selected_vertices";
        let selected_vertices_index: usize =
            AddVertexType::<i32>::add_new_vertex_type(&mut graph, selected_vertices_key).unwrap();

        ApplyIndexUnaryOperatorToVertexVector::<u8>::with_key(
            &mut graph,
            numbers_vertex_type_key,
            &IsValueEqualTo::<u8>::new(),
            &1,
            &Assignment::new(),
            selected_vertices_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        for _i in 0..2 {
            VertexVectorAdjacencyMatrixMultiplication::<u8>::by_index(
                &mut graph,
                &selected_vertices_index,
                &PlusTimes::<u8>::new(),
                &odd_number_sequence_edge_type_index,
                &Assignment::new(),
                &selected_vertices_index,
                &OperatorOptions::new_default(),
            )
            .unwrap();
        }

        BinaryOperatorElementWiseVertexVectorMultiplication::<u8>::by_key(
            &mut graph,
            selected_vertices_key,
            &Plus::<u8>::new(),
            numbers_vertex_type_key,
            &Assignment::new(),
            selected_vertices_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            ReadVertexValue::<u8>::vertex_value_by_key(&graph, selected_vertices_key, "vertex_7")
                .unwrap(),
            Some(7)
        )
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
