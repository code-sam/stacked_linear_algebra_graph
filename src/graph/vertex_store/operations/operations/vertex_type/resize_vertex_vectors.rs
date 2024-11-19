use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;

pub(crate) trait ResizeVertexVectors {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}
