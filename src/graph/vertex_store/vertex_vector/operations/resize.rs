use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::resize_sparse_vector;

use crate::{
    error::GraphComputingError,
    graph::{indexing::ElementCount, vertex_store::VertexVector},
};

pub(crate) trait ResizeWeightedAdjacencyMatrix {
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
}

impl ResizeWeightedAdjacencyMatrix for VertexVector {
    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError> {
        Ok(resize_sparse_vector(self, new_vertex_capacity)?)
    }
}
