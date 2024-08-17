use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{
    GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices,
};
use crate::graph::indexing::ElementIndex;

use super::{AtomicInMemoryIndexerTransaction, GetIndexerUnderTransaction};

impl<'t> GetValidIndices for AtomicInMemoryIndexerTransaction<'t> {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        self.indexer_ref().mask_with_valid_indices_ref()
    }

    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        self.indexer_ref().valid_indices()
    }
}

impl<'t> GetValidPublicIndices for AtomicInMemoryIndexerTransaction<'t> {
    fn mask_with_valid_public_indices_ref(&self) -> &SparseVector<bool> {
        self.indexer_ref().mask_with_valid_public_indices_ref()
    }

    fn valid_public_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        self.indexer_ref().valid_public_indices()
    }
}

impl<'t> GetValidPrivateIndices for AtomicInMemoryIndexerTransaction<'t> {
    fn mask_with_valid_private_indices_ref(&self) -> &SparseVector<bool> {
        self.indexer_ref().mask_with_valid_private_indices_ref()
    }

    fn valid_private_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        self.indexer_ref().valid_private_indices()
    }
}
