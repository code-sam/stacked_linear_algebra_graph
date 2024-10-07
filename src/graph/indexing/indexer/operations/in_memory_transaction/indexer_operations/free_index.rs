use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::operations::free_private_index_unchecked as free_private_index_unchecked_in_memory;
use crate::graph::indexing::operations::free_public_index_unchecked as free_public_index_unchecked_in_memory;
use crate::graph::indexing::operations::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::indexing::operations::try_is_valid_private_index;
use crate::graph::indexing::operations::try_is_valid_public_index;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::GetIndexMask;
use crate::graph::indexing::Index;

// Implementation of FreeIndex for AtomicInMemoryIndexerTransaction in transaction module itself
// to enable mutable references for indexer and indexer-state-restorer simultaneously.

pub(crate) fn free_public_index(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    try_is_valid_public_index(indexer, index)?;
    free_public_index_unchecked(indexer, indexer_state_restorer, index)
}

pub(crate) fn free_private_index(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    try_is_valid_private_index(indexer, index)?;
    free_private_index_unchecked(indexer, indexer_state_restorer, index)
}

pub(crate) fn free_public_index_unchecked(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    free_public_index_unchecked_in_memory(indexer, index)?;
    indexer_state_restorer.register_freed_public_index_to_restore(index)
}

pub(crate) fn free_private_index_unchecked(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    indexer_state_restorer: &mut impl RegisterFreedIndexToRestore,
    index: Index,
) -> Result<(), GraphComputingError> {
    free_private_index_unchecked_in_memory(indexer, index)?;
    indexer_state_restorer.register_freed_private_index_to_restore(index)
}
