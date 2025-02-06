use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    sparse_matrix_column_width, sparse_matrix_row_height, sparse_matrix_size, GetSparseMatrixSize,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::ElementCount;

pub trait GetMatrixSize {
    fn size(&self) -> Result<Size, GraphComputingError>;

    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}

impl GetMatrixSize for WeightedAdjacencyMatrix {
    fn size(&self) -> Result<Size, GraphComputingError> {
        Ok(sparse_matrix_size(self)?)
    }

    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(sparse_matrix_column_width(self)?)
    }
}

impl GetSparseMatrixSize for WeightedAdjacencyMatrix {
    fn column_width(
        &self,
    ) -> Result<
        graphblas_sparse_linear_algebra::index::ElementIndex,
        graphblas_sparse_linear_algebra::error::SparseLinearAlgebraError,
    > {
        Ok(sparse_matrix_column_width(self)?)
    }

    fn row_height(
        &self,
    ) -> Result<
        graphblas_sparse_linear_algebra::index::ElementIndex,
        graphblas_sparse_linear_algebra::error::SparseLinearAlgebraError,
    > {
        Ok(sparse_matrix_row_height(self)?)
    }

    fn size(
        &self,
    ) -> Result<Size, graphblas_sparse_linear_algebra::error::SparseLinearAlgebraError> {
        Ok(sparse_matrix_size(self)?)
    }
}
