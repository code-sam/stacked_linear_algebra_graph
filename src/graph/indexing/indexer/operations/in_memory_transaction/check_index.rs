use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::Index;

use super::{AtomicInMemoryIndexerTransaction, GetIndexerUnderTransaction};

impl<'t> CheckIndex for AtomicInMemoryIndexerTransaction<'t> {
    fn is_valid_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_index(index)
    }

    fn try_index_validity(&self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_index_validity(index)
    }

    fn is_valid_private_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_private_index(index)
    }

    fn try_is_valid_private_index(&self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_public_index(index)
    }

    fn is_public_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_public_index(index)
    }

    fn try_is_public_index(&self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_public_index(index)
    }

    fn is_valid_public_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_public_index(index)
    }

    fn try_is_valid_public_index(&self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_public_index(index)
    }
}

#[cfg(test)]
mod tests {}
