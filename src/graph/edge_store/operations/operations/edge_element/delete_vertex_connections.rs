use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;

pub(crate) trait DeleteVertexConnections {
    fn delete_vertex_connections_unchecked(
        &mut self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}
