use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::sparse_vector_length;

use crate::{
    error::GraphComputingError,
    graph::{index::ElementIndex, vertex_store::VertexVector},
};

pub(crate) trait GetVectorLength {
    fn length(&self) -> Result<ElementIndex, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError>;
}

impl GetVectorLength for VertexVector {
    fn length(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(sparse_vector_length(self)?)
    }
    fn vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(sparse_vector_length(self)?)
    }
}
