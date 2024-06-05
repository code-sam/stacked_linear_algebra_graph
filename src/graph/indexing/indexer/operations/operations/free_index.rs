use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::DeleteSparseVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::Index;
use crate::{error::GraphComputingError, graph::indexing::Indexer};

use super::CheckIndex;

pub(crate) trait FreeIndex {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError>;

    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
}
