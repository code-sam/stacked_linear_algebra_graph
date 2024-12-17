use crate::graph::indexing::indexer::operations::set_index_capacity as set_index_capacity_in_memory;
use crate::graph::indexing::operations::in_memory_transaction::{
    GetIndexerStateRestorer, GetIndexerUnderTransaction, InMemoryIndexerTransaction,
    RegisterIndexCapacityToRestore,
};
use crate::graph::indexing::operations::SetIndexCapacity;
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
