use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::VectorElementIndexIterator;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    operations::GetSparseVectorElementIndices, SparseVector,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::ElementIndex;

pub(crate) trait GetValidIndices {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
    fn iter_valid_indices(&self) -> Result<VectorElementIndexIterator<bool>, GraphComputingError>;
}

pub(crate) trait GetValidPublicIndices {
    fn mask_with_valid_public_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_public_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
    fn iter_valid_public_indices(
        &self,
    ) -> Result<VectorElementIndexIterator<bool>, GraphComputingError>;
}

pub(crate) trait GetValidPrivateIndices {
    fn mask_with_valid_private_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_private_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
    fn iter_valid_private_indices(
        &self,
    ) -> Result<VectorElementIndexIterator<bool>, GraphComputingError>;
}

// The mask is updated at each push() and free() operation.
// benefit: mask is pre-computed, resulting in faster query operations
// downside: slower push() and free() operations
pub(crate) fn mask_with_valid_indices_ref(indexer: &impl GetIndexMask) -> &SparseVector<bool> {
    GetIndexMask::mask_with_valid_indices_ref(indexer)
}

pub(crate) fn valid_indices(
    indexer: &impl GetIndexMask,
) -> Result<Vec<ElementIndex>, GraphComputingError> {
    // self.key_to_index_map.values().into_iter().collect()
    Ok(GetIndexMask::mask_with_valid_indices_ref(indexer).element_indices()?)
}

pub(crate) fn iter_valid_indices(
    indexer: &impl GetIndexMask,
) -> Result<VectorElementIndexIterator<bool>, GraphComputingError> {
    let index_iterator =
        VectorElementIndexIterator::new(GetIndexMask::mask_with_valid_indices_ref(indexer))?;
    Ok(index_iterator)
}

// The mask is updated at each push() and free() operation.
// benefit: mask is pre-computed, resulting in faster query operations
// downside: slower push() and free() operations
pub(crate) fn mask_with_valid_public_indices_ref(
    indexer: &impl GetIndexMask,
) -> &SparseVector<bool> {
    GetIndexMask::mask_with_valid_public_indices_ref(indexer)
}

pub(crate) fn valid_public_indices(
    indexer: &impl GetIndexMask,
) -> Result<Vec<ElementIndex>, GraphComputingError> {
    // self.key_to_index_map.values().into_iter().collect()
    Ok(GetIndexMask::mask_with_valid_public_indices_ref(indexer).element_indices()?)
}

pub(crate) fn iter_valid_public_indices(
    indexer: &impl GetIndexMask,
) -> Result<VectorElementIndexIterator<bool>, GraphComputingError> {
    let index_iterator =
        VectorElementIndexIterator::new(GetIndexMask::mask_with_valid_public_indices_ref(indexer))?;
    Ok(index_iterator)
}

// The mask is updated at each push() and free() operation.
// benefit: mask is pre-computed, resulting in faster query operations
// downside: slower push() and free() operations
pub(crate) fn mask_with_valid_private_indices_ref(
    indexer: &impl GetIndexMask,
) -> &SparseVector<bool> {
    GetIndexMask::mask_with_valid_private_indices_ref(indexer)
}

pub(crate) fn valid_private_indices(
    indexer: &impl GetIndexMask,
) -> Result<Vec<ElementIndex>, GraphComputingError> {
    // self.key_to_index_map.values().into_iter().collect()
    Ok(GetIndexMask::mask_with_valid_private_indices_ref(indexer).element_indices()?)
}

pub(crate) fn iter_valid_private_indices(
    indexer: &impl GetIndexMask,
) -> Result<VectorElementIndexIterator<bool>, GraphComputingError> {
    let index_iterator = VectorElementIndexIterator::new(
        GetIndexMask::mask_with_valid_private_indices_ref(indexer),
    )?;
    Ok(index_iterator)
}

#[cfg(test)]
mod tests {}
