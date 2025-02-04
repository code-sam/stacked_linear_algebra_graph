use graphblas_sparse_linear_algebra::operators::apply::{
    ApplyIndexUnaryOperator, IndexUnaryOperatorApplier,
};
use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;

use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operators::in_memory::apply_operator::{
    apply_index_unary_operator_to_adjacency_matrix,
    apply_index_unary_operator_to_adjacency_matrix_unchecked,
};
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;

use crate::error::GraphComputingError;
use crate::graph::value_type::ValueType;
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrix;
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked;

impl<'g, EvaluationDomain: ValueType> ApplyIndexUnaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        apply_index_unary_operator_to_adjacency_matrix(
            &mut self.edge_store_transaction,
            adjacency_matrix,
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

impl<'g, EvaluationDomain: ValueType>
    ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        apply_index_unary_operator_to_adjacency_matrix_unchecked(
            &mut self.edge_store_transaction,
            adjacency_matrix,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            self.graphblas_operator_applier_collection_ref(),
        )
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::Graph;
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

        ApplyIndexUnaryOperatorToAdjacencyMatrix::<f32>::apply(
            &mut graph,
            &edge_type_1_index,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     WeightedAdjacencyMatrixSparseMatrixTrait::<u16>::sparse_matrix_ref(
        //         graph
        //             .edge_store_ref()
        //             .adjacency_matrix_ref_for_key(result_type_key)
        //             .unwrap()
        //     )
        //     .get_element_list()
        //     .unwrap()
        // );

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
            Some(1)
        );
    }
}
