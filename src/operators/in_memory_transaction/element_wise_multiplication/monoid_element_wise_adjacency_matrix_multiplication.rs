use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::in_memory::element_wise_multiplication::apply_monoid_element_wise_adjacency_matrix_multiplication;
use crate::operators::in_memory::element_wise_multiplication::apply_monoid_element_wise_adjacency_matrix_multiplication_unchecked;
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::element_wise_multiplication::MonoidElementWiseAdjacencyMatrixMultiplication;
use crate::operators::operators::element_wise_multiplication::MonoidElementWiseAdjacencyMatrixMultiplicationUnchecked;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain: ValueType>
    MonoidElementWiseAdjacencyMatrixMultiplication<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn by_index(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        apply_monoid_element_wise_adjacency_matrix_multiplication::<EvaluationDomain>(
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
    MonoidElementWiseAdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        apply_monoid_element_wise_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
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
    use graphblas_sparse_linear_algebra::operators::monoid::Plus;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::Graph;
    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn monoid_element_wise_adjacency_matrix_multiplication() {
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
            MonoidElementWiseAdjacencyMatrixMultiplication::<u8>::by_index(
                &mut graph,
                &edge_type_1_index,
                &Plus::<u8>::new(),
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

        MonoidElementWiseAdjacencyMatrixMultiplication::<u8>::by_index(
            &mut graph,
            &edge_type_1_index,
            &Plus::<u8>::new(),
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
            None
        );
    }
}
