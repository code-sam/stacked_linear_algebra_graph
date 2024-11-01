use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::AtomicInMemoryVertexStoreTransaction;
use crate::graph::vertex_store::operations::ResizeVertexVectors;

impl<'s> ResizeVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.resize_vertex_vectors(new_vertex_capacity)
    }
}
