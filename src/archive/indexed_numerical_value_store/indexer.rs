use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
    GetVectorElementValue, SetVectorElement, SparseVector, VectorElement,
};

use super::index::{Index, IndexTrait, IndexedDataStoreIndex};
use crate::error::{GraphComputingError, LogicError, LogicErrorType};

// + Debug required for free(). Can this additional bound be added to the method itself?
pub(crate) trait IndexerTrait<I: IndexTrait + Debug> {
    fn claim_available_index(&mut self) -> Result<I, GraphComputingError>;

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free(&mut self, index: I) -> Result<(), GraphComputingError>;
    fn is_valid_index(&self, index: &I) -> Result<bool, GraphComputingError>;
    fn check_index_validity(&self, index: &I) -> Result<(), GraphComputingError>;
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError>;
}

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
impl IndexerTrait<IndexedDataStoreIndex> for Indexer {
    fn claim_available_index(&mut self) -> Result<IndexedDataStoreIndex, GraphComputingError> {
        let available_index = match self.indices_available_for_reuse.pop_front() {
            None => self.mask_with_valid_indices.number_of_stored_elements()?,
            Some(index) => index,
        };

        // new indices are popped from a stack. Indices of freed indices are pushed to the stack, and re-used.
        // benefit: no memory is allocated for unused indices
        // downside: runtime cost; more complexity; no use of speedy pre-allocation; memory is never deallocated
        // let data = self.get_write_locked_data()?;
        if available_index >= self.capacity()? {
            self.expand_capacity()?;
        }
        self.mask_with_valid_indices
            .set_element(VectorElement::from_pair(available_index, true))?;
        Ok(IndexedDataStoreIndex::new(available_index))
    }

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free(&mut self, index: IndexedDataStoreIndex) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices
            .drop_element(index.index_ref().clone())?;
        self.indices_available_for_reuse.push_back(index.index());
        Ok(())
    }

    fn is_valid_index(&self, index: &IndexedDataStoreIndex) -> Result<bool, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .get_element_value(index.index_ref())?)
    }

    fn check_index_validity(
        &self,
        index: &IndexedDataStoreIndex,
    ) -> Result<(), GraphComputingError> {
        if self.is_valid_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No valid value at index [{}], the value may have been freed.",
                    index.index_ref()
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

    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?)
    }
}

impl Indexer {
    pub(crate) fn new(
        graphblas_context: Arc<GraphBLASContext>,
    ) -> Result<Self, GraphComputingError> {
        let default_initial_capacity = 256;
        Self::with_initial_capacity(&default_initial_capacity, graphblas_context)
    }

    /// Sets a minimum capacity of 1, if initial_capacity = 0
    pub(crate) fn with_initial_capacity(
        initial_capacity: &Index,
        graphblas_context: Arc<GraphBLASContext>,
    ) -> Result<Self, GraphComputingError> {
        // NOTE: setting and enforcing this minimum improves performance,
        // as the minimum is guaranteed once and no longer needs checkungupon capacity expansion.
        // However, the API is slightly misleading for initial_capacity = 0.
        let minimum_initial_capacity = 1;
        let initial_capacity = std::cmp::max(initial_capacity.clone(), minimum_initial_capacity);
        Ok(Self {
            _graphblas_context: graphblas_context.clone(),
            indices_available_for_reuse: VecDeque::new(),
            mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
        })
    }

    fn expand_capacity(&mut self) -> Result<Index, GraphComputingError> {
        // TODO: test more sophisticated expansion sizing algorithms for better performance
        let new_capacity = self.capacity()? * 2;
        self.mask_with_valid_indices.resize(new_capacity)?;
        Ok(new_capacity)
    }

    // Method is implementation-specific, and therefore not part of the IndexerTrait
    fn get_number_of_stored_and_reusable_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?
            + self.indices_available_for_reuse.len())
    }

    // includes freed elements
    // pub(crate) fn get_number_stored_elements(&self) -> Index {
    //     self.data.len()
    // }

    // Method is implementation-specific, and therefore not part of the IndexerTrait
    fn capacity(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.length()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::context::Mode as GraphBLASMode;
    use graphblas_sparse_linear_algebra::value_types::sparse_vector::GetVectorElementValue;

    #[test]
    fn new_indexer() {
        let initial_capacity = 10;
        let mut indexer = Indexer::with_initial_capacity(
            &10,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            0
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);

        let index = indexer.claim_available_index().unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 1);
        assert_eq!(indexer.is_valid_index(&index).unwrap(), true);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            1
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(index.index_ref())
                .unwrap(),
            true
        );

        indexer.free(index.clone()).unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 0);
        assert_eq!(indexer.is_valid_index(&index).unwrap(), false);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            0
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );
    }

    #[test]
    fn new_store_with_zero_capacity() {
        let initial_capacity = 2;
        let mut indexer = Indexer::with_initial_capacity(
            &10,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        let mut indices = Vec::new();
        let nIndices = 100;
        for i in 0..nIndices {
            indices.push(indexer.claim_available_index().unwrap());
        }

        indexer.free(indices[2].clone()).unwrap();
        indexer.free(indices[20].clone()).unwrap();
        indexer.free(indices[92].clone()).unwrap();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            nIndices
        );
        assert_eq!(indexer.is_valid_index(&indices[0]).unwrap(), true);
        assert_eq!(indexer.is_valid_index(&indices[10]).unwrap(), true);
        assert_eq!(indexer.is_valid_index(&indices[33]).unwrap(), true);
        assert_eq!(indexer.is_valid_index(&indices[77]).unwrap(), true);
        assert_eq!(indexer.is_valid_index(&indices[99]).unwrap(), true);
        assert_eq!(indexer.is_valid_index(&indices[2]).unwrap(), false);
        assert_eq!(indexer.is_valid_index(&indices[20]).unwrap(), false);
        assert_eq!(indexer.is_valid_index(&indices[92]).unwrap(), false);

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            nIndices
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), nIndices - 3);

        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            nIndices - 3
        );
        // assert_eq!(
        //     mask_with_valid_indices
        //         .length()
        //         .unwrap(),
        //     initial_capacity
        // );
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(indices[33].index_ref())
                .unwrap(),
            true
        );
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(indices[20].index_ref())
                .unwrap(),
            false
        );
    }
}
