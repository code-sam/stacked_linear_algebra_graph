use crate::graph::indexing::indexer::operations::operations::new_private_index as new_private_index_in_memory;
use crate::graph::indexing::indexer::operations::operations::new_public_index as new_public_index_in_memory;
use crate::graph::indexing::operations::in_memory_transaction::{
    GetIndexerStateRestorer, GetIndexerUnderTransaction, InMemoryIndexerTransaction,
    RegisterNewIndexToRevert,
};
use crate::graph::indexing::operations::{
    CheckIndex, GeneratePrivateIndex, GeneratePublicIndex, SetIndexCapacity,
};
use crate::graph::indexing::{GetIndexCapacity, GetIndexMask, GetIndicesAvailableForReuse};
use crate::{error::GraphComputingError, graph::indexing::AssignedIndex};

// Implementation of GeneratePublicIndex and GeneratePrivateIndex for AtomicInMemoryIndexerTransaction in transaction module itself
// to enable mutable references for indexer and indexer-state-restorer simultaneously.

pub(crate) fn new_public_index(
    indexer: &mut (impl GetIndexMask
              + GetIndicesAvailableForReuse
              + GetIndexCapacity
              + SetIndexCapacity),
    indexer_state_restorer: &mut impl RegisterNewIndexToRevert,
) -> Result<AssignedIndex, GraphComputingError> {
    let index = new_public_index_in_memory(indexer)?;
    indexer_state_restorer.register_new_public_index_to_revert(&index)?;
    Ok(index)
}

pub(crate) fn new_private_index(
    indexer: &mut (impl GetIndexMask
              + GetIndicesAvailableForReuse
              + GetIndexCapacity
              + SetIndexCapacity),
    indexer_state_restorer: &mut impl RegisterNewIndexToRevert,
) -> Result<AssignedIndex, GraphComputingError> {
    let index = new_private_index_in_memory(indexer)?;
    indexer_state_restorer.register_new_private_index_to_revert(&index)?;
    Ok(index)
}
