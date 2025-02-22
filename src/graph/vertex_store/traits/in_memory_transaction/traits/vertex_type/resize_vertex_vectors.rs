use crate::error::GraphComputingError;
use crate::graph::indexing::{ElementCount, GetIndexCapacity};
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction, RegisterVertexCapacityToRestore,
};
use crate::graph::vertex_store::traits::vertex_type::ResizeVertexVectors;
use crate::graph::vertex_store::GetVertexElementIndexer;

impl<'s> ResizeVertexVectors for InMemoryVertexStoreTransaction<'s> {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        let current_vertex_capacity = self.vertex_store.element_indexer_ref().capacity()?;
        self.vertex_store_state_restorer
            .register_vertex_capacity_to_restore(&current_vertex_capacity)?;

        self.vertex_store_mut_ref()
            .resize_vertex_vectors(new_vertex_capacity)
    }
}
