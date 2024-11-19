use crate::error::GraphComputingError;
use crate::graph::indexing::{ElementCount, GetIndexCapacity, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, RegisterVertexCapacityToRestore,
};
use crate::graph::vertex_store::operations::vertex_type::ResizeVertexVectors;
use crate::graph::vertex_store::{GetVertexElementIndexer, GetVertexVectors};

impl<'s> ResizeVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        let current_vertex_capacity = self.vertex_store.element_indexer_ref().capacity()?;
        for (vertex_type_index, vertex_vector) in self
            .vertex_store
            .vertex_vector_for_all_vertex_types_ref()
            .iter()
            .enumerate()
        {
            self.vertex_store_state_restorer
                .register_vertex_capacity_to_restore(
                    &VertexTypeIndex::new(vertex_type_index),
                    vertex_vector,
                    &current_vertex_capacity,
                )?;
        }

        self.vertex_store_mut_ref()
            .resize_vertex_vectors(new_vertex_capacity)
    }
}
