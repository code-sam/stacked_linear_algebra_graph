use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::sparse_matrix_size;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetMatrixDimensions;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::mask::MatrixMask;
use graphblas_sparse_linear_algebra::operators::options::OptionsForOperatorWithMatrixArgument;
use graphblas_sparse_linear_algebra::operators::transpose::TransposeMatrix;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::GetGraphblasSparseMatrix, context::GetContext,
    operators::transpose::MatrixTranspose,
};
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    CreateWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
};
use crate::graph::value_type::implement_macro_for_all_native_value_types_with_capitalized_value_type;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OptionsForOperatorWithMatrixArgument> =
    Lazy::new(|| OptionsForOperatorWithMatrixArgument::new_default());

static MATRIX_TRANSPOSE_OPERATOR: Lazy<MatrixTranspose> = Lazy::new(|| MatrixTranspose::new());

macro_rules! create_lazy_static_assignment_operator {
    ($VALUE_TYPE:ident, $value_type:ty) => {
        paste::paste! {
            static [<ASSIGNMENT_OPERATOR_ $VALUE_TYPE>]: Lazy<Assignment<$value_type>> = Lazy::new(|| Assignment::<$value_type>::new());
        }
    };
}
implement_macro_for_all_native_value_types_with_capitalized_value_type!(
    create_lazy_static_assignment_operator
);

macro_rules! create_transpose_adjacency_matrix_function {
    ($VALUE_TYPE:ident, $value_type:ty) => {
        paste::paste! {
            pub(crate) fn [<transpose_adjacency_matrix_ $value_type>](
                adjacency_matrix: &(impl GetGraphblasSparseMatrix + GetContext),
                mask: &(impl MatrixMask + GetContext)
            ) -> Result<WeightedAdjacencyMatrix, GraphComputingError> {
                let sparse_matrix_size = sparse_matrix_size(adjacency_matrix)?; // TODO: would it be more efficient to use a cached size here?
                let mut transposed_adjacency_matrix =
                    <WeightedAdjacencyMatrix as CreateWeightedAdjacencyMatrix<$value_type>>::new(
                        adjacency_matrix.context(),
                        sparse_matrix_size.column_width(),
                    )?;

                MatrixTranspose::new().apply(
                    adjacency_matrix,
                    &Assignment::<$value_type>::new(),
                    &mut transposed_adjacency_matrix,
                    // &SelectEntireMatrix::new(adjacency_matrix.context_ref()), // TODO: consider caching the selector into the edge store
                    mask,
                    &*DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;

                // TODO: performance benchmarking to select fastest variant
                // MATRIX_TRANSPOSE_OPERATOR.apply(
                //     adjacency_matrix,
                //     &*[<ASSIGNMENT_OPERATOR_ $VALUE_TYPE>],
                //     // Assignment::<$value_type>::new() TODO: it might be that the overhead of dereferncing the Lazy is more expensive than inlining the function call.
                //     &mut transposed_adjacency_matrix,
                //     // &SelectEntireMatrix::new(adjacency_matrix.context_ref()), // TODO: consider caching the selector into the edge store
                //     mask,
                //     &*DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                // )?;

                Ok(transposed_adjacency_matrix)
            }
        }
    };
}
implement_macro_for_all_native_value_types_with_capitalized_value_type!(
    create_transpose_adjacency_matrix_function
);

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::{context::Context, operators::mask::SelectEntireMatrix};

    use crate::graph::{
        edge_store::weighted_adjacency_matrix::{
            traits::{GetEdgeWeight, SetEdge},
            CreateWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
        },
        indexing::VertexIndex,
    };

    #[test]
    fn transpose_adjacency_matrix() {
        let context = Context::init_default().unwrap();

        let mut adjacency_matrix = <WeightedAdjacencyMatrix as CreateWeightedAdjacencyMatrix<
            u32,
        >>::new(context.clone(), 10)
        .unwrap();

        adjacency_matrix
            .set_edge_unchecked(&VertexIndex::new(0), &VertexIndex::new(0), 1e3)
            .unwrap();
        adjacency_matrix
            .set_edge_unchecked(&VertexIndex::new(1), &VertexIndex::new(0), 2e3)
            .unwrap();

        let transposed = transpose_adjacency_matrix_u32(
            &adjacency_matrix,
            &SelectEntireMatrix::new(context.clone()),
        )
        .unwrap();

        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                &transposed,
                &VertexIndex::new(0),
                &VertexIndex::new(0)
            )
            .unwrap()
            .unwrap(),
            1000u32
        );
        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                &transposed,
                &VertexIndex::new(0),
                &VertexIndex::new(1)
            )
            .unwrap()
            .unwrap(),
            2000u32
        );
        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                &transposed,
                &VertexIndex::new(1),
                &VertexIndex::new(0)
            )
            .unwrap(),
            None
        );
    }
}
