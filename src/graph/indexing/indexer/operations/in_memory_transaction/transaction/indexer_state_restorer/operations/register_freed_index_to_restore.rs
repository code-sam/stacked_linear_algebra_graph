use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::operations::in_memory_transaction::transaction::indexer_state_restorer::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::indexing::Index;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;
use crate::operators::in_memory_transaction::transaction::RegisterSparseVectorChangeToRevert;


pub(crate) trait RegisterFreedIndexToRestore {
    fn register_freed_public_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn register_freed_private_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError>;
}

impl RegisterFreedIndexToRestore for IndexerStateRestorer {
    fn register_freed_public_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_restorer_mut_ref().register_element_value_to_restore(index, true);
        self.mask_with_valid_public_indices_restorer_mut_ref().register_element_value_to_restore(index, true);
        Ok(())
    }

    fn register_freed_private_index_to_restore(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_restorer_mut_ref().register_element_value_to_restore(index, true);
        self.mask_with_private_indices_restorer_mut_ref().register_element_value_to_restore(index, true);
        self.mask_with_valid_private_indices_restorer_mut_ref().register_element_value_to_restore(index, true);
        Ok(())
    }
}