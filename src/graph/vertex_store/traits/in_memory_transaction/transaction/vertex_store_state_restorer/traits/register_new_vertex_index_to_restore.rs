use crate::error::GraphComputingError;
use crate::graph::indexing::traits::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::{
    GetVertexStoreStateReverters, VertexStoreStateRestorer,
};

pub(crate) trait RegisterNewVertexIndexToRevert {
    fn register_new_vertex_index_to_revert(
        &mut self,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterNewVertexIndexToRevert for VertexStoreStateRestorer {
    fn register_new_vertex_index_to_revert(
        &mut self,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_state_restorer_mut_ref()
            .register_new_index_to_revert(vertex_index)
    }
}
