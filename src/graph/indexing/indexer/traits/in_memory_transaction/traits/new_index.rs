use crate::graph::indexing::indexer::traits::traits::new_index as new_index_in_memory;
use crate::graph::indexing::traits::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::traits::SetIndexCapacity;
use crate::graph::indexing::{GetIndexCapacity, GetIndexMask, GetIndicesAvailableForReuse};
use crate::{error::GraphComputingError, graph::indexing::AssignedIndex};

// Implementation of GeneratePublicIndex and GeneratePrivateIndex for AtomicInMemoryIndexerTransaction in transaction module itself
// to enable mutable references for indexer and indexer-state-restorer simultaneously.

pub(crate) fn new_index(
    indexer: &mut (impl GetIndexMask
              + GetIndicesAvailableForReuse
              + GetIndexCapacity
              + SetIndexCapacity),
    indexer_state_restorer: &mut impl RegisterNewIndexToRevert,
) -> Result<AssignedIndex, GraphComputingError> {
    let index = new_index_in_memory(indexer)?;
    indexer_state_restorer.register_new_index_to_revert(&index)?;
    Ok(index)
}
