use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::RegisterAdjacencyMatrixToRestore;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, GetEdgeStoreStateRestorer, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_type::delete_edge_type::DropEdgeType;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::indexing::GetEdgeTypeIndex;

impl<'s> DropEdgeType for InMemoryEdgeStoreTransaction<'s> {
    fn drop_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(edge_type_index)?;
        self.drop_edge_type_unchecked(edge_type_index)
    }

    fn drop_edge_type_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store.drop_edge_type_unchecked(edge_type_index)?;

        let adjacency_matrix_to_restore = self
            .edge_store
            .adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index);

        self.edge_store_state_restorer
            .register_deleted_adjacency_matrix_to_restore(
                edge_type_index,
                adjacency_matrix_to_restore,
            )
    }
}
