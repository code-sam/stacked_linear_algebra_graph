use graphblas_sparse_linear_algebra::operators::options::{
    GetOperatorOptions as GetGraphblasOperatorOptions, MutateOperatorOptions,
};

use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        edge_store::{
            operations::{
                get_adjacency_matrix::GetAdjacencyMatrix,
                get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes,
            },
            weighted_adjacency_matrix::WeightedAdjacencyMatrix,
        },
    },
};

use crate::operators::options::{GetOperatorOptions, OperatorOptions};

use super::{
    adjacency_matrix_ref_unchecked, transposed_adjacency_matrix_ref_unchecked,
    try_adjacency_matrix_ref, try_transposed_adjacency_matrix_ref,
};

pub(crate) struct GraphblasAdjacencyMatrixBinaryOperatorArguments<'a> {
    argument_0: &'a WeightedAdjacencyMatrix,
    argument_1: &'a WeightedAdjacencyMatrix,
    options: OperatorOptions,
}

pub(crate) trait GetGraphblasAdjacencyMatrixBinaryOperatorArguments {
    fn argument_0(&self) -> &WeightedAdjacencyMatrix;
    fn argument_1(&self) -> &WeightedAdjacencyMatrix;
    fn options(&self) -> &OperatorOptions;
}

impl<'a> GetGraphblasAdjacencyMatrixBinaryOperatorArguments
    for GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>
{
    fn argument_0(&self) -> &WeightedAdjacencyMatrix {
        &self.argument_0
    }
    fn argument_1(&self) -> &WeightedAdjacencyMatrix {
        &self.argument_1
    }
    fn options(&self) -> &OperatorOptions {
        &self.options
    }
}

impl<'a> GraphblasAdjacencyMatrixBinaryOperatorArguments<'a> {
    fn new(
        argument_0: &'a WeightedAdjacencyMatrix,
        argument_1: &'a WeightedAdjacencyMatrix,
        options: OperatorOptions,
    ) -> Self {
        Self {
            argument_0,
            argument_1,
            options,
        }
    }
}

pub(crate) trait CreateGraphblasBinaryOperatorArguments<'a> {
    fn create_try(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError>;

    fn create_try_with_transposed_arg0(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError>;

    fn create_try_with_transposed_arg1(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError>;

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self;

    fn create_unchecked_with_transposed_arg0(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self;

    fn create_unchecked_with_transposed_arg1(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self;
}

// DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
// The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
// This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
// For example, an alternative to unsafe access would be to clone the operands.
impl<'a> CreateGraphblasBinaryOperatorArguments<'a>
    for GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>
{
    fn create_try(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError> {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        )?;

        let argument_1 = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        Ok(GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        ))
    }

    fn create_try_with_transposed_arg0(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError> {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        )?;

        let argument_1 = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        Ok(GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        ))
    }

    fn create_try_with_transposed_arg1(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Result<GraphblasAdjacencyMatrixBinaryOperatorArguments<'a>, GraphComputingError> {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = try_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        )?;

        let argument_1 = try_transposed_adjacency_matrix_ref(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        )?;

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        Ok(GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        ))
    }

    fn create_unchecked(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        );

        let argument_1 = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_arg0(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        );

        let argument_1 = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        )
    }

    fn create_unchecked_with_transposed_arg1(
        edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + 'a),
        edge_type_index_arg0: &EdgeTypeIndex,
        edge_type_index_arg1: &EdgeTypeIndex,
        operator_options: &'a OperatorOptions,
    ) -> Self {
        let mut transpose_argument_0_by_graphblas = operator_options.transpose_input0();
        let mut transpose_argument_1_by_graphblas = operator_options.transpose_input1();

        let argument_0 = adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg0,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_0_by_graphblas,
        );

        let argument_1 = transposed_adjacency_matrix_ref_unchecked(
            unsafe { &mut *edge_store },
            edge_type_index_arg1,
            operator_options.use_cached_adjacency_matrix_transpose(),
            &mut transpose_argument_1_by_graphblas,
        );

        let graphblas_operator_options = operator_options.with_transpose_input(
            transpose_argument_0_by_graphblas,
            transpose_argument_1_by_graphblas,
        );

        GraphblasAdjacencyMatrixBinaryOperatorArguments::new(
            argument_0,
            argument_1,
            graphblas_operator_options,
        )
    }
}
