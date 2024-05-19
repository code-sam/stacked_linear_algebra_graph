use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::ResizeSparseVector;

use crate::{error::GraphComputingError, graph::indexing::{ElementCount, Indexer}};
use crate::graph::indexing::indexer::indexer::GetIndexMask;


pub(crate) trait SetIndexCapacity {
    fn set_index_capacity(&mut self, capacity: &ElementCount) -> Result<(), GraphComputingError>;
}

impl SetIndexCapacity for Indexer {
    fn set_index_capacity(&mut self, capacity: &ElementCount) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_mut_ref()
            .resize(*capacity)?; // TODO: if this fails, state will be inconsistent
        self.mask_with_private_indices_mut_ref().resize(*capacity)?;
        Ok(())
    }
}