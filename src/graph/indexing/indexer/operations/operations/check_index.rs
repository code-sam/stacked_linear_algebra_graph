use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValue;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::Index;
use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::indexing::Indexer,
};

pub(crate) trait CheckIndex {
    fn is_valid_index(&self, index: Index) -> Result<bool, GraphComputingError>;
    fn try_index_validity(&self, index: Index) -> Result<(), GraphComputingError>;

    fn is_valid_private_index(&self, index: Index) -> Result<bool, GraphComputingError>;
    fn try_is_valid_private_index(&self, index: Index) -> Result<(), GraphComputingError>;

    fn is_public_index(&self, index: Index) -> Result<bool, GraphComputingError>;
    fn try_is_public_index(&self, index: Index) -> Result<(), GraphComputingError>;

    fn is_valid_public_index(&self, index: Index) -> Result<bool, GraphComputingError>;
    fn try_is_valid_public_index(&self, index: Index) -> Result<(), GraphComputingError>;
}

pub(crate) fn is_valid_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    match indexer.mask_with_valid_indices_ref().element_value(index)? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
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

pub(crate) fn is_valid_private_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    Ok(is_index_private(indexer, index)? && is_valid_index(indexer, index)?)
}

pub(crate) fn try_is_valid_private_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<(), GraphComputingError> {
    if is_valid_private_index(indexer, index)? {
        return Ok(());
    } else {
        return Err(LogicError::new(
            LogicErrorType::IndexOutOfBounds,
            format!(
                "No valid private index [{}], the index may have been freed.",
                index
            ),
            None,
        )
        .into());
    }
}

pub(crate) fn is_public_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    is_index_not_private(indexer, index)
}

pub(crate) fn try_is_public_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<(), GraphComputingError> {
    if is_public_index(indexer, index)? {
        return Ok(());
    } else {
        return Err(LogicError::new(
            LogicErrorType::IndexOutOfBounds,
            format!(
                "No public index [{}], the index may have been freed.",
                index
            ),
            None,
        )
        .into());
    }
}

pub(crate) fn is_valid_public_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    Ok(is_public_index(indexer, index)? && is_valid_index(indexer, index)?)
}

pub(crate) fn try_is_valid_public_index(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<(), GraphComputingError> {
    if is_valid_public_index(indexer, index)? {
        return Ok(());
    } else {
        return Err(LogicError::new(
            LogicErrorType::IndexOutOfBounds,
            format!(
                "No valid public index [{}], the index may have been freed.",
                index
            ),
            None,
        )
        .into());
    }
}

pub(crate) fn is_index_private(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    match indexer
        .mask_with_private_indices_ref()
        .element_value(index)?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub(crate) fn is_index_not_private(
    indexer: &impl GetIndexMask,
    index: Index,
) -> Result<bool, GraphComputingError> {
    match indexer
        .mask_with_private_indices_ref()
        .element_value(index)?
    {
        Some(_) => Ok(false),
        None => Ok(true),
    }
}

#[cfg(test)]
mod tests {}
