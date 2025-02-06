use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory::apply_operator::{
    apply_unary_operator_to_adjacency_matrix, apply_unary_operator_to_adjacency_matrix_unchecked,
};
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToAdjacencyMatrix;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToAdjacencyMatrixUnchecked;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;

impl<'g, EvaluationDomain: ValueType> ApplyUnaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_adjacency_matrix::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain: ValueType> ApplyUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_adjacency_matrix_unchecked::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            operator,
            argument,
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
    use graphblas_sparse_linear_algebra::operators::unary_operator::ColumnIndex;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::Graph;
    use crate::graph::indexing::GetIndex;
    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

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

        ApplyUnaryOperatorToAdjacencyMatrix::<i32>::apply(
            &mut graph,
            &ColumnIndex::<i32>::new(),
            &edge_type_1_index,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

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
            Some(vertex_2_index.index() as u16)
        );
    }
}
