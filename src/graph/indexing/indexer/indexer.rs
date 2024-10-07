use std::cmp::max;
use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorLength, SetSparseVectorElement,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::Collection;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::{AssignedIndex, ElementCount, Index};

use super::operations::SetIndexCapacity;
use super::{Queue, VecDequeQueue};

pub(crate) const MINIMUM_INDEXER_CAPACITY: usize = 1;

#[derive(Clone, Debug)]
pub(crate) struct Indexer {
    _graphblas_context: Arc<GraphBLASContext>,
    select_entire_vector: SelectEntireVector,

    indices_available_for_reuse: VecDequeQueue<Index>,

    mask_with_valid_indices: SparseVector<bool>,
    mask_with_private_indices: SparseVector<bool>,
    mask_with_valid_private_indices: SparseVector<bool>,
    mask_with_valid_public_indices: SparseVector<bool>,
}

pub(crate) trait GetIndicesAvailableForReuse {
    fn indices_available_for_reuse_ref(&self) -> &VecDequeQueue<Index>;
    fn indices_available_for_reuse_mut_ref(&mut self) -> &mut VecDequeQueue<Index>;
}

pub(crate) trait GetIndexMask {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_valid_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_private_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_valid_private_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_valid_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_valid_public_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_valid_public_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;
}

impl GetIndicesAvailableForReuse for Indexer {
    fn indices_available_for_reuse_ref(&self) -> &VecDequeQueue<Index> {
        &self.indices_available_for_reuse
    }

    fn indices_available_for_reuse_mut_ref(&mut self) -> &mut VecDequeQueue<Index> {
        &mut self.indices_available_for_reuse
    }
}

impl GetIndexMask for Indexer {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_indices
    }

    fn mask_with_valid_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        &mut self.mask_with_valid_indices
    }

    fn mask_with_valid_private_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_private_indices
    }

    fn mask_with_valid_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        &mut self.mask_with_valid_private_indices
    }

    fn mask_with_valid_public_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_public_indices
    }

    fn mask_with_valid_public_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        &mut self.mask_with_valid_public_indices
    }

    fn mask_with_private_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_private_indices
    }

    fn mask_with_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        &mut self.mask_with_private_indices
    }
}

pub(crate) trait GetQueueWithIndicesForReuse {
    fn queue_with_indices_for_reuse_ref(&self) -> &VecDequeQueue<Index>;
}

impl GetQueueWithIndicesForReuse for Indexer {
    fn queue_with_indices_for_reuse_ref(&self) -> &VecDequeQueue<Index> {
        &self.indices_available_for_reuse
    }
}

pub(crate) trait GetIndexCapacity {
    fn capacity(&self) -> Result<Index, GraphComputingError>;
}

impl GetIndexCapacity for Indexer {
    fn capacity(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices_ref().length()?)
    }
}

impl Indexer {
    // NOTE: setting and enforcing this minimum improves performance,
    // as the minimum is guaranteed once and no longer needs checking upon capacity expansion.
    // However, the API is slightly misleading for initial_capacity = 0.

    pub(crate) fn new(
        graphblas_context: Arc<GraphBLASContext>,
    ) -> Result<Self, GraphComputingError> {
        let default_initial_capacity = 256;
        Self::with_initial_capacity(graphblas_context, default_initial_capacity)
    }

    /// Sets a minimum capacity of 1, if initial_capacity = 0
    pub(crate) fn with_initial_capacity(
        graphblas_context: Arc<GraphBLASContext>,
        initial_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let initial_capacity = max(initial_capacity.clone(), MINIMUM_INDEXER_CAPACITY);

        let empty_bool_vector: SparseVector<bool> =
            SparseVector::new(graphblas_context.clone(), initial_capacity)?;

        Ok(Self {
            _graphblas_context: graphblas_context.clone(),
            select_entire_vector: SelectEntireVector::new(graphblas_context.clone()),
            indices_available_for_reuse: VecDequeQueue::new(),
            mask_with_valid_indices: empty_bool_vector.clone(),
            mask_with_private_indices: empty_bool_vector.clone(),
            mask_with_valid_private_indices: empty_bool_vector.clone(),
            mask_with_valid_public_indices: empty_bool_vector,
        })
    }

    // Method is implementation-specific, and therefore not part of the IndexerTrait
    pub(crate) fn get_number_of_stored_and_reusable_elements(
        &self,
    ) -> Result<Index, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .number_of_stored_elements()?
            + self.indices_available_for_reuse.length())
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValue;

    use crate::graph::indexing::indexer::operations::GetIndexerStatus;
    use crate::graph::indexing::{
        operations::{CheckIndex, FreeIndex, GeneratePublicIndex},
        GetAssignedIndexData,
    };

    use super::*;

    #[test]
    fn new_indexer() {
        let initial_capacity = 10;
        let mut indexer = Indexer::with_initial_capacity(
            GraphBLASContext::init_default().unwrap(),
            initial_capacity,
        )
        .unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            0
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);

        let index = indexer.new_public_index().unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 1);
        assert_eq!(indexer.is_valid_index(index.index()).unwrap(), true);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            1
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .element_value(index.index())
                .unwrap(),
            Some(true)
        );

        indexer
            .free_public_index(index.index_ref().clone())
            .unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);
        assert_eq!(indexer.is_valid_index(index.index()).unwrap(), false);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            0
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .element_value(index.index())
                .unwrap(),
            None
        );
    }

    #[test]
    fn new_store_with_zero_capacity() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for _i in 0..n_indices {
            let new_public_index = indexer.new_public_index().unwrap();
            indices.push(new_public_index);
        }

        indexer
            .free_public_index(indices[2].index_ref().clone())
            .unwrap();
        indexer
            .free_public_index(indices[20].index_ref().clone())
            .unwrap();
        indexer
            .free_public_index(indices[92].index_ref().clone())
            .unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            n_indices
        );
        assert_eq!(indexer.is_valid_index(indices[0].index()).unwrap(), true);
        assert_eq!(indexer.is_valid_index(indices[10].index()).unwrap(), true);
        assert_eq!(indexer.is_valid_index(indices[33].index()).unwrap(), true);
        assert_eq!(indexer.is_valid_index(indices[77].index()).unwrap(), true);
        assert_eq!(indexer.is_valid_index(indices[99].index()).unwrap(), true);
        assert_eq!(indexer.is_valid_index(indices[2].index()).unwrap(), false);
        assert_eq!(indexer.is_valid_index(indices[20].index()).unwrap(), false);
        assert_eq!(indexer.is_valid_index(indices[92].index()).unwrap(), false);

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            n_indices
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), n_indices - 3);

        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            n_indices - 3
        );
        assert_eq!(
            mask_with_valid_indices
                .element_value(indices[33].index())
                .unwrap(),
            Some(true)
        );
        assert_eq!(
            mask_with_valid_indices
                .element_value(indices[20].index())
                .unwrap(),
            None
        );
    }

    #[test]
    fn delete_same_key_multiple_times() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 10).unwrap();

        let mut indices = Vec::new();
        let n_indices = 10;
        for _i in 0..n_indices {
            indices.push(indexer.new_public_index().unwrap());
        }

        for _i in 0..20 {
            match indexer.free_public_index(1) {
                // deleting the same key multiple times will result in errors, this error is not tested.
                Ok(_) => (),
                Err(_) => (),
            }
        }

        assert!(!indexer.is_valid_index(1).unwrap());
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            9
        );

        for _i in 0..n_indices {
            indices.push(indexer.new_public_index().unwrap());
        }

        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 19);
    }

    #[test]
    fn test_mask_with_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for i in 0..n_indices {
            indices.push(indexer.new_public_index().unwrap());
            assert_eq!(
                indexer
                    .mask_with_valid_indices_ref()
                    .element_value_or_default(i)
                    .unwrap(),
                true
            );
            assert_eq!(
                indexer.mask_with_valid_indices_ref().length().unwrap(),
                indexer.capacity().unwrap()
            );
            assert_eq!(
                indexer
                    .mask_with_valid_indices_ref()
                    .number_of_stored_elements()
                    .unwrap(),
                i + 1
            );
        }

        indexer.free_public_index(0).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .element_value_or_default(0)
                .unwrap(),
            false
        );
        indexer.free_public_index(10).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .element_value_or_default(10)
                .unwrap(),
            false
        );
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            98
        );
    }

    #[test]
    fn test_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

        let n_indices = 10;
        for _i in 0..n_indices {
            indexer.new_public_index().unwrap();
        }

        indexer.free_public_index(0).unwrap();
        indexer.free_public_index(3).unwrap();
        indexer.free_public_index(4).unwrap();

        indexer.new_public_index().unwrap();

        assert_eq!(
            crate::graph::indexing::operations::GetValidIndices::valid_indices(&indexer).unwrap(),
            vec![0, 1, 2, 5, 6, 7, 8, 9]
        )
    }
}
