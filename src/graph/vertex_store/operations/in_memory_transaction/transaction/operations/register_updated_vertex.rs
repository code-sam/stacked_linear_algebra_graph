use crate::error::GraphComputingError;
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData, GetVertexIndexIndex, GetVertexTypeIndex, VertexIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::{RegisterVertexCapacityToRestore, RegisterVertexValueToRestore};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::AtomicInMemoryVertexStoreTransaction;

pub(crate) trait RegisterUpdatedVertex<'t> {
    fn register_updated_vertex(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterUpdatedVertex<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_updated_vertex(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_vertex_value_to_restore(vertex_type_index, vertex_index)?;
        Ok(())
    }
}
