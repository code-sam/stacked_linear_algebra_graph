use graphblas_sparse_linear_algebra::operators::{
    apply::ApplyUnaryOperator as ApplyGraphBlasUnaryOperator,
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::traits::traits::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatrixOperator, CreateArgumentsForAdjacencyMatrixOperator,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::graph::{
    GetGraphblasOperatorAppliers, GraphblasOperatorApplierCollection,
};
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::graph_operators::operator_traits::apply_operator::ApplyUnaryOperatorToAdjacencyMatrix;
use crate::graph_operators::operator_traits::apply_operator::ApplyUnaryOperatorToAdjacencyMatrixUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::error::GraphComputingError;

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToAdjacencyMatrix<EvaluationDomain> for Graph {
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
            &mut self.public_edge_store,
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

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
    for Graph
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
            &mut self.public_edge_store,
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

pub(crate) fn apply_unary_operator_to_adjacency_matrix<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    operator: &impl UnaryOperator<EvaluationDomain>,
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

    apply_unary_operator_to_adjacency_matrix_unchecked::<EvaluationDomain>(
        edge_store,
        operator,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_unary_operator_to_adjacency_matrix_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    operator: &impl UnaryOperator<EvaluationDomain>,
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
                .unary_operator_applier()
                .apply_to_matrix(
                    operator,
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
                .unary_operator_applier()
                .apply_to_matrix(
                    operator,
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
    use graphblas_sparse_linear_algebra::operators::unary_operator::ColumnIndex;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::indexing::GetIndex;
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetEdgeWeight;

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
