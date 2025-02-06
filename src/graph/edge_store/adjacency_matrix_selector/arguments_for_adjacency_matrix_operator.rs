use graphblas_sparse_linear_algebra::operators::options::{
    GetTransposeMatrixArgument, WithTransposeMatrixArgument,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::options::{
    GetUseCachedAdjacencyMatrixTranspose, OptionsForOperatorWithAdjacencyMatrixArgument,
};

use super::{
    adjacency_matrix_ref_unchecked, transposed_adjacency_matrix_ref_unchecked,
    try_adjacency_matrix_ref, try_transposed_adjacency_matrix_ref,
};

pub(crate) struct ArgumentsForAdjacencyMatrixOperator<'a> {
    adjacency_matrix: &'a WeightedAdjacencyMatrix,
    options: OptionsForOperatorWithAdjacencyMatrixArgument,
}

pub(crate) trait GetArgumentsForAdjacencyMatrixOperator {
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixArgument;
}

impl<'a> GetArgumentsForAdjacencyMatrixOperator for ArgumentsForAdjacencyMatrixOperator<'a> {
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.adjacency_matrix
    }
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixArgument {
        &self.options
    }
}

impl<'a> ArgumentsForAdjacencyMatrixOperator<'a> {
    fn new(
        adjacency_matrix: &'a WeightedAdjacencyMatrix,
        options: OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Self {
        Self {
            adjacency_matrix,
            options,
        }
    }
}

pub(crate) trait CreateArgumentsForAdjacencyMatrixOperator<'a> {
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<ArgumentsForAdjacencyMatrixOperator<'a>, GraphComputingError>;

    fn try_create_with_transposed_adjacency_matrix_argument(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<ArgumentsForAdjacencyMatrixOperator<'a>, GraphComputingError>;

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Self;

    fn create_unchecked_with_transposed_adjacency_matrix_argument(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Self;
}

// DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
// The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
// This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
// For example, an alternative to unsafe access would be to clone the operands.
impl<'a> CreateArgumentsForAdjacencyMatrixOperator<'a> for ArgumentsForAdjacencyMatrixOperator<'a> {
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<ArgumentsForAdjacencyMatrixOperator<'a>, GraphComputingError> {
        let mut transpose_argument_by_graphblas = operator_options.transpose_matrix_argument();

        let adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_argument_by_graphblas);

        Ok(ArgumentsForAdjacencyMatrixOperator::new(
            adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn try_create_with_transposed_adjacency_matrix_argument(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<ArgumentsForAdjacencyMatrixOperator<'a>, GraphComputingError> {
        let mut transpose_argument_by_graphblas = operator_options.transpose_matrix_argument();

        let adjacency_matrix = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_argument_by_graphblas);

        Ok(ArgumentsForAdjacencyMatrixOperator::new(
            adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Self {
        let mut transpose_argument_by_graphblas = operator_options.transpose_matrix_argument();

        let adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_argument_by_graphblas);

        ArgumentsForAdjacencyMatrixOperator::new(adjacency_matrix, graphblas_operator_options)
    }

    fn create_unchecked_with_transposed_adjacency_matrix_argument(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Self {
        let mut transpose_argument_by_graphblas = operator_options.transpose_matrix_argument();

        let adjacency_matrix = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_argument_by_graphblas);

        ArgumentsForAdjacencyMatrixOperator::new(adjacency_matrix, graphblas_operator_options)
    }
}
