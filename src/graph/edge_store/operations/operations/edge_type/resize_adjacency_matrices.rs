use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;

pub(crate) trait ResizeAdjacencyMatrices {
    ///
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}
