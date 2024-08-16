use crate::{
    error::GraphComputingError,
    graph::indexing::{operations::GetIndexerStatus, ElementCount},
};

use super::{AtomicInMemoryIndexerTransaction, GetIndexerUnderTransaction};

impl<'t> GetIndexerStatus for AtomicInMemoryIndexerTransaction<'t> {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().number_of_indexed_elements()
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        self.indexer_ref().index_capacity()
    }
}
