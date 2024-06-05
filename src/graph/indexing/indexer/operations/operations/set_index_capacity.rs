use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::ResizeSparseVector;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, Indexer},
};

pub(crate) trait SetIndexCapacity {
    fn set_index_capacity(&mut self, capacity: &ElementCount) -> Result<(), GraphComputingError>;
}
