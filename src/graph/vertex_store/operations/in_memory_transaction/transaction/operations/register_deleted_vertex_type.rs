use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::indexing::GetIndex;
use crate::graph::indexing::{
    AssignedIndex, GetAssignedIndexData, GetVertexTypeIndex, VertexIndex, VertexTypeIndex,
};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStoreStateRestorer, GetVertexStoreStateReverters,
};

pub(crate) trait RegisterDeletedVertexType<'t> {
    fn register_deleted_public_vertex_type(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_deleted_private_vertex_type(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterDeletedVertexType<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_deleted_public_vertex_type(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .vertex_type_indexer_state_restorer_mut_ref()
            .register_freed_public_index_to_restore(vertex_type_index.index())?;

        self.register_vertex_vector_to_restore(vertex_type_index)?;

        Ok(())
    }

    fn register_deleted_private_vertex_type(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .vertex_type_indexer_state_restorer_mut_ref()
            .register_freed_public_index_to_restore(vertex_type_index.index())?;

        self.register_vertex_vector_to_restore(vertex_type_index)?;

        Ok(())
    }
}
