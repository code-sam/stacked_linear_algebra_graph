use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::{
    RegisterFreedIndexToRestore, RegisterNewIndexToRevert,
};
use crate::graph::indexing::{
    AssignedIndex, GetAssignedIndexData, GetVertexTypeIndex, VertexIndex, VertexTypeIndex,
};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    GetVertexVector, GetVertexVectorNativeValueType,
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
