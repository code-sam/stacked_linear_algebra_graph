use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph_operators::in_memory::element_wise_multiplication::apply_semiring_element_wise_adjacency_matrix_multiplication;
use crate::graph_operators::in_memory::element_wise_multiplication::apply_semiring_element_wise_adjacency_matrix_multiplication_unchecked;
use crate::graph_operators::operator_traits::element_wise_multiplication::SemiringElementWiseAdjacencyMatrixMultiplication;
use crate::graph_operators::operator_traits::element_wise_multiplication::SemiringElementWiseAdjacencyMatrixMultiplicationUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain: ValueType>
    SemiringElementWiseAdjacencyMatrixMultiplication<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        apply_semiring_element_wise_adjacency_matrix_multiplication::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain: ValueType>
    SemiringElementWiseAdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        apply_semiring_element_wise_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::Graph;
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetEdgeWeight;

    #[test]
    fn semiring_element_wise_adjacency_matrix_multiplication() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u16>::apply(&mut graph).unwrap();
        let result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        for _i in 0..2 {
            SemiringElementWiseAdjacencyMatrixMultiplication::<u8>::apply(
                &mut graph,
                &edge_type_1_index,
                &PlusTimes::<u8>::new(),
                &edge_type_1_index,
                &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
                &result_edge_type_index,
                None,
                &OptionsForOperatorWithAdjacencyMatrixArguments::new_default(),
            )
            .unwrap();
        }

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_1_index,
                    vertex_2_index,
                ),
            )
            .unwrap(),
            Some(4)
        );

        SemiringElementWiseAdjacencyMatrixMultiplication::<u8>::apply(
            &mut graph,
            &edge_type_1_index,
            &PlusTimes::<u8>::new(),
            &edge_type_2_index,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArguments::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_2_index,
                    vertex_1_index,
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
