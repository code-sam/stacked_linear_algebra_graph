use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::operations::in_memory_transaction::transaction::indexer_state_restorer::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::indexing::ElementCount;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;

pub(crate) trait RegisterIndexCapacityToRestore {
    fn register_index_capacity_to_restore(
        &mut self,
        capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterIndexCapacityToRestore for IndexerStateRestorer {
    fn register_index_capacity_to_restore(
        &mut self,
        capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        *self.index_capacity_to_restore_mut_ref() = capacity;
        Ok(())
    }
}
