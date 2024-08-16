use crate::graph::indexing::operations::SetIndexCapacity;
use crate::{
    error::GraphComputingError,
    graph::indexing::ElementCount,
};

use super::{AtomicInMemoryIndexerTransaction, GetIndexerStateRestorer, GetIndexerUnderTransaction, RegisterIndexCapacityToRestore};

impl<'t> SetIndexCapacity for AtomicInMemoryIndexerTransaction<'t> {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError> {
        self.indexer_mut_ref().set_index_capacity(capacity)?;
        self.indexer_state_restorer_mut_ref().register_index_capacity_to_restore(capacity)
    }
}
