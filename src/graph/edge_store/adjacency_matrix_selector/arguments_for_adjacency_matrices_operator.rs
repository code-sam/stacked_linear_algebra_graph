use graphblas_sparse_linear_algebra::operators::options::{
    GetTransposeArguments, WithTransposeArguments,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operator_options::{
    GetUseCachedAdjacencyMatrixTranspose, OptionsForOperatorWithAdjacencyMatrixArguments,
};

use super::{
    adjacency_matrix_ref_unchecked, transposed_adjacency_matrix_ref_unchecked,
    try_adjacency_matrix_ref, try_transposed_adjacency_matrix_ref,
};

pub(crate) struct ArgumentsForAdjacencyMatricesOperator<'a> {
    left_adjacency_matrix: &'a WeightedAdjacencyMatrix,
    right_adjacency_matrix: &'a WeightedAdjacencyMatrix,
    options: OptionsForOperatorWithAdjacencyMatrixArguments,
}

pub(crate) trait GetArgumentsForAdjacencyMatricesOperator {
    fn left_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn right_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixArguments;
}

impl<'a> GetArgumentsForAdjacencyMatricesOperator for ArgumentsForAdjacencyMatricesOperator<'a> {
    fn left_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.left_adjacency_matrix
    }
    fn right_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.right_adjacency_matrix
    }
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixArguments {
        &self.options
    }
}

impl<'a> ArgumentsForAdjacencyMatricesOperator<'a> {
    fn new(
        left_adjacency_matrix: &'a WeightedAdjacencyMatrix,
        right_adjacency_matrix: &'a WeightedAdjacencyMatrix,
        options: OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self {
        Self {
            left_adjacency_matrix,
            right_adjacency_matrix,
            options,
        }
    }
}

pub(crate) trait CreateArgumentsForAdjacencyMatricesOperator<'a> {
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError>;

    fn try_create_with_transposed_left_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError>;

    fn try_create_with_transposed_right_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError>;

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self;

    fn create_unchecked_with_transposed_left_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self;

    fn create_unchecked_with_transposed_right_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self;
}

// DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
// The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
// This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
// For example, an alternative to unsafe access would be to clone the operands.
impl<'a> CreateArgumentsForAdjacencyMatricesOperator<'a>
    for ArgumentsForAdjacencyMatricesOperator<'a>
{
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError> {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        )?;

        let second_adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        Ok(ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn try_create_with_transposed_left_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError> {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        )?;

        let second_adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        Ok(ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn try_create_with_transposed_right_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<ArgumentsForAdjacencyMatricesOperator<'a>, GraphComputingError> {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        )?;

        let second_adjacency_matrix = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        Ok(ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        );

        let second_adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_left_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        );

        let second_adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_right_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        left_edge_type_index: &impl GetEdgeTypeIndex,
        right_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Self {
        let mut transpose_first_argument_by_graphblas = operator_options.transpose_first_argument();
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_argument();

        let first_adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            left_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        );

        let second_adjacency_matrix = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            right_edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_matrix_arguments(
            transpose_first_argument_by_graphblas,
            transpose_second_argument_by_graphblas,
        );

        ArgumentsForAdjacencyMatricesOperator::new(
            first_adjacency_matrix,
            second_adjacency_matrix,
            graphblas_operator_options,
        )
    }
}
