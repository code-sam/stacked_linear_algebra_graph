use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::sparse_vector_length;

use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;
use crate::graph::vertex_store::VertexVector;

pub(crate) trait GetVectorLength {
    fn length(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}

impl GetVectorLength for VertexVector {
    fn length(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(sparse_vector_length(self)?)
    }
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(sparse_vector_length(self)?)
    }
}
