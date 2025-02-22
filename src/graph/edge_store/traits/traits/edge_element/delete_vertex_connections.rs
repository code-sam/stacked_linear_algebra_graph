use crate::error::GraphComputingError;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;

pub(crate) trait DeleteVertexConnections {
    fn delete_vertex_connections(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_connections_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}
