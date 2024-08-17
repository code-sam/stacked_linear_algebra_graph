use crate::graph::indexing::operations::{GeneratePrivateIndex, GeneratePublicIndex};
use crate::{error::GraphComputingError, graph::indexing::AssignedIndex};

use super::{
    AtomicInMemoryIndexerTransaction, GetIndexerStateRestorer, GetIndexerUnderTransaction,
    RegisterNewIndexToRevert,
};

impl<'a> GeneratePublicIndex for AtomicInMemoryIndexerTransaction<'a> {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.indexer_mut_ref().new_public_index()?;
        self.indexer_state_restorer_mut_ref()
            .register_new_public_index_to_revert(&index)?;
        Ok(index)
    }
}

impl<'a> GeneratePrivateIndex for AtomicInMemoryIndexerTransaction<'a> {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.indexer_mut_ref().new_private_index()?;
        self.indexer_state_restorer_mut_ref()
            .register_new_private_index_to_revert(&index)?;
        Ok(index)
    }
}
