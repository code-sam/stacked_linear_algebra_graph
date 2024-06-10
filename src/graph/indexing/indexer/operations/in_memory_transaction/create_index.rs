use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::{
    error::GraphComputingError,
    graph::indexing::{AssignedIndex, GetAssignedIndexData, Indexer},
};

pub(crate) trait GeneratePublicIndex {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

impl GeneratePublicIndex for Indexer {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        Ok(index)
    }
}

pub(crate) trait GeneratePrivateIndex {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

impl GeneratePrivateIndex for Indexer {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        self.mask_with_private_indices_mut_ref()
            .set_value(index.index(), true)?;
        Ok(index)
    }
}
