use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::traits::free_index_unchecked as free_index_unchecked_in_memory;
use crate::graph::indexing::traits::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::indexing::traits::try_index_validity;
use crate::graph::indexing::traits::CheckIndex;
use crate::graph::indexing::GetIndexMask;
use crate::graph::indexing::Index;

// Implementation of FreeIndex for AtomicInMemoryIndexerTransaction in transaction module itself
// to enable mutable references for indexer and indexer-state-restorer simultaneously.

pub(crate) fn free_index(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    try_index_validity(indexer, index)?;
    free_index_unchecked(indexer, indexer_state_restorer, index)
}

pub(crate) fn free_index_unchecked(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    free_index_unchecked_in_memory(indexer, index)?;
    indexer_state_restorer.register_freed_index_to_restore(index)
}
