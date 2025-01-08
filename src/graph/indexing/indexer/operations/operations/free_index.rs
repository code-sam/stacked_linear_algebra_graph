use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::DeleteSparseVectorElement;

use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::Index;
use crate::graph::indexing::Queue;

use super::CheckIndex;

pub(crate) trait FreeIndex {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError>;

    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
}

pub(crate) fn free_valid_index(
    indexer: &mut (impl GetIndexMask + CheckIndex + FreeIndex),
    index: Index,
) -> Result<(), GraphComputingError> {
    if indexer.is_valid_index(index)? {
        indexer.free_index_unchecked(index)
    } else {
        Ok(())
    }
}

pub(crate) fn free_index_unchecked(
    indexer: &mut (impl GetIndexMask + CheckIndex + GetIndicesAvailableForReuse),
    index: Index,
) -> Result<(), GraphComputingError> {
    indexer
        .mask_with_valid_indices_mut_ref()
        .drop_element(index)?;
    indexer
        .mask_with_valid_indices_mut_ref()
        .drop_element(index)?;

    indexer
        .indices_available_for_reuse_mut_ref()
        .push_back(index);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
