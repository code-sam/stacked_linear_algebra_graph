use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{binary_operator::AccumulatorBinaryOperator, mask::MatrixMask},
};

use crate::graph::indexing::{
    GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex,
};
use crate::graph_operators::in_memory::select::{
    select_edges_with_tail_vertex, select_edges_with_tail_vertex_unchecked,
};
use crate::graph_operators::operator_traits::select::{
    SelectEdgesWithTailVertex, SelectEdgesWithTailVertexUnchecked,
};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain> SelectEdgesWithTailVertex<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    SparseMatrix<EvaluationDomain>: MatrixMask,
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        tail_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_edges_with_tail_vertex::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            &mut self.vertex_store_transaction,
            adjacency_matrix,
            tail_vertex,
            accumlator,
            extract_to,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain> SelectEdgesWithTailVertexUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    SparseMatrix<EvaluationDomain>: MatrixMask,
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        tail_vertex: &impl GetVertexIndexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_edges_with_tail_vertex_unchecked::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            &mut self.vertex_store_transaction,
            adjacency_matrix,
            tail_vertex,
            accumlator,
            extract_to,
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

    use crate::graph::graph::Graph;
    use crate::graph::indexing::VertexTypeIndex;
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn select_edges_with_tail_vertex() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_result_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u16>::apply(&mut graph).unwrap();
        let _result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

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

        SelectEdgesWithTailVertex::<isize>::apply(
            &mut graph,
            &edge_type_1_index,
            &vertex_1_index,
            &Plus::<isize>::new(),
            &vertex_result_type_index,
            None::<&VertexTypeIndex>,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_2_index,
            )
            .unwrap(),
            Some(1)
        );

        SelectEdgesWithTailVertex::<isize>::apply(
            &mut graph,
            &edge_type_1_index,
            &vertex_2_index,
            &Assignment::new(),
            &vertex_result_type_index,
            None::<&VertexTypeIndex>,
            &&OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     ReadAdjacencyMatrixElementList::<u8>::with_key(&graph, edge_type_1_key).unwrap()
        // );
        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<isize>::with_key(&graph, vertex_result_type_key).unwrap()
        // );
        assert_eq!(
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_1_index,
            )
            .unwrap(),
            Some(2)
        );
    }
}
