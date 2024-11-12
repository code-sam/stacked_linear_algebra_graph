use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::resize_sparse_vector;

use crate::{
    error::GraphComputingError,
    graph::{indexing::ElementCount, vertex_store::VertexVector},
};

pub(crate) trait ResizeWeightedAdjacencyMatrix {
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
}
