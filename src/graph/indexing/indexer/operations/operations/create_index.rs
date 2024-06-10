use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::{
    error::GraphComputingError,
    graph::indexing::{AssignedIndex, GetAssignedIndexData, Indexer},
};

pub(crate) trait GeneratePublicIndex {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

pub(crate) trait GeneratePrivateIndex {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}
