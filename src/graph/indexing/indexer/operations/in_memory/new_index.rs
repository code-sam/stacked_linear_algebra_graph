use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElement;
use graphblas_sparse_linear_algebra::collections::Collection;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::operations::{
    new_private_index, new_public_index, GeneratePrivateIndex, GeneratePublicIndex,
};
use crate::graph::indexing::{GetIndexCapacity, Index, Queue};
use crate::{
    error::GraphComputingError,
    graph::indexing::{AssignedIndex, GetAssignedIndexData, Indexer},
};

impl GeneratePublicIndex for Indexer {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        new_public_index(self)
    }
}

impl GeneratePrivateIndex for Indexer {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        new_private_index(self)
    }
}
