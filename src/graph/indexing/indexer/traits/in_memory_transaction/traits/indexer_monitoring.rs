use crate::error::GraphComputingError;
use crate::graph::indexing::traits::in_memory_transaction::{
    GetIndexerUnderTransaction, InMemoryIndexerTransaction,
};
use crate::graph::indexing::traits::GetIndexerStatus;
use crate::graph::indexing::ElementCount;

impl<'t> GetIndexerStatus for InMemoryIndexerTransaction<'t> {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().number_of_indexed_elements()
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().index_capacity()
    }
}
