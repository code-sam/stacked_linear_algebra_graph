use crate::graph::indexing::operations::in_memory_transaction::{RegisterFreedIndexToRestore, RegisterNewIndexToRevert};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::vertex_vector::ToSparseVector;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::indexing::{GetAssignedIndexData, GetVertexTypeIndex};
use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::RegisterTypedVertexVectorToRestore;
use crate::graph::vertex_store::vertex_vector::IntoSparseVectorAndClearValuesForValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

pub(crate) trait RegisterNewVertexTypeToRevert {
    fn register_new_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterNewVertexTypeToRevert for VertexStoreStateRestorer {
    fn register_new_vertex_type_to_revert(
        &mut self,
        vertex_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_state_restorer_mut_ref()
            .register_new_index_to_revert(vertex_type_index)
    }
}
