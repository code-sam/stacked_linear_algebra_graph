use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::{RegisterFreedIndexToRestore, RegisterNewIndexToRevert};
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData, GetVertexIndexIndex, VertexIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::{RegisterEmptyVertexToRestore, RegisterVertexValueToRestore};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{AtomicInMemoryVertexStoreTransaction, GetVertexStoreStateRestorer};
use crate::graph::indexing::GetIndex;

pub(crate) trait RegisterDeletedVertex<'t> {
    fn register_deleted_public_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_deleted_private_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterDeletedVertex<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_deleted_public_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_freed_public_index_to_restore(vertex_index.index())?;

        self.register_vertex_value_to_restore(vertex_type_index, vertex_index)?;
        Ok(())
    }

    fn register_deleted_private_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_freed_private_index_to_restore(vertex_index.index())?;

        self.register_vertex_value_to_restore(vertex_type_index, vertex_index)?;
        Ok(())
    }
}
