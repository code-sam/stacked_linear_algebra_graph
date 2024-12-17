use crate::{
    error::GraphComputingError,
    graph::indexing::{
        operations::{
            in_memory_transaction::{GetIndexerUnderTransaction, InMemoryIndexerTransaction},
            GetIndexerStatus,
        },
        ElementCount,
    },
};

impl<'t> GetIndexerStatus for InMemoryIndexerTransaction<'t> {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().number_of_indexed_elements()
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().index_capacity()
    }
}
