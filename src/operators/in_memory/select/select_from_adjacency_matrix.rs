use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::select::MatrixSelector;
use graphblas_sparse_linear_algebra::operators::select::SelectFromMatrix;

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
use crate::operators::operators::select::SelectFromAdjacencyMatrix;
use crate::operators::operators::select::SelectFromAdjacencyMatrixUnchecked;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain: ValueType> SelectFromAdjacencyMatrix<EvaluationDomain> for Graph
where
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn apply(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: EvaluationDomain,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_from_adjacency_matrix_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            selector,
            selector_argument,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<EvaluationDomain: ValueType> SelectFromAdjacencyMatrixUnchecked<EvaluationDomain> for Graph
where
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn apply(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: EvaluationDomain,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_from_adjacency_matrix_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            selector,
            selector_argument,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn select_from_adjacency_matrix<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    selector: &impl IndexUnaryOperator<EvaluationDomain>,
    selector_argument: EvaluationDomain,
    argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    edge_store.try_edge_type_index_validity(argument)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    select_from_adjacency_matrix_unchecked::<EvaluationDomain>(
        edge_store,
        selector,
        selector_argument,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn select_from_adjacency_matrix_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    selector: &impl IndexUnaryOperator<EvaluationDomain>,
    selector_argument: EvaluationDomain,
    argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    let operator_argument =
        ArgumentsForAdjacencyMatrixOperator::create_unchecked(edge_store, argument, options);

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .matrix_selector()
                .apply(
                    selector,
                    selector_argument,
                    operator_argument.adjacency_matrix_ref(),
                    accumlator,
                    adjacency_matrix_product,
                    adjacency_matrix_mask,
                    operator_argument.options_ref(),
                )?)
        }
        None => {
            let adjacency_matrix_mask =
                graphblas_operator_applier_collection.entire_matrix_selector();

            Ok(graphblas_operator_applier_collection
                .matrix_selector()
                .apply(
                    selector,
                    selector_argument,
                    operator_argument.adjacency_matrix_ref(),
                    accumlator,
                    adjacency_matrix_product,
                    adjacency_matrix_mask,
                    operator_argument.options_ref(),
                )?)
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
        GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes,
    };
    use crate::graph::edge_store::operations::operations::edge_type::map::MapAdjacencyMatricesWithCachedAttributes;
    use crate::graph::graph::GetEdgeStore;
    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn select_from_adjacency_matrix() {
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

        SelectFromAdjacencyMatrix::apply(
            &mut graph,
            &IsValueGreaterThan::<u8>::new(),
            1,
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
            None
        );

        graph.edge_store_ref().map_all_adjacency_matrices(
            |adjacency_matrix: &WeightedAdjacencyMatrixWithCachedAttributes| {
                println!("{}", adjacency_matrix.weighted_adjacency_matrix_ref());
                Ok(())
            },
        );
        // for adjacency_matrix in graph.edge_store_ref().ma.adjacency_matrices_ref().into_iter() {
        //     println!("{}", adjacency_matrix.weighted_adjacency_matrix_ref());
        // }

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
