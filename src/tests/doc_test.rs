use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueEqualTo;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
use crate::graph::graph::Graph;
use crate::graph::indexing::{VertexIndex, VertexTypeIndex};
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToVertexVector;
use crate::operators::operators::element_wise_multiplication::BinaryOperatorElementWiseVertexVectorMultiplication;
use crate::operators::operators::multiplication::VertexVectorAdjacencyMatrixMultiplication;
use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
use crate::operators::operators::read::GetVertexValue;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixAsRightArgument;

fn main() {
    let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

    let numbers_vertex_type_index: VertexTypeIndex =
        NewVertexType::<i32>::apply(&mut graph).unwrap();
    let odd_number_sequence_edge_type_index = NewEdgeType::<i32>::apply(&mut graph).unwrap();

    // Add vertices
    let mut vertex_indices: Vec<VertexIndex> = Vec::new();
    for n in 0..12 {
        vertex_indices.push(
            graph
                .new_vertex(&numbers_vertex_type_index, n as u8)
                .unwrap(),
        );
    }

    // Define a sequence of subsequent odd numbers
    for i in [1, 3, 5, 7, 9] {
        let edge = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                odd_number_sequence_edge_type_index,
                vertex_indices[i],
                vertex_indices[i + 2],
            ),
            true,
        );

        graph.new_edge_from_edge(edge).unwrap();
    }

    // Find the fourth number in the sequence, starting at 1
    let selected_vertices_index: VertexTypeIndex = NewVertexType::<i32>::apply(&mut graph).unwrap();

    ApplyIndexUnaryOperatorToVertexVector::<u8>::apply(
        &mut graph,
        &numbers_vertex_type_index,
        &IsValueEqualTo::<u8>::new(),
        &1,
        &Assignment::new(),
        &selected_vertices_index,
        None,
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
            None,
            &&OptionsForOperatorWithAdjacencyMatrixAsRightArgument::new_default(),
        )
        .unwrap();
    }

    BinaryOperatorElementWiseVertexVectorMultiplication::<u8>::apply(
        &mut graph,
        &selected_vertices_index,
        &Plus::<u8>::new(),
        &numbers_vertex_type_index,
        &Assignment::new(),
        &selected_vertices_index,
        None,
        &OperatorOptions::new_default(),
    )
    .unwrap();

    assert_eq!(
        GetVertexValue::<u8>::vertex_value(&graph, &numbers_vertex_type_index, &vertex_indices[7])
            .unwrap(),
        Some(7u8)
    )
}
