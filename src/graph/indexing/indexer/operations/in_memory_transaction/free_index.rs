use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::operations::FreeIndex;
use crate::graph::indexing::Index;

use super::AtomicInMemoryIndexerTransaction;
use super::GetIndexerStateRestorer;
use super::GetIndexerUnderTransaction;
use super::RegisterFreedIndexToRestore;

impl<'t> FreeIndex for AtomicInMemoryIndexerTransaction<'t> {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_public_index(index)?;
        self.free_public_index_unchecked(index)
    }

    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_private_index(index)?;
        self.free_private_index_unchecked(index)
    }

    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_mut_ref().free_public_index_unchecked(index)?;
        self.indexer_state_restorer_mut_ref()
            .register_freed_public_index_to_restore(index)
    }

    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_mut_ref().free_private_index_unchecked(index)?;
        self.indexer_state_restorer_mut_ref()
            .register_freed_private_index_to_restore(index)
    }
}
