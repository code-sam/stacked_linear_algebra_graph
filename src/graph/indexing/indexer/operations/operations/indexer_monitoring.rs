use graphblas_sparse_linear_algebra::collections::{
    sparse_vector::operations::GetSparseVectorLength, Collection,
};

use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, GetIndexMask, Indexer},
};

use super::GetValidIndices;

pub(crate) trait GetIndexerStatus {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError>;
    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}

pub(crate) fn number_of_indexed_elements(
    indexer: &impl GetIndexMask,
) -> Result<ElementCount, GraphComputingError> {
    Ok(indexer
        .mask_with_valid_indices_ref()
        .number_of_stored_elements()?)
}

pub(crate) fn index_capacity(
    indexer: &impl GetIndexMask,
) -> Result<ElementCount, GraphComputingError> {
    Ok(indexer.mask_with_valid_indices_ref().length()?)
}
