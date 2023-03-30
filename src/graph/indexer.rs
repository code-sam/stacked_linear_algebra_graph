use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    GetVectorElementValue, SetVectorElement, SparseVector, SparseVectorTrait, VectorElement,
};
use graphblas_sparse_linear_algebra::collections::Collection;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::index::ElementIndex;
use hashbrown::HashMap;

use crate::error::{GraphComputingError, LogicError, LogicErrorType, UserError, UserErrorType};
use crate::graph::index::ElementCount;

pub type Index = ElementIndex;

pub type Key = String;
pub type KeyRef = str;

pub(crate) struct NewIndex {
    index: Index,
    new_index_capacity: Option<ElementCount>,
}

impl NewIndex {
    fn new(index: Index, new_index_capacity: Option<ElementCount>) -> Self {
        Self {
            index,
            new_index_capacity,
        }
    }
}

pub(crate) trait NewIndexTrait {
    fn index_ref(&self) -> &Index;
    fn new_index_capacity(&self) -> Option<ElementCount>;
}

impl NewIndexTrait for NewIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn new_index_capacity(&self) -> Option<ElementCount> {
        self.new_index_capacity
    }
}

pub(crate) trait IndexerTrait {
    fn add_new_key(&mut self, key: &KeyRef) -> Result<NewIndex, GraphComputingError>;
    fn add_or_replace_key(&mut self, key: &KeyRef) -> Result<NewIndex, GraphComputingError>;

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_key(&mut self, key: &KeyRef) -> Result<(), GraphComputingError>;

    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn is_valid_key(&self, key: &KeyRef) -> bool;

    fn index_for_key(&self, key: &KeyRef) -> Option<&Index>;
    // fn index_for_key_unchecked(&self, key: &KeyRef) -> &Index; // not useful in current implementation
    fn try_index_for_key(&self, key: &KeyRef) -> Result<&Index, GraphComputingError>;
    fn key_for_index(&self, index: &Index) -> Result<Key, GraphComputingError>;
    fn key_for_index_unchecked(&self, index: &Index) -> Key;
    // fn try_key_for_index(&self, index: &Index) -> Result<&KeyRef, GraphComputingError>;

    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError>;
    fn try_key_validity(&self, key: &KeyRef) -> Result<(), GraphComputingError>;

    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError>;
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
    key_to_index_map: HashMap<Key, Index>,
    index_to_key_map: Vec<Key>,
}

// TODO: probably, Indexer needs a generic type annotation, and then be implemented for IndexedDataStoreIndex
// TODO: drop type annotation altogether. Moving Index struct higher up towards the client would be better.
impl IndexerTrait for Indexer {
    fn add_or_replace_key(&mut self, key: &KeyRef) -> Result<NewIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        match self
            .key_to_index_map
            .insert(key.to_owned(), *index.index_ref())
        {
            Some(superseded_index) => {
                self.mask_with_valid_indices
                    .drop_element(superseded_index.clone())?;
                // self.index_to_key_map[index] = String::from("INVALID_INDEX");
                self.indices_available_for_reuse.push_back(superseded_index);
            }
            None => {}
        }

        self.update_index_to_key_map(*index.index_ref(), key);

        Ok(index)
    }

    fn add_new_key(&mut self, key: &KeyRef) -> Result<NewIndex, GraphComputingError> {
        let index = self.claim_available_index()?;
        match self
            .key_to_index_map
            .insert(key.to_owned(), *index.index_ref())
        {
            Some(superseded_index) => {
                // roll-back
                self.key_to_index_map
                    .insert(key.to_owned(), superseded_index);
                return Err(LogicError::new(
                    LogicErrorType::KeyAlreadyExists,
                    format!("Key \"{}\" is already in use", key),
                    None,
                )
                .into());
            }
            None => {}
        }

        self.update_index_to_key_map(*index.index_ref(), key);

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

    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.key_to_index_map
            .remove(self.index_to_key_map[index].as_str());
        self.mask_with_valid_indices.drop_element(index.clone())?;
        // self.index_to_key_map[index] = String::from("INVALID_INDEX");
        self.indices_available_for_reuse.push_back(index);
        Ok(())
    }

    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_key(&mut self, key: &KeyRef) -> Result<(), GraphComputingError> {
        let index = self.try_index_for_key(key)?.clone();
        self.key_to_index_map.remove(key);
        self.mask_with_valid_indices.drop_element(index)?;
        // self.index_to_key_map[index] = String::from("INVALID_INDEX");
        self.indices_available_for_reuse.push_back(index.clone());
        Ok(())
    }

    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .get_element_value_or_default(index)?)
    }

    fn is_valid_key(&self, key: &KeyRef) -> bool {
        self.key_to_index_map.contains_key(key)
    }

    fn index_for_key(&self, key: &KeyRef) -> Option<&Index> {
        self.key_to_index_map.get(key)
    }

    fn try_index_for_key(&self, key: &KeyRef) -> Result<&Index, GraphComputingError> {
        match self.index_for_key(key) {
            Some(index_ref) => Ok(index_ref),
            None => {
                return Err(LogicError::new(
                    LogicErrorType::InvalidKey,
                    format!("Unknown key: {}", key),
                    None,
                )
                .into())
            }
        }
    }

    fn key_for_index(&self, index: &Index) -> Result<Key, GraphComputingError> {
        match self.is_valid_index(index)? {
            true => Ok(self.key_for_index_unchecked(index)),
            false => {
                return Err(LogicError::new(
                    LogicErrorType::InvalidIndex,
                    format!("Invalid index: {}, the index may have been freed", index),
                    None,
                )
                .into())
            }
        }
    }

    fn key_for_index_unchecked(&self, index: &Index) -> Key {
        self.index_to_key_map[*index].clone()
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

    fn number_of_indexed_elements(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices.number_of_stored_elements()?)
    }

    fn try_key_validity(&self, key: &KeyRef) -> Result<(), GraphComputingError> {
        todo!()
    }
}

impl Indexer {
    pub fn new(graphblas_context: &Arc<GraphBLASContext>) -> Result<Self, GraphComputingError> {
        let default_initial_capacity = 256;
        Self::with_initial_capacity(graphblas_context, &default_initial_capacity)
    }

    /// Sets a minimum capacity of 1, if initial_capacity = 0
    pub fn with_initial_capacity(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        // NOTE: setting and enforcing this minimum improves performance,
        // as the minimum is guaranteed once and no longer needs checking upon capacity expansion.
        // However, the API is slightly misleading for initial_capacity = 0.
        let minimum_initial_capacity = 1;
        let initial_capacity = max(initial_capacity.clone(), minimum_initial_capacity);

        let mut key_to_index_map: HashMap<Key, Index> = HashMap::default();
        key_to_index_map.reserve(initial_capacity);

        Ok(Self {
            _graphblas_context: graphblas_context.clone(),
            indices_available_for_reuse: VecDeque::new(),
            mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
            key_to_index_map,
            index_to_key_map: Vec::with_capacity(initial_capacity),
        })
    }

    fn claim_available_index(&mut self) -> Result<NewIndex, GraphComputingError> {
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
            new_index = NewIndex::new(available_index, Some(new_capacity));
        } else {
            new_index = NewIndex::new(available_index, None);
        }

        self.mask_with_valid_indices
            .set_element(VectorElement::from_pair(available_index, true))?;

        Ok(new_index)
    }

    fn expand_capacity(&mut self) -> Result<Index, GraphComputingError> {
        // TODO: test more sophisticated expansion sizing algorithms for better performance
        self.index_to_key_map
            .try_reserve(self.index_to_key_map.capacity())?;
        let new_capacity = self.index_to_key_map.capacity();
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

    fn update_index_to_key_map(&mut self, index: usize, key: &str) {
        // REVIEW: would using a HashMap result in better performance?
        if index == self.index_to_key_map.len() {
            self.index_to_key_map.push(key.to_owned());
        } else {
            self.index_to_key_map[index] = key.to_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::collections::sparse_vector::GetVectorElementValue;
    use graphblas_sparse_linear_algebra::context::Mode as GraphBLASMode;

    #[test]
    fn new_indexer() {
        let initial_capacity = 10;
        let mut indexer = Indexer::with_initial_capacity(
            &GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
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

        let index = indexer.add_new_key("key1").unwrap();
        let mask_with_valid_indices = indexer.mask_with_valid_indices_ref();

        assert_eq!(
            indexer
                .get_number_of_stored_and_reusable_elements()
                .unwrap(),
            1
        );
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 1);
        assert_eq!(indexer.is_valid_index(&index.index_ref()).unwrap(), true);
        assert_eq!(indexer.is_valid_key("key1"), true);

        assert_eq!(
            mask_with_valid_indices.number_of_stored_elements().unwrap(),
            1
        );
        assert_eq!(mask_with_valid_indices.length().unwrap(), initial_capacity);
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(&index.index_ref())
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
                .get_element_value(&index.index_ref())
                .unwrap(),
            None
        );
    }

    #[test]
    fn new_store_with_zero_capacity() {
        let mut indexer = Indexer::with_initial_capacity(
            &GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
            &0,
        )
        .unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for i in 0..n_indices {
            indices.push(indexer.add_new_key(format!("{}", i).as_str()).unwrap());
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
                .get_element_value(&indices[33].index_ref())
                .unwrap(),
            Some(true)
        );
        assert_eq!(
            mask_with_valid_indices
                .get_element_value(&indices[20].index_ref())
                .unwrap(),
            None
        );
    }

    #[test]
    fn delete_same_key_multiple_times() {
        let mut indexer = Indexer::with_initial_capacity(
            &GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
            &10,
        )
        .unwrap();

        let mut indices = Vec::new();
        let n_indices = 10;
        for i in 0..n_indices {
            indices.push(indexer.add_new_key(format!("{}", i).as_str()).unwrap());
        }

        for i in 0..20 {
            indexer.free_key("1");
        }
        for i in 0..20 {
            indexer.free_index(1);
        }

        assert!(!indexer.is_valid_key("1"));
        assert!(!indexer.is_valid_index(&1).unwrap());
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()
                .unwrap(),
            9
        );

        for i in 0..n_indices {
            indices.push(
                indexer
                    .add_or_replace_key(format!("{}", i).as_str())
                    .unwrap(),
            );
        }

        assert_eq!(indexer.key_for_index(&1).unwrap(), "0");
        assert_eq!(indexer.index_for_key("5"), Some(&4));
        assert_eq!(indexer.key_for_index(&0).unwrap(), "1");
        assert_eq!(indexer.number_of_indexed_elements().unwrap(), 10);
    }

    #[test]
    fn test_mask_with_valid_indices() {
        let mut indexer = Indexer::with_initial_capacity(
            &GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
            &0,
        )
        .unwrap();

        let mut indices = Vec::new();
        let n_indices = 100;
        for i in 0..n_indices {
            indices.push(indexer.add_new_key(format!("{}", i).as_str()).unwrap());
            assert_eq!(
                indexer
                    .mask_with_valid_indices_ref()
                    .get_element_value_or_default(&i)
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
                .get_element_value_or_default(&0)
                .unwrap(),
            false
        );
        indexer.free_index(10).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .get_element_value_or_default(&10)
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
}
