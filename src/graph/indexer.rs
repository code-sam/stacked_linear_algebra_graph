use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    DeleteSparseVectorElement, GetSparseVectorLength, GetVectorElementIndices,
    GetVectorElementValue, ResizeSparseVector, SetVectorElement,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::Collection;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::index::ElementIndex;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::index::ElementCount;

pub type Index = ElementIndex;

pub(crate) const MINIMUM_INDEXER_CAPACITY: usize = 1;

#[derive(Debug)]
pub(crate) struct AssignedIndex {
    index: Index,
    new_index_capacity: Option<ElementCount>,
}

impl AssignedIndex {
    fn new(index: Index, new_index_capacity: Option<ElementCount>) -> Self {
        Self {
            index,
            new_index_capacity,
        }
    }
}

pub(crate) trait GetAssignedIndexData {
    fn index(&self) -> Index;
    fn index_ref(&self) -> &Index;
    fn new_index_capacity(&self) -> Option<ElementCount>;
}

impl GetAssignedIndexData for AssignedIndex {
    fn index(&self) -> Index {
        self.index.to_owned()
    }

    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn new_index_capacity(&self) -> Option<ElementCount> {
        self.new_index_capacity
    }
}

pub(crate) trait IndexerTrait {
    fn new_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;

    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError>;

    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError>;
    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}

#[derive(Clone, Debug)]
pub(crate) struct Indexer {
    _graphblas_context: Arc<GraphBLASContext>,
    // _is_index_or_available_for_reuse keeps the size of the index vector and it's automatic resizing
    // TODO: Is there a less memory intensive and cheaper method?
    // Such as keeping an integer index with the current capacity?
    // is_index_or_available_for_reuse: Vec<bool>,
    indices_available_for_reuse: VecDeque<Index>,
    mask_with_valid_indices: SparseVector<bool>,
}

// TODO: probably, Indexer needs a generic type annotation, and then be implemented for IndexedDataStoreIndex
// TODO: drop type annotation altogether. Moving Index struct higher up towards the client would be better.
impl IndexerTrait for Indexer {
    fn new_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        Ok(index)
    }

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        if self.is_valid_index(&index)? {
            self.free_index_unchecked(index)
        } else {
            Ok(())
        }
    }

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices.drop_element(index.clone())?;
        self.indices_available_for_reuse.push_back(index);
        Ok(())
    }

    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .element_value_or_default(index)?)
    }

    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError> {
        if self.is_valid_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No valid value at index [{}], the value may have been freed.",
                    index
                ),
                None,
            )
            .into());
        }
    }

    // The mask is updated at each push() and free() operation.
    // benefit: mask is pre-computed, resulting in faster query operations
    // downside: slower push() and free() operations
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_indices
    }

    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        // self.key_to_index_map.values().into_iter().collect()
        Ok(self.mask_with_valid_indices.element_indices()?)
    }

    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?)
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.mask_with_valid_indices.length()?)
    }
}

impl Indexer {
    // NOTE: setting and enforcing this minimum improves performance,
    // as the minimum is guaranteed once and no longer needs checking upon capacity expansion.
    // However, the API is slightly misleading for initial_capacity = 0.

    pub fn new(graphblas_context: &Arc<GraphBLASContext>) -> Result<Self, GraphComputingError> {
        let default_initial_capacity = 256;
        Self::with_initial_capacity(graphblas_context, &default_initial_capacity)
    }

    /// Sets a minimum capacity of 1, if initial_capacity = 0
    pub fn with_initial_capacity(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let initial_capacity = max(initial_capacity.clone(), MINIMUM_INDEXER_CAPACITY);

        Ok(Self {
            _graphblas_context: graphblas_context.clone(),
            indices_available_for_reuse: VecDeque::new(),
            mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
        })
    }

    fn claim_available_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let available_index = match self.indices_available_for_reuse.pop_front() {
            None => self.mask_with_valid_indices.number_of_stored_elements()?,
            Some(index) => index,
        };

        // new indices are popped from a stack. Indices of freed indices are pushed to the stack, and re-used.
        // benefit: no memory is allocated for unused indices
        // downside: runtime cost; more complexity; no use of speedy pre-allocation; memory is never deallocated
        let new_index;
        if available_index >= self.capacity()? {
            let new_capacity = self.expand_capacity()?;
            new_index = AssignedIndex::new(available_index, Some(new_capacity));
        } else {
            new_index = AssignedIndex::new(available_index, None);
        }

        self.mask_with_valid_indices
            .set_value(&available_index, true)?;

        Ok(new_index)
    }

    fn expand_capacity(&mut self) -> Result<Index, GraphComputingError> {
        // TODO: test more sophisticated expansion sizing algorithms for better performance
        let new_capacity = self.capacity()? * 2;
        self.mask_with_valid_indices.resize(new_capacity)?; // TODO: if this fails, state will be inconsistent
        Ok(new_capacity)
    }

    // Method is implementation-specific, and therefore not part of the IndexerTrait
    fn get_number_of_stored_and_reusable_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?
            + self.indices_available_for_reuse.len())
    }

    fn capacity(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.length()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::context::Mode as GraphBLASMode;

    #[test]
    fn new_indexer() {
        let initial_capacity = 10;
        let mut indexer = Indexer::with_initial_capacity(
            &GraphBLASContext::init_default().unwrap(),
            &initial_capacity,
        )
        .unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            0
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);

        let index = indexer.new_index().unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 1);
        assert_eq!(indexer.is_valid_index(&index.index_ref()).unwrap(), true);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            1
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .element_value(&index.index_ref())
                .unwrap(),
            Some(true)
        );

        indexer.free_index(index.index_ref().clone()).unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);
        assert_eq!(indexer.is_valid_index(&index.index_ref()).unwrap(), false);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            0
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .element_value(&index.index_ref())
                .unwrap(),
            None
        );
    }

    #[test]
    fn new_store_with_zero_capacity() {
        let mut indexer =
            Indexer::with_initial_capacity(&GraphBLASContext::init_default().unwrap(), &0).unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for _i in 0..n_indices {
            indices.push(indexer.new_index().unwrap());
        }

        indexer.free_index(indices[2].index_ref().clone()).unwrap();
        indexer.free_index(indices[20].index_ref().clone()).unwrap();
        indexer.free_index(indices[92].index_ref().clone()).unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            n_indices
        );
        assert_eq!(
            indexer.is_valid_index(&indices[0].index_ref()).unwrap(),
            true
        );
        assert_eq!(
            indexer.is_valid_index(&indices[10].index_ref()).unwrap(),
            true
        );
        assert_eq!(
            indexer.is_valid_index(&indices[33].index_ref()).unwrap(),
            true
        );
        assert_eq!(
            indexer.is_valid_index(&indices[77].index_ref()).unwrap(),
            true
        );
        assert_eq!(
            indexer.is_valid_index(&indices[99].index_ref()).unwrap(),
            true
        );
        assert_eq!(
            indexer.is_valid_index(&indices[2].index_ref()).unwrap(),
            false
        );
        assert_eq!(
            indexer.is_valid_index(&indices[20].index_ref()).unwrap(),
            false
        );
        assert_eq!(
            indexer.is_valid_index(&indices[92].index_ref()).unwrap(),
            false
        );

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
                .element_value(&indices[33].index_ref())
                .unwrap(),
            Some(true)
        );
        assert_eq!(
            mask_with_valid_indices
                .element_value(&indices[20].index_ref())
                .unwrap(),
            None
        );
    }

    #[test]
    fn delete_same_key_multiple_times() {
        let mut indexer =
            Indexer::with_initial_capacity(&GraphBLASContext::init_default().unwrap(), &10)
                .unwrap();

        let mut indices = Vec::new();
        let n_indices = 10;
        for _i in 0..n_indices {
            indices.push(indexer.new_index().unwrap());
        }

        for _i in 0..20 {
            match indexer.free_index(1) {
                // deleting the same key multiple times will result in errors, this error is not tested.
                Ok(_) => (),
                Err(_) => (),
            }
        }

        assert!(!indexer.is_valid_index(&1).unwrap());
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            9
        );

        for _i in 0..n_indices {
            indices.push(indexer.new_index().unwrap());
        }

        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 19);
    }

    #[test]
    fn test_mask_with_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(&GraphBLASContext::init_default().unwrap(), &0).unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for i in 0..n_indices {
            indices.push(indexer.new_index().unwrap());
            assert_eq!(
                indexer
                    .mask_with_valid_indices_ref()
                    .element_value_or_default(&i)
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

        indexer.free_index(0).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .element_value_or_default(&0)
                .unwrap(),
            false
        );
        indexer.free_index(10).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .element_value_or_default(&10)
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
            Indexer::with_initial_capacity(&GraphBLASContext::init_default().unwrap(), &0).unwrap();

        let n_indices = 10;
        for _i in 0..n_indices {
            indexer.new_index().unwrap();
        }

        indexer.free_index(0).unwrap();
        indexer.free_index(3).unwrap();
        indexer.free_index(4).unwrap();

        indexer.new_index().unwrap();

        assert_eq!(
            indexer.valid_indices().unwrap(),
            vec![0, 1, 2, 5, 6, 7, 8, 9]
        )
    }
}
