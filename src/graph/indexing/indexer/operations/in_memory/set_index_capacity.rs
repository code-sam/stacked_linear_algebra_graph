use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::ResizeSparseVector;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::operations::{set_index_capacity, SetIndexCapacity};
use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, Indexer},
};

impl SetIndexCapacity for Indexer {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError> {
        set_index_capacity(self, capacity)
    }
}
