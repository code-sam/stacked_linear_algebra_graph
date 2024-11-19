use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::resize_sparse_vector;

use crate::graph::vertex_store::VertexVector;
use crate::graph::indexing::ElementCount;
use crate::error::GraphComputingError;

pub(crate) trait ResizeVertexVector {
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
}

impl ResizeVertexVector for VertexVector {
    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError> {
        Ok(resize_sparse_vector(self, new_vertex_capacity)?)
    }
}
