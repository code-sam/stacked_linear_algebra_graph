use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::in_memory::transpose::transpose_adjacency_matrix;
use crate::operators::in_memory::transpose::transpose_adjacency_matrix_unchecked;
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::transpose::TransposeAdjacencyMatrix;
use crate::operators::operators::transpose::TransposeAdjacencyMatrixUnchecked;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain> TransposeAdjacencyMatrix<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        transpose_adjacency_matrix::<EvaluationDomain>(
            &mut self.edge_store_transaction,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain> TransposeAdjacencyMatrixUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        transpose_adjacency_matrix_unchecked::<EvaluationDomain>(
            &mut self.edge_store_transaction,
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

    use super::*;

    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::graph::graph::Graph;
    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn transpose_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();
        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let result_edge_type_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_index_1 = graph
            .new_vertex(&vertex_type_index, vertex_value_1)
            .unwrap();
        let vertex_index_2 = graph
            .new_vertex(&vertex_type_index, vertex_value_2)
            .unwrap();

        let edge_vertex1_vertex2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_1_index, vertex_index_1, vertex_index_2),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_1_index, vertex_index_2, vertex_index_1),
            2u8,
        );
        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_2_index, vertex_index_1, vertex_index_2),
            1u8,
        );

        graph
            .new_edge_from_edge(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .new_edge_from_edge(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .new_edge_from_edge(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        TransposeAdjacencyMatrix::<u16>::apply(
            &mut graph,
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
                    vertex_index_2,
                    vertex_index_1,
                ),
            )
            .unwrap(),
            Some(1)
        );

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_index_1,
                    vertex_index_2,
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
