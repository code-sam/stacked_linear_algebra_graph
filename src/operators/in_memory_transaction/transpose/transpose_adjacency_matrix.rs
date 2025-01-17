use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::transpose::TransposeMatrix;

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::ArgumentsForAdjacencyMatrixOperator;
use crate::graph::edge_store::CreateArgumentsForAdjacencyMatrixOperator;
use crate::graph::edge_store::GetArgumentsForAdjacencyMatrixOperator;
use crate::graph::graph::GetEdgeStore;
use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::operators::indexing::CheckIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain> TransposeAdjacencyMatrix<EvaluationDomain> for Graph
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
        self.try_edge_type_index_validity(argument)?;
        self.try_edge_type_index_validity(product)?;
        self.try_optional_edge_type_index_validity(mask)?;

        TransposeAdjacencyMatrixUnchecked::apply(self, argument, accumlator, product, mask, options)
    }
}

impl<EvaluationDomain> TransposeAdjacencyMatrixUnchecked<EvaluationDomain> for Graph
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
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            ArgumentsForAdjacencyMatrixOperator::create_unchecked(edge_store, argument, options);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product);

        match mask {
            Some(mask) => {
                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_transposer()
                    .apply(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_transposer()
                    .apply(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;

    use super::*;

    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn transpose_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let result_edge_type_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_index_1 = graph
            .add_vertex(&vertex_type_index, vertex_value_1)
            .unwrap();
        let vertex_index_2 = graph
            .add_vertex(&vertex_type_index, vertex_value_2)
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
            .add_edge_from_edge(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_edge_from_edge(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_edge_from_edge(edge_vertex1_vertex2_type_2.clone())
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
