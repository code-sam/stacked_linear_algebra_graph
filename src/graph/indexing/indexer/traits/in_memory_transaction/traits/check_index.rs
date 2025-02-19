use crate::error::GraphComputingError;
use crate::graph::indexing::traits::in_memory_transaction::{
    GetIndexerUnderTransaction, InMemoryIndexerTransaction,
};
use crate::graph::indexing::traits::CheckIndex;
use crate::graph::indexing::Index;

impl<'t> CheckIndex for InMemoryIndexerTransaction<'t> {
    fn is_valid_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_index(index)
    }

    fn try_index_validity(&self, index: Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_index_validity(index)
    }
}

#[cfg(test)]
mod tests {}
