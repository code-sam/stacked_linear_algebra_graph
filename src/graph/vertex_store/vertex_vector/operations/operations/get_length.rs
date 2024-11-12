use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::sparse_vector_length;

use crate::{
    error::GraphComputingError,
    graph::{indexing::ElementCount, vertex_store::VertexVector},
};

pub(crate) trait GetVectorLength {
    fn length(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}
