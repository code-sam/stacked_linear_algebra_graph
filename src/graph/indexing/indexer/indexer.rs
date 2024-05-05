use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorLength, ResizeSparseVector, SetVectorElement,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::Collection;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::operators::apply::UnaryOperatorApplier;
use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, LogicalAnd, Minus};
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ElementWiseVectorAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseVectorMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ElementWiseVectorMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::index::{ElementCount, Index};
use crate::graph::indexing::AssignedIndex;

pub(crate) const MINIMUM_INDEXER_CAPACITY: usize = 1;

static ELEMENT_WISE_VECTOR_ADDITION_BINARY_OPERATOR: Lazy<ElementWiseVectorAdditionBinaryOperator> =
    Lazy::new(|| ElementWiseVectorAdditionBinaryOperator::new());

static ELEMENT_WISE_VECTOR_MULTIPLICATION_BINARY_OPERATOR: Lazy<
    ElementWiseVectorMultiplicationBinaryOperator,
> = Lazy::new(|| ElementWiseVectorMultiplicationBinaryOperator::new());

static UNARY_OPERATOR_APPLIER: Lazy<UnaryOperatorApplier> =
    Lazy::new(|| UnaryOperatorApplier::new());

static LOGICAL_AND_BOOL: Lazy<LogicalAnd<bool>> = Lazy::new(|| LogicalAnd::<bool>::new());

static MINUS_BOOL: Lazy<Minus<bool>> = Lazy::new(|| Minus::<bool>::new());

static ASSIGNMENT_OPERATOR_BOOL: Lazy<Assignment<bool>> = Lazy::new(|| Assignment::<bool>::new());

static DEFAULT_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

#[derive(Clone, Debug)]
pub(crate) struct Indexer {
    _graphblas_context: Arc<GraphBLASContext>,
    select_entire_vector: SelectEntireVector,

    indices_available_for_reuse: VecDeque<Index>,
    mask_with_valid_indices: SparseVector<bool>,
    mask_with_private_indices: SparseVector<bool>,

    // TODO: evaluate if caching, or updating on each change yields better performance
    mask_with_valid_private_indices: Option<SparseVector<bool>>,
    mask_with_public_indices: Option<SparseVector<bool>>,
    mask_with_valid_public_indices: Option<SparseVector<bool>>,
}

pub(super) trait GetIndicesAvailableForReuse {
    fn indices_available_for_reuse_ref(&self) -> &VecDeque<Index>;
    fn indices_available_for_reuse_mut_ref(&mut self) -> &mut VecDeque<Index>;
}

pub(super) trait GetIndexMask {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_valid_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_private_indices_ref(&self) -> &SparseVector<bool>;
    fn mask_with_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_valid_private_indices_ref(
        &mut self,
    ) -> Result<&SparseVector<bool>, GraphComputingError>;
    // fn mask_with_valid_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    // fn mask_with_public_indices_ref(&mut self) -> Result<&SparseVector<bool>, GraphComputingError>;
    // fn mask_with_public_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;

    fn mask_with_valid_public_indices_ref(
        &mut self,
    ) -> Result<&SparseVector<bool>, GraphComputingError>;
    // fn mask_with_valid_public_indices_mut_ref(&mut self) -> &mut SparseVector<bool>;
}

impl GetIndicesAvailableForReuse for Indexer {
    fn indices_available_for_reuse_ref(&self) -> &VecDeque<Index> {
        &self.indices_available_for_reuse
    }

    fn indices_available_for_reuse_mut_ref(&mut self) -> &mut VecDeque<Index> {
        &mut self.indices_available_for_reuse
    }
}

impl GetIndexMask for Indexer {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_valid_indices
    }

    fn mask_with_valid_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        self.invalidate_cached_masks();
        &mut self.mask_with_valid_indices
    }

    fn mask_with_private_indices_ref(&self) -> &SparseVector<bool> {
        &self.mask_with_private_indices
    }

    fn mask_with_private_indices_mut_ref(&mut self) -> &mut SparseVector<bool> {
        self.invalidate_cached_masks();
        &mut self.mask_with_private_indices
    }

    fn mask_with_valid_private_indices_ref(
        &mut self,
    ) -> Result<&SparseVector<bool>, GraphComputingError> {
        if self.mask_with_valid_private_indices.is_none() {
            let mut mask = SparseVector::<bool>::new(&self._graphblas_context, &self.capacity()?)?;

            ELEMENT_WISE_VECTOR_MULTIPLICATION_BINARY_OPERATOR.apply(
                self.mask_with_valid_indices_ref(),
                &*LOGICAL_AND_BOOL,
                self.mask_with_private_indices_ref(),
                &*ASSIGNMENT_OPERATOR_BOOL,
                &mut mask,
                &self.select_entire_vector,
                &*DEFAULT_OPERATOR_OPTIONS,
            )?;

            self.mask_with_valid_private_indices = Some(mask);
        }

        Ok(self.mask_with_valid_private_indices.as_ref().unwrap())
    }

    // fn mask_with_public_indices_ref(&mut self) -> Result<&SparseVector<bool>, GraphComputingError> {
    //     todo!()
    // }

    fn mask_with_valid_public_indices_ref(
        &mut self,
    ) -> Result<&SparseVector<bool>, GraphComputingError> {
        if self.mask_with_valid_private_indices.is_none() {
            let mut mask = SparseVector::<bool>::new(&self._graphblas_context, &self.capacity()?)?;

            ELEMENT_WISE_VECTOR_ADDITION_BINARY_OPERATOR.apply(
                self.mask_with_valid_indices_ref(),
                &*MINUS_BOOL,
                self.mask_with_private_indices_ref(),
                &*ASSIGNMENT_OPERATOR_BOOL,
                &mut mask,
                &self.select_entire_vector,
                &*DEFAULT_OPERATOR_OPTIONS,
            )?;

            self.mask_with_valid_private_indices = Some(mask);
        }

        Ok(self.mask_with_valid_private_indices.as_ref().unwrap())
    }
}

impl Indexer {
    // NOTE: setting and enforcing this minimum improves performance,
    // as the minimum is guaranteed once and no longer needs checking upon capacity expansion.
    // However, the API is slightly misleading for initial_capacity = 0.

    pub(crate) fn new(
        graphblas_context: &Arc<GraphBLASContext>,
    ) -> Result<Self, GraphComputingError> {
        let default_initial_capacity = 256;
        Self::with_initial_capacity(graphblas_context, &default_initial_capacity)
    }

    /// Sets a minimum capacity of 1, if initial_capacity = 0
    pub(crate) fn with_initial_capacity(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let initial_capacity = max(initial_capacity.clone(), MINIMUM_INDEXER_CAPACITY);

        Ok(Self {
            _graphblas_context: graphblas_context.clone(),
            select_entire_vector: SelectEntireVector::new(graphblas_context),
            indices_available_for_reuse: VecDeque::new(),
            mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
            mask_with_private_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
            mask_with_valid_private_indices: None,
            mask_with_public_indices: None,
            mask_with_valid_public_indices: None,
        })
    }

    pub(super) fn claim_available_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let available_index = match self.indices_available_for_reuse.pop_front() {
            None => self
                .mask_with_valid_indices_ref()
                .number_of_stored_elements()?,
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

        self.mask_with_valid_indices_mut_ref()
            .set_value(&available_index, true)?;

        Ok(new_index)
    }

    pub(super) fn expand_capacity(&mut self) -> Result<Index, GraphComputingError> {
        // TODO: test more sophisticated expansion sizing algorithms for better performance
        let new_capacity = self.capacity()? * 2;
        self.mask_with_valid_indices_mut_ref()
            .resize(new_capacity)?; // TODO: if this fails, state will be inconsistent
        self.mask_with_private_indices.resize(new_capacity)?;
        Ok(new_capacity)
    }

    // Method is implementation-specific, and therefore not part of the IndexerTrait
    pub(super) fn get_number_of_stored_and_reusable_elements(
        &self,
    ) -> Result<Index, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .number_of_stored_elements()?
            + self.indices_available_for_reuse.len())
    }

    pub(super) fn capacity(&self) -> Result<Index, GraphComputingError> {
        Ok(self.mask_with_valid_indices_ref().length()?)
    }

    fn invalidate_cached_masks(&mut self) {
        self.mask_with_valid_private_indices = None;
        self.mask_with_public_indices = None;
        self.mask_with_valid_public_indices = None;
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;

    use crate::graph::indexing::{
        operations::{CheckIndex, FreeIndex, GeneratePublicIndex, GetIndexerStatus},
        GetAssignedIndexData,
    };

    use super::*;

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

        let index = indexer.new_public_index().unwrap();
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

        indexer.free_valid_index(index.index_ref().clone()).unwrap();
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
            indices.push(indexer.new_public_index().unwrap());
        }

        indexer
            .free_valid_index(indices[2].index_ref().clone())
            .unwrap();
        indexer
            .free_valid_index(indices[20].index_ref().clone())
            .unwrap();
        indexer
            .free_valid_index(indices[92].index_ref().clone())
            .unwrap();

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
            indices.push(indexer.new_public_index().unwrap());
        }

        for _i in 0..20 {
            match indexer.free_valid_index(1) {
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
            indices.push(indexer.new_public_index().unwrap());
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
            indices.push(indexer.new_public_index().unwrap());
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

        indexer.free_valid_index(0).unwrap();
        assert_eq!(
            indexer
                .mask_with_valid_indices_ref()
                .element_value_or_default(&0)
                .unwrap(),
            false
        );
        indexer.free_valid_index(10).unwrap();
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
            indexer.new_public_index().unwrap();
        }

        indexer.free_valid_index(0).unwrap();
        indexer.free_valid_index(3).unwrap();
        indexer.free_valid_index(4).unwrap();

        indexer.new_public_index().unwrap();

        assert_eq!(
            crate::graph::indexing::operations::GetValidIndices::valid_indices(&indexer).unwrap(),
            vec![0, 1, 2, 5, 6, 7, 8, 9]
        )
    }
}
