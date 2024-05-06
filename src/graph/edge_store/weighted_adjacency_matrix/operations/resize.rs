use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    operations::resize_sparse_matrix, Size,
};

use crate::{
    error::GraphComputingError,
    graph::{edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix, indexing::ElementCount},
};

pub(crate) trait ResizeWeightedAdjacencyMatrix {
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
}

impl ResizeWeightedAdjacencyMatrix for WeightedAdjacencyMatrix {
    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError> {
        Ok(resize_sparse_matrix(
            self,
            &Size::new(new_vertex_capacity, new_vertex_capacity),
        )?)
    }
}
