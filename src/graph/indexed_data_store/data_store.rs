use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use rayon::prelude::*;

use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
    GetVectorElementValue, SetVectorElement, SparseVector, VectorElement,
};

use super::index::{Index, IndexTrait, IndexedDataStoreIndex};
use crate::error::{GraphComputingError, LogicError, LogicErrorType};

pub(crate) trait IndexedDataStoreTrait {}

// Note: benchmark before allowing parallel operations on self.data
// up to 2x better read performance than HashMap (approaching Vec)
// about 5x better write performance than Hashmap (approaching Vec)
// pub struct IndexedDataStore<T: Copy> {
#[derive(Clone, Debug)]
pub(crate) struct IndexedDataStore<T>
where
    T: Send + Sync,
{
    data: Vec<T>,
    indices_available_for_reuse: VecDeque<Index>,

    _graphblas_context: Arc<GraphBLASContext>,
    mask_with_valid_indices: SparseVector<bool>,
}

impl<T: Send + Sync> IndexedDataStoreTrait for IndexedDataStore<T> {}

impl<T: Send + Sync> IndexedDataStore<T> {
    pub(crate) fn with_capacity(
        initial_capacity: Index,
        graphblas_context: Arc<GraphBLASContext>,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            // data: RwLock::new(Vec::with_capacity(*initial_capacity)),
            data: Vec::with_capacity(initial_capacity),
            indices_available_for_reuse: VecDeque::new(),

            _graphblas_context: graphblas_context.clone(),
            mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
        })
    }

    pub(crate) fn push(
        &mut self,
        data_to_push: T,
    ) -> Result<IndexedDataStoreIndex, GraphComputingError> {
        let available_index = self.get_available_index()?;

        // new indices are popped from a stack. Indices of freed indices are pushed to the stack, and re-used.
        // benefit: no memory is allocated for unused indices
        // downside: runtime cost; more complexity; no use of speedy pre-allocation; memory is never deallocated
        // let data = self.get_write_locked_data()?;
        if available_index < self.data.len() {
            self.mask_with_valid_indices
                .set_element(VectorElement::from_pair(available_index, true))?;
            self.data[available_index] = data_to_push;
        } else {
            if available_index < self.mask_with_valid_indices.length()? {
                self.mask_with_valid_indices
                    .set_element(VectorElement::from_pair(available_index, true))?;
                self.data.push(data_to_push);
            } else {
                self.data.push(data_to_push);
                match self.mask_with_valid_indices.resize(self.data.capacity()) {
                    Ok(_) => {
                        match self
                            .mask_with_valid_indices
                            .set_element(VectorElement::from_pair(available_index, true))
                        {
                            Ok(_) => (),
                            Err(error) => {
                                self.data.pop();
                                return Err(error.into());
                            }
                        }
                    }
                    Err(error) => {
                        self.data.pop();
                        return Err(error.into());
                    }
                }
            }
        }
        return Ok(IndexedDataStoreIndex::new(available_index));
    }

    pub(crate) fn get_ref<I: IndexTrait>(&self, index: I) -> Result<&T, GraphComputingError> {
        // #[cfg(debug_assertions)] // TODO: review performance cost of checking the validity of the index
        self.check_index(&index)?;

        Ok(&self.data[index.index()])
    }

    pub(crate) fn get_mut_ref<I: IndexTrait>(
        &mut self,
        index: I,
    ) -> Result<&mut T, GraphComputingError> {
        // #[cfg(debug_assertions)]
        self.check_index(&index)?;

        Ok(&mut self.data[index.index()])
    }

    pub(crate) fn is_valid_index<I: IndexTrait>(
        &self,
        index: &I,
    ) -> Result<bool, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .get_element_value(index.index_ref())?)
    }

    pub(crate) fn check_index<I: IndexTrait>(&self, index: &I) -> Result<(), GraphComputingError> {
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
    pub(crate) fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_indices
    }

    /// Apply function to all stored elements
    pub(crate) fn map_mut_all<F>(&mut self, function_to_apply: F) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut T) -> Result<(), GraphComputingError> + Send + Sync,
    {
        let result: Vec<_> = self
            .data
            .as_mut_slice()
            .into_par_iter()
            .map(function_to_apply)
            .collect();
        for result in result.into_iter() {
            // TODO: consider parallelization
            result?
        }
        Ok(()) // TODO: check result vector
    }

    pub(crate) fn update<I: IndexTrait>(
        &mut self,
        index: I,
        data_to_set: T,
    ) -> Result<(), GraphComputingError> {
        // #[cfg(debug_assertions)]
        self.check_index(&index)?;

        self.data[index.index()] = data_to_set;
        Ok(())
    }

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    pub(crate) fn free<I: IndexTrait + Debug>(
        &mut self,
        index: I,
    ) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices
            .drop_element(index.index_ref().clone())?;
        self.indices_available_for_reuse.push_back(index.index());
        Ok(())
    }

    pub(crate) fn get_number_of_indexed_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?)
    }

    // includes freed elements
    // pub(crate) fn get_number_stored_elements(&self) -> Index {
    //     self.data.len()
    // }

    pub(crate) fn get_number_of_stored_and_reusable_elements(
        &self,
    ) -> Result<Index, GraphComputingError> {
        // Ok(self.get_read_locked_data()?.len())
        Ok(self.data.len())
    }

    pub(crate) fn get_capacity(&self) -> Result<Index, GraphComputingError> {
        // Ok(self.get_read_locked_data()?.capacity())
        Ok(self.data.capacity())
    }

    pub(crate) fn get_available_index(&mut self) -> Result<Index, GraphComputingError> {
        match self.indices_available_for_reuse.pop_front() {
            None => self.get_number_of_stored_and_reusable_elements(),
            Some(index) => Ok(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::AddAssign;

    use graphblas_sparse_linear_algebra::context::Mode as GraphBLASMode;
    use graphblas_sparse_linear_algebra::value_types::sparse_vector::GetVectorElementValue;

    #[test]
    fn new_store() {
        let mut store = IndexedDataStore::<i32>::with_capacity(
            10,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        let value = 1;
        let index = store.push(value.clone()).unwrap();
        assert_eq!(store.get_ref(index).unwrap(), &value);

        let another_value = 2;
        let another_index = store.push(another_value.clone()).unwrap();
        assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
        assert_eq!(store.get_ref(index).unwrap(), &value)
    }

    #[test]
    fn new_store_with_zero_capacity() {
        let mut store = IndexedDataStore::<i32>::with_capacity(
            0,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            0
        );
        assert_eq!(store.mask_with_valid_indices_ref().length().unwrap(), 0);

        let value = 1;
        let index = store.push(value.clone()).unwrap();
        assert_eq!(store.get_ref(index).unwrap(), &value);
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            1
        );

        let another_value = 2;
        let another_index = store.push(another_value.clone()).unwrap();
        assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
        assert_eq!(store.get_ref(index).unwrap(), &value);
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            2
        );

        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .get_element_value(&0)
                .unwrap(),
            true
        );
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .get_element_value(&1)
                .unwrap(),
            true
        );
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .get_element_value(&2)
                .unwrap(),
            false
        );
    }

    #[test]
    fn store_expansion() {
        let mut store = IndexedDataStore::<i32>::with_capacity(
            1,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        let value = 1;
        let index = store.push(value.clone()).unwrap();
        assert_eq!(store.get_ref(index).unwrap(), &value);

        let another_value = 2;
        let another_index = store.push(another_value.clone()).unwrap();
        assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
        assert_eq!(store.get_ref(index).unwrap(), &value);

        for test_value in 3..1000 {
            let test_index = store.push(test_value.clone()).unwrap();
            assert_eq!(store.get_ref(test_index).unwrap(), &test_value);
            assert_eq!(store.get_ref(index).unwrap(), &value)
        }

        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &1);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &3);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &4);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &6);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &31);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &100);
    }

    #[test]
    fn free_data() {
        let mut store = IndexedDataStore::<i32>::with_capacity(
            1,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        let value = 1;
        let index = store.push(value.clone()).unwrap();
        assert_eq!(store.get_ref(index).unwrap(), &value);
        assert_eq!(index, IndexedDataStoreIndex::new(0));

        let another_value = 2;
        let another_index = store.push(another_value.clone()).unwrap();
        assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
        assert_eq!(store.get_ref(index).unwrap(), &value);
        assert_eq!(another_index, IndexedDataStoreIndex::new(1));

        store.free(index.clone()).unwrap();
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            1
        );
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );
        assert_eq!(
            store
                .mask_with_valid_indices_ref()
                .get_element_value(another_index.index_ref())
                .unwrap(),
            true
        );

        let value_after_free = 3;
        let index_after_free = store.push(value_after_free.clone()).unwrap();
        assert_eq!(store.get_ref(index_after_free).unwrap(), &value_after_free);
        assert_eq!(store.get_ref(index).unwrap(), &value_after_free);
        assert_eq!(index_after_free, IndexedDataStoreIndex::new(0));

        for test_value in 2..100 {
            let test_index = store.push(test_value.clone()).unwrap();
            assert_eq!(store.get_ref(test_index).unwrap(), &test_value);
        }

        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &3);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &2);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &3);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &5);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &30);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &99);
    }

    #[test]
    fn test_map_mut_all() {
        let mut store = IndexedDataStore::<i32>::with_capacity(
            1,
            GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
        )
        .unwrap();

        for test_value in 0..100 {
            store.push(test_value.clone()).unwrap();
        }

        let add_one = |value: &mut i32| -> Result<(), GraphComputingError> {
            value.add_assign(1);
            Ok(())
        };

        store.map_mut_all(add_one).unwrap();

        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &1);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &3);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &4);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &6);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &31);
        assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &100);
    }
}
