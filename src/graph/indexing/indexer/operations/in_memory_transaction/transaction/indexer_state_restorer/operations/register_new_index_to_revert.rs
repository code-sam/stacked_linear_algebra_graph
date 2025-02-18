use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::operations::in_memory_transaction::transaction::indexer_state_restorer::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;
use crate::transaction::in_memory::{RegisterQueueChangeToRevert, RegisterSparseVectorChangeToRevert};

pub(crate) trait RegisterNewIndexToRevert {
    fn register_new_index_to_revert(
        &mut self,
        index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterNewIndexToRevert for IndexerStateRestorer {
    fn register_new_index_to_revert(
        &mut self,
        index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        if index.is_reused() {
            self.indices_available_for_reuse_restorer_mut_ref()
                .front_popped_value_to_restore(index.index());
        }
        self.mask_with_valid_indices_restorer_mut_ref()
            .register_empty_element_to_restore(index.index());
        Ok(())
    }
}
