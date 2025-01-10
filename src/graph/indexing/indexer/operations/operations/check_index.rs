use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::IsSparseVectorElement;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::Index;

pub(crate) trait CheckIndex {
    fn is_valid_index(&self, index: Index) -> Result<bool, GraphComputingError>;
    fn try_index_validity(&self, index: Index) -> Result<(), GraphComputingError>;
}

pub(crate) fn is_valid_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    Ok(indexer.mask_with_valid_indices_ref().is_element(index)?)
}

pub(crate) fn try_index_validity(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<(), GraphComputingError> {
    if is_valid_index(indexer, index)? {
        return Ok(());
    } else {
        return Err(LogicError::new(
            LogicErrorType::IndexOutOfBounds,
            format!("No valid index [{}], the index may have been freed.", index),
            None,
        )
        .into());
    }
}

#[cfg(test)]
mod tests {}
