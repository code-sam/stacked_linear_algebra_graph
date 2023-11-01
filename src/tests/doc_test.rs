use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueEqualTo;

use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

use crate::graph::edge::{
    DirectedEdgeCoordinateDefinedByIndices, WeightedDirectedEdgeDefinedByIndices,
};

use crate::graph::graph::Graph;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
use crate::operators::apply_operator::ApplyIndexUnaryOperatorToVertexVector;
use crate::operators::element_wise_multiplication::BinaryOperatorElementWiseVertexVectorMultiplication;
use crate::operators::multiplication::VertexVectorAdjacencyMatrixMultiplication;
use crate::operators::read::ReadVertexValue;

fn main() {
    let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

    let numbers_vertex_type_key = "numbers";
    let odd_number_sequence_edge_type_key = "odd_number_sequence";

    let _vertex_type_1_index = graph.add_new_vertex_type(numbers_vertex_type_key).unwrap();

    // Add vertices
    let mut vertex_indices = Vec::new();
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
    let selected_vertices_key = "selected_vertices";
    let selected_vertices_index = graph.add_new_vertex_type(selected_vertices_key).unwrap();

    ApplyIndexUnaryOperatorToVertexVector::<u8, u8, u8>::with_key(
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
        VertexVectorAdjacencyMatrixMultiplication::<u8, bool, u8, u8>::by_index(
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

    BinaryOperatorElementWiseVertexVectorMultiplication::<u8, u8, u8, u8>::by_key(
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
