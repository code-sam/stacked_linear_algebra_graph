use graphblas_sparse_linear_algebra::operators::apply::{
    ApplyIndexUnaryOperator, IndexUnaryOperatorApplier,
};
use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;

use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatrixOperator, CreateArgumentsForAdjacencyMatrixOperator,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::graph::{
    GetGraphblasOperatorAppliers, GraphblasOperatorApplierCollection,
};
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::operators::operator_traits::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrix;
use crate::operators::operator_traits::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked;

impl<EvaluationDomain: ValueType> ApplyIndexUnaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for Graph
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
            &mut self.public_edge_store,
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

impl<EvaluationDomain: ValueType>
    ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain> for Graph
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
            &mut self.public_edge_store,
            adjacency_matrix,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            &mut self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn apply_index_unary_operator_to_adjacency_matrix<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    adjacency_matrix: &impl GetEdgeTypeIndex,
    operator: &impl IndexUnaryOperator<EvaluationDomain>,
    argument: &EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    edge_store.try_edge_type_index_validity(adjacency_matrix)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    apply_index_unary_operator_to_adjacency_matrix_unchecked::<EvaluationDomain>(
        edge_store,
        adjacency_matrix,
        operator,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_index_unary_operator_to_adjacency_matrix_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    adjacency_matrix: &impl GetEdgeTypeIndex,
    operator: &impl IndexUnaryOperator<EvaluationDomain>,
    argument: &EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    let operator_argument = ArgumentsForAdjacencyMatrixOperator::create_unchecked(
        edge_store,
        adjacency_matrix,
        options,
    );

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .index_unary_operator_applier()
                .apply_to_matrix(
                    operator_argument.adjacency_matrix_ref(),
                    operator,
                    argument,
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
                .index_unary_operator_applier()
                .apply_to_matrix(
                    operator_argument.adjacency_matrix_ref(),
                    operator,
                    argument,
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
    use crate::operators::operator_traits::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operator_traits::read::GetEdgeWeight;

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
