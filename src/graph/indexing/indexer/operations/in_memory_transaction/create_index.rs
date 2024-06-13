
use crate::graph::indexing::operations::GeneratePrivateIndex;
use crate::{
    error::GraphComputingError,
    graph::indexing::AssignedIndex,
};

use super::{AtomicInMemoryIndexerTransaction, GetIndexerStateRestorer, GetIndexerUnderTransaction};

impl<'t> GeneratePrivateIndex for AtomicInMemoryIndexerTransaction<'t> {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        self.indexer_mut_ref().new_private_index()
        self.indexer_state_restorer_mut_ref().
    }
}
