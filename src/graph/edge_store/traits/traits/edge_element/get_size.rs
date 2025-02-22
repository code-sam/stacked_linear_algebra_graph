use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::{
    error::GraphComputingError,
    graph::indexing::ElementCount,
};

pub trait GetMatrixSize {
    fn size(&self) -> Result<Size, GraphComputingError>;

    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}
