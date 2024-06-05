use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::operations::{GeneratePrivateIndex, GeneratePublicIndex};
use crate::{
    error::GraphComputingError,
    graph::indexing::{AssignedIndex, GetAssignedIndexData, Indexer},
};

impl GeneratePublicIndex for Indexer {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        self.mask_with_valid_public_indices_mut_ref()
            .set_value(index.index_ref(), true)?;
        Ok(index)
    }
}

impl GeneratePrivateIndex for Indexer {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        self.mask_with_private_indices_mut_ref()
            .set_value(index.index_ref(), true)?;
        self.mask_with_valid_private_indices_mut_ref()
            .set_value(index.index_ref(), true)?;
        Ok(index)
    }
}
