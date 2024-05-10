use graphblas_sparse_linear_algebra::operators::options::{
    GetTransposeFirstMatrixArgument, WithTransposeMatrixArgument,
};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::{
                get_adjacency_matrix::GetAdjacencyMatrix,
                get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes,
            },
            weighted_adjacency_matrix::WeightedAdjacencyMatrix,
        },
        indexing::GetEdgeTypeIndex,
    },
    operators::options::{
        GetUseCachedAdjacencyMatrixTranspose, OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    },
};

use super::{
    adjacency_matrix_ref_unchecked, transposed_adjacency_matrix_ref_unchecked,
    try_adjacency_matrix_ref, try_transposed_adjacency_matrix_ref,
};

pub(crate) struct ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a> {
    adjacency_matrix: &'a WeightedAdjacencyMatrix,
    options: OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
}

pub(crate) trait GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument;
}

impl<'a> GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument
    for ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>
{
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.adjacency_matrix
    }
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
        &self.options
    }
}

impl<'a> ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a> {
    fn new(
        adjacency_matrix: &'a WeightedAdjacencyMatrix,
        options: OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Self {
        Self {
            adjacency_matrix,
            options,
        }
    }
}

pub(crate) trait CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a> {
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>, GraphComputingError>;

    fn try_create_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>, GraphComputingError>;

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Self;

    fn create_unchecked_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Self;
}

// DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
// The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
// This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
// For example, an alternative to unsafe access would be to clone the operands.
impl<'a> CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>
    for ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>
{
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>, GraphComputingError>
    {
        let mut transpose_first_argument_by_graphblas =
            operator_options.transpose_first_matrix_argument();

        let adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_first_argument_by_graphblas);

        Ok(ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn try_create_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument<'a>, GraphComputingError>
    {
        let mut transpose_first_argument_by_graphblas =
            operator_options.transpose_first_matrix_argument();

        let adjacency_matrix = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_first_argument_by_graphblas);

        Ok(ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        ))
    }

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Self {
        let mut transpose_first_argument_by_graphblas =
            operator_options.transpose_first_matrix_argument();

        let adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_first_argument_by_graphblas);

        ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Self {
        let mut transpose_first_argument_by_graphblas =
            operator_options.transpose_first_matrix_argument();

        let adjacency_matrix = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_first_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_first_argument_by_graphblas);

        ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        )
    }
}
