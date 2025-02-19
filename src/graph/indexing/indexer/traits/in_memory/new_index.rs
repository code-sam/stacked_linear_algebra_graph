use crate::error::GraphComputingError;
use crate::graph::indexing::traits::{new_index, GenerateIndex};
use crate::graph::indexing::{AssignedIndex, Indexer};

impl GenerateIndex for Indexer {
    fn new_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        new_index(self)
    }
}
