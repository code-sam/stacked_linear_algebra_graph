use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::DeleteSparseVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::Index;
use crate::{
    error::GraphComputingError,
    graph::indexing::Indexer,
};

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

impl FreeIndex for Indexer {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        if self.is_valid_index(&index)? {
            self.free_index_unchecked(index)
        } else {
            Ok(())
        }
    }

    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        if self.is_valid_index(&index)? {
            self.free_public_index_unchecked(index)
        } else {
            Ok(())
        }
    }

    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.free_valid_index(index)
    }

    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_mut_ref()
            .drop_element(index.to_owned())?;

        self.mask_with_private_indices_mut_ref()
            .drop_element(index.to_owned())?;

        self.indices_available_for_reuse_mut_ref().push_back(index);
        Ok(())
    }

    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_mut_ref()
            .drop_element(index.to_owned())?;
        self.indices_available_for_reuse_mut_ref().push_back(index);
        Ok(())
    }

    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.free_index_unchecked(index)
    }
}
