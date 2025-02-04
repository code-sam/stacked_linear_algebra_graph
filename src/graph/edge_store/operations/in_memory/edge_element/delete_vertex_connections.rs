use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_element::DeleteVertexConnections;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::weighted_adjacency_matrix::operations::DeleteVertexConnections as DeleteVertexConnectionsFromAdjacencyMatrix;

impl DeleteVertexConnections for EdgeStore {
    fn delete_vertex_connections(
        &mut self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(edge_type_index)?;
        vertex_indexer.try_vertex_index_validity(vertex_index)?;

        self.delete_vertex_connections_unchecked(edge_type_index, vertex_index)
    }

    fn delete_vertex_connections_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_mut_ref_unchecked(edge_type_index)?
            .delete_vertex_connections_unchecked(vertex_index)
    }
}
