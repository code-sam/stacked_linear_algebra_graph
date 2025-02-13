use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::operations::in_memory_transaction::transaction::indexer_state_restorer::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::indexing::Index;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;
use crate::operators::transaction::in_memory::RegisterSparseVectorChangeToRevert;

pub(crate) trait RegisterFreedIndexToRestore {
    fn register_freed_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError>;
}

impl RegisterFreedIndexToRestore for IndexerStateRestorer {
    fn register_freed_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_restorer_mut_ref()
            .register_element_value_to_restore(index, true);
        self.mask_with_valid_indices_restorer_mut_ref()
            .register_element_value_to_restore(index, true);
        Ok(())
    }
}
