use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_vectors_state_restorer::RegisterEmptyVertexToRestore;
use crate::graph::vertex_store::operations::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    GetVertexVectorNativeValueType,
};

pub(crate) trait RegisterNewVertexType<'t> {
    fn register_new_public_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;

    fn register_new_private_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterNewVertexType<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_new_public_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .vertex_type_indexer_state_restorer_mut_ref()
            .register_new_public_index_to_revert(vertex_type_index)?;
        Ok(())
    }

    fn register_new_private_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .vertex_type_indexer_state_restorer_mut_ref()
            .register_new_private_index_to_revert(vertex_type_index)?;
        Ok(())
    }
}
