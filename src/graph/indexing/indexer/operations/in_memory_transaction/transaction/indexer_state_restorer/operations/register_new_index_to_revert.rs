use graphblas_sparse_linear_algebra::error::SparseLinearAlgebraError;

use crate::graph::indexing::indexer::operations::in_memory_transaction::transaction::indexer_state_restorer::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;
use crate::operators::in_memory_transaction::transaction::{RegisterQueueChangeToRevert, RegisterSparseVectorChangeToRevert};


pub(crate) trait RegisterNewIndexToRevert {
    fn register_new_public_index_to_revert(&mut self, index: &impl GetAssignedIndexData) -> Result<(), SparseLinearAlgebraError>;
    fn register_new_private_index_to_revert(&mut self, index: &impl GetAssignedIndexData) -> Result<(), SparseLinearAlgebraError>;
}

impl RegisterNewIndexToRevert for IndexerStateRestorer {
    fn register_new_public_index_to_revert(&mut self, index: &impl GetAssignedIndexData) -> Result<(), SparseLinearAlgebraError> {
        if index.is_reused() {
            self.indices_available_for_reuse_restorer_mut_ref().front_popped_value_to_restore(index.index());
        }
        self.mask_with_valid_indices_restorer_mut_ref().register_empty_element_to_restore(index.index());
        self.mask_with_valid_public_indices_restorer_mut_ref().register_empty_element_to_restore(index.index());
        Ok(())
    }

    fn register_new_private_index_to_revert(&mut self, index: &impl GetAssignedIndexData) -> Result<(), SparseLinearAlgebraError> {
        if index.is_reused() {
            self.indices_available_for_reuse_restorer_mut_ref().front_popped_value_to_restore(index.index());
        }
        self.mask_with_valid_indices_restorer_mut_ref().register_empty_element_to_restore(index.index());
        self.mask_with_private_indices_restorer_mut_ref().register_empty_element_to_restore(index.index());
        self.mask_with_valid_private_indices_restorer_mut_ref().register_empty_element_to_restore(index.index());
        Ok(())
    }
}