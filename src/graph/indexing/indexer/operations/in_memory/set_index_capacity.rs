use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{set_index_capacity, SetIndexCapacity};
use crate::graph::indexing::{ElementCount, Indexer};

impl SetIndexCapacity for Indexer {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError> {
        set_index_capacity(self, capacity)
    }
}
