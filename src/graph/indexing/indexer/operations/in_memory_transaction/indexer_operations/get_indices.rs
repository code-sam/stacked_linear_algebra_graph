use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::VectorElementIndexIterator;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::{
    GetIndexerUnderTransaction, InMemoryIndexerTransaction,
};
use crate::graph::indexing::operations::GetValidIndices;
use crate::graph::indexing::ElementIndex;

impl<'t> GetValidIndices for InMemoryIndexerTransaction<'t> {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        self.indexer_ref().mask_with_valid_indices_ref()
    }

    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        self.indexer_ref().valid_indices()
    }

    fn iter_valid_indices(&self) -> Result<VectorElementIndexIterator<bool>, GraphComputingError> {
        self.indexer_ref().iter_valid_indices()
    }
}
