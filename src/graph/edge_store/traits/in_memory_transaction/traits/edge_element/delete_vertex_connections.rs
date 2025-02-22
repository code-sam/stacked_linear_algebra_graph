use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::in_memory_transaction::InMemoryEdgeStoreTransaction;
use crate::graph::edge_store::traits::in_memory_transaction::RegisterAdjacencyMatrixToRestore;
use crate::graph::edge_store::traits::traits::edge_element::DeleteVertexConnections;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::traits::traits::edge_type::indexing::Indexing;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;
use crate::graph::weighted_adjacency_matrix::traits::DeleteVertexConnections as DeleteVertexConnectionsFromAdjacencyMatrix;

impl<'s> DeleteVertexConnections for InMemoryEdgeStoreTransaction<'s> {
    fn delete_vertex_connections(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
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
        let adjacency_matrix_restore = self
            .edge_store
            .adjacency_matrix_ref_unchecked(edge_type_index);

        // TODO: This copies the entire matrix. Consider if the back-up can be implemented more efficiently.
        self.edge_store_state_restorer
            .register_updated_adjacency_matrix_to_restore(
                edge_type_index,
                adjacency_matrix_restore,
            )?;

        self.adjacency_matrix_mut_ref_unchecked(edge_type_index)?
            .delete_vertex_connections_unchecked(vertex_index)
    }
}
