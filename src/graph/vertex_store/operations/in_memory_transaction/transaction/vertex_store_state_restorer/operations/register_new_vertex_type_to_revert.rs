use crate::graph::indexing::operations::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::vertex_vector::ToSparseVector;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::indexing::GetVertexTypeIndex;
use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::RegisterTypedVertexVectorToRestore;
use crate::graph::vertex_store::vertex_vector::IntoSparseVectorAndClearValuesForValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

pub(crate) trait RegisterNewVertexTypeToRevert {
    fn register_new_public_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_new_private_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterNewVertexTypeToRevert for VertexStoreStateRestorer {
    fn register_new_public_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_state_restorer_mut_ref()
            .register_freed_public_index_to_restore(vertex_type_index.index())
    }

    fn register_new_private_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_state_restorer_mut_ref()
            .register_freed_private_index_to_restore(vertex_type_index.index())
    }
}
