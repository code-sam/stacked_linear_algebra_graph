use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::ResizeSparseVector;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, Indexer},
};

pub(crate) trait SetIndexCapacity {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError>;
}

pub(crate) fn set_index_capacity(
    indexer: &mut impl GetIndexMask,
    capacity: ElementCount,
) -> Result<(), GraphComputingError> {
    indexer.mask_with_valid_indices_mut_ref().resize(capacity)?; // TODO: if this fails, state will be inconsistent
    indexer
        .mask_with_private_indices_mut_ref()
        .resize(capacity)?;
    indexer
        .mask_with_valid_private_indices_mut_ref()
        .resize(capacity)?;
    indexer
        .mask_with_valid_public_indices_mut_ref()
        .resize(capacity)?;
    Ok(())
}
