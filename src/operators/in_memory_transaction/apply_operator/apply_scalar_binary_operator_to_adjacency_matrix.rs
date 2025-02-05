use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::graph::value_type::ValueType;
use crate::operators::operators::apply_operator::ApplyScalarBinaryOperatorToAdjacencyMatrix;
use crate::operators::operators::apply_operator::ApplyScalarBinaryOperatorToAdjacencyMatrixUnchecked;
use crate::operators::options::{
    OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
};
use crate::{
    error::GraphComputingError,
    operators::{
        in_memory::apply_operator::{
            apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument,
            apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument_and_by_unchecked_index,
            apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument,
            apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument_and_by_unchecked_index,
        },
        in_memory_transaction::transaction::InMemoryGraphTransaction,
    },
};

impl<'g, EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument::<EvaluationDomain>(
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

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument::<EvaluationDomain>(
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
    ApplyScalarBinaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_adjacency_matrix_as_left_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument_and_by_unchecked_index::<
            EvaluationDomain,
        >(
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

    fn with_adjacency_matrix_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument_and_by_unchecked_index::<
            EvaluationDomain,
        >(
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
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::Graph;
    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetEdgeWeight;
    use crate::operators::transaction::UseTransaction;

    #[test]
    fn add_scalar_to_adjacency_matrix() {
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

        {
            let mut transaction = InMemoryGraphTransaction::new(&mut graph).unwrap();

            ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
                &mut transaction,
                &edge_type_1_index,
                &Plus::<u8>::new(),
                1,
                &Assignment::new(),
                &result_edge_type_index,
                None,
                &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
            )
            .unwrap();

            assert_eq!(
                GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                    &transaction,
                    &DirectedEdgeCoordinate::new(
                        result_edge_type_index,
                        vertex_1_index,
                        vertex_2_index,
                    ),
                )
                .unwrap(),
                Some(2)
            );

            ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
                &mut transaction,
                &edge_type_1_index,
                &Plus::<u8>::new(),
                1,
                &Assignment::new(),
                &edge_type_1_index,
                None,
                &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
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
            None
        );

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_1_index,
                    vertex_1_index,
                    vertex_2_index,
                ),
            )
            .unwrap(),
            Some(1)
        );

    }
}
