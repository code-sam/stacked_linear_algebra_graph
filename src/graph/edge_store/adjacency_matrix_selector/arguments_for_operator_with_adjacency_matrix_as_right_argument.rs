use graphblas_sparse_linear_algebra::operators::options::{
    GetTransposeSecondMatrixArgument, WithTransposeMatrixArgument,
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
        indexing::{EdgeTypeIndex, GetEdgeTypeIndex},
    },
    operators::options::{
        GetUseCachedAdjacencyMatrixTranspose, OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    },
};

use super::{
    adjacency_matrix_ref_unchecked, transposed_adjacency_matrix_ref_unchecked,
    try_adjacency_matrix_ref, try_transposed_adjacency_matrix_ref,
};

pub(crate) struct ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a> {
    adjacency_matrix: &'a WeightedAdjacencyMatrix,
    options: OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
}

pub(crate) trait GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument {
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixAsRightArgument;
}

impl<'a> GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument
    for ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>
{
    fn adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.adjacency_matrix
    }
    fn options_ref(&self) -> &OptionsForOperatorWithAdjacencyMatrixAsRightArgument {
        &self.options
    }
}

impl<'a> ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a> {
    fn new(
        adjacency_matrix: &'a WeightedAdjacencyMatrix,
        options: OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Self {
        Self {
            adjacency_matrix,
            options,
        }
    }
}

pub(crate) trait CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument<'a> {
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>, GraphComputingError>;

    fn try_create_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>, GraphComputingError>;

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        second_argument_edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Self;

    fn create_unchecked_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Self;
}

// DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
// The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
// This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
// For example, an alternative to unsafe access would be to clone the operands.
impl<'a> CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument<'a>
    for ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>
{
    fn try_create(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>, GraphComputingError>
    {
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_matrix_argument();

        let adjacency_matrix = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_second_argument_by_graphblas);

        Ok(
            ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::new(
                adjacency_matrix,
                graphblas_operator_options,
            ),
        )
    }

    fn try_create_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument<'a>, GraphComputingError>
    {
        let mut transpose_secod_argument_by_graphblas =
            operator_options.transpose_second_matrix_argument();

        let adjacency_matrix = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_secod_argument_by_graphblas,
        )?;

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_secod_argument_by_graphblas);

        Ok(
            ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::new(
                adjacency_matrix,
                graphblas_operator_options,
            ),
        )
    }

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Self {
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_matrix_argument();

        let adjacency_matrix = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_second_argument_by_graphblas);

        ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_adjacency_matrix(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index: &impl GetEdgeTypeIndex,
        operator_options: &'a OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Self {
        let mut transpose_second_argument_by_graphblas =
            operator_options.transpose_second_matrix_argument();

        let adjacency_matrix = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_second_argument_by_graphblas,
        );

        let graphblas_operator_options =
            operator_options.with_transpose_matrix_argument(transpose_second_argument_by_graphblas);

        ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::new(
            adjacency_matrix,
            graphblas_operator_options,
        )
    }
}
