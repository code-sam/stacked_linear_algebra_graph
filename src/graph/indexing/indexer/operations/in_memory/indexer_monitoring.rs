use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{
    index_capacity, number_of_indexed_elements, GetIndexerStatus,
};
use crate::graph::indexing::{ElementCount, Indexer};

impl GetIndexerStatus for Indexer {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        number_of_indexed_elements(self)
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        index_capacity(self)
    }
}
