use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElement;
use graphblas_sparse_linear_algebra::collections::Collection;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::{GetIndexCapacity, Index, Queue};
use crate::{
    error::GraphComputingError,
    graph::indexing::{AssignedIndex, GetAssignedIndexData, Indexer},
};

use super::SetIndexCapacity;

pub(crate) trait GeneratePublicIndex {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

pub(crate) trait GeneratePrivateIndex {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

pub(crate) fn new_public_index(
    indexer: &mut (impl GetIndexMask
              + GetIndexCapacity
              + SetIndexCapacity
              + GetIndicesAvailableForReuse),
) -> Result<AssignedIndex, GraphComputingError> {
    let index = claim_available_index(indexer)?;
    indexer
        .mask_with_valid_public_indices_mut_ref()
        .set_value(index.index(), true)?;
    Ok(index)
}

pub(crate) fn new_private_index(
    indexer: &mut (impl GetIndexMask
              + GetIndexCapacity
              + SetIndexCapacity
              + GetIndicesAvailableForReuse),
) -> Result<AssignedIndex, GraphComputingError> {
    let index = claim_available_index(indexer)?;
    indexer
        .mask_with_private_indices_mut_ref()
        .set_value(index.index(), true)?;
    indexer
        .mask_with_valid_private_indices_mut_ref()
        .set_value(index.index(), true)?;
    Ok(index)
}

pub(crate) fn claim_available_index(
    indexer: &mut (impl GetIndexMask
              + GetIndexCapacity
              + SetIndexCapacity
              + GetIndicesAvailableForReuse),
) -> Result<AssignedIndex, GraphComputingError> {
    let is_index_reused: bool;
    let available_index = match indexer.indices_available_for_reuse_mut_ref().pop_front() {
        None => {
            is_index_reused = false;
            indexer
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()?
        }
        Some(index) => {
            is_index_reused = true;
            index
        }
    };

    let new_index;
    if (!is_index_reused) && (available_index >= indexer.capacity()?) {
        let new_capacity = expand_capacity(indexer)?;
        new_index = AssignedIndex::new(available_index, Some(new_capacity), is_index_reused);
    } else {
        new_index = AssignedIndex::new(available_index, None, is_index_reused);
    }

    indexer
        .mask_with_valid_indices_mut_ref()
        .set_value(available_index, true)?;

    Ok(new_index)
}

pub(crate) fn expand_capacity(
    indexer: &mut (impl GetIndexCapacity + SetIndexCapacity),
) -> Result<Index, GraphComputingError> {
    // TODO: test more sophisticated expansion sizing algorithms for better performance
    let new_capacity = indexer.capacity()? * 2;
    indexer.set_index_capacity(new_capacity)?;
    Ok(new_capacity)
}
