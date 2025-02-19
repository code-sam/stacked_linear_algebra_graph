use crate::graph::indexing::indexer::traits::set_index_capacity as set_index_capacity_in_memory;
use crate::graph::indexing::traits::in_memory_transaction::RegisterIndexCapacityToRestore;
use crate::graph::indexing::GetIndexMask;
use crate::{error::GraphComputingError, graph::indexing::ElementCount};

// Implementation of SetIndexCapacity for AtomicInMemoryIndexerTransaction in transaction module itself
// to enable mutable references for indexer and indexer-state-restorer simultaneously.

pub(crate) fn set_index_capacity(
    indexer: &mut impl GetIndexMask,
    indexer_state_restorer: &mut impl RegisterIndexCapacityToRestore,
    capacity: ElementCount,
) -> Result<(), GraphComputingError> {
    set_index_capacity_in_memory(indexer, capacity)?;
    indexer_state_restorer.register_index_capacity_to_restore(capacity)
}
