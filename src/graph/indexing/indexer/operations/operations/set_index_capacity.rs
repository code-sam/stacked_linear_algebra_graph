use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::ResizeSparseVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::ElementCount;

pub(crate) trait SetIndexCapacity {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError>;
}

pub(crate) fn set_index_capacity(
    indexer: &mut impl GetIndexMask,
    capacity: ElementCount,
) -> Result<(), GraphComputingError> {
    indexer.mask_with_valid_indices_mut_ref().resize(capacity)?; // TODO: if this fails, state will be inconsistent
    Ok(())
}
