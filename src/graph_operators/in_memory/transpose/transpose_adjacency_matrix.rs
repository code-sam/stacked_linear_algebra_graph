use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::transpose::TransposeMatrix;

use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::ArgumentsForAdjacencyMatrixOperator;
use crate::graph::edge_store::CreateArgumentsForAdjacencyMatrixOperator;
use crate::graph::edge_store::GetArgumentsForAdjacencyMatrixOperator;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollection;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph_operators::operator_traits::transpose::TransposeAdjacencyMatrix;
use crate::graph_operators::operator_traits::transpose::TransposeAdjacencyMatrixUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
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
        transpose_adjacency_matrix::<EvaluationDomain>(
            &mut self.public_edge_store,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
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
        transpose_adjacency_matrix_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn transpose_adjacency_matrix<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    edge_store.try_edge_type_index_validity(argument)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    transpose_adjacency_matrix_unchecked::<EvaluationDomain>(
        edge_store,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn transpose_adjacency_matrix_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let adjacency_matrix_argument =
        ArgumentsForAdjacencyMatrixOperator::create_unchecked(edge_store, argument, options);

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
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
            let adjacency_matrix_mask =
                graphblas_operator_applier_collection.entire_matrix_selector();

            Ok(graphblas_operator_applier_collection
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

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;

    use super::*;

    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetEdgeWeight;

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
