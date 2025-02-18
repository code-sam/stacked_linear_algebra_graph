use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
};

use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::{
    operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix,
    ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
};
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::graph_operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToAdjacencyMatrix;
use crate::graph_operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToAdjacencyMatrixUnchecked;
use crate::operator_options::{
    OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
};
use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes,
        graph::GraphblasOperatorApplierCollection,
    },
};

impl<EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for Graph
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
            &mut self.public_edge_store,
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
            &mut self.public_edge_store,
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

impl<EvaluationDomain: ValueType>
    ApplyScalarBinaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain> for Graph
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
            &mut self.public_edge_store,
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
            &mut self.public_edge_store,
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

pub(crate) fn apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument<
    EvaluationDomain,
>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    edge_store.try_edge_type_index_validity(left_argument)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument_and_by_unchecked_index::<
        EvaluationDomain,
    >(
        edge_store,
        left_argument,
        operator,
        right_argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument<
    EvaluationDomain,
>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    left_argument: EvaluationDomain,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    edge_store.try_edge_type_index_validity(right_argument)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument_and_by_unchecked_index::<
        EvaluationDomain,
    >(
        edge_store,
        left_argument,
        operator,
        right_argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_scalar_binary_operator_with_adjacency_matrix_as_left_argument_and_by_unchecked_index<
    EvaluationDomain,
>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    let adjacency_matrix_argument =
        ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::create_unchecked(
            edge_store,
            left_argument,
            options,
        );

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .binary_operator_applier()
                .apply_with_matrix_as_left_argument(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    operator,
                    right_argument,
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
                .binary_operator_applier()
                .apply_with_matrix_as_left_argument(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    operator,
                    right_argument,
                    accumlator,
                    adjacency_matrix_product,
                    adjacency_matrix_mask,
                    adjacency_matrix_argument.options_ref(),
                )?)
        }
    }
}

pub(crate) fn apply_scalar_binary_operator_with_adjacency_matrix_as_right_argument_and_by_unchecked_index<
    EvaluationDomain,
>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    left_argument: EvaluationDomain,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    let adjacency_matrix_argument =
        ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::create_unchecked(
            edge_store,
            right_argument,
            options,
        );

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .binary_operator_applier()
                .apply_with_matrix_as_right_argument(
                    left_argument,
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
                .binary_operator_applier()
                .apply_with_matrix_as_right_argument(
                    left_argument,
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
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
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

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
            &mut graph,
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
                &graph,
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
            &mut graph,
            &edge_type_2_index,
            &Plus::<u8>::new(),
            u8::MAX,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
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
            Some(2)
        );

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_1_index,
            &Plus::<u8>::new(),
            1u8,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetEdgeWeight::<usize>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_2_index,
                    vertex_1_index,
                ),
            )
            .unwrap(),
            Some(3)
        )
    }
}
