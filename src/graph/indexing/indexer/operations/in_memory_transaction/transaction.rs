use std::collections::VecDeque;

use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{ElementCount, Index, Indexer},
        value_type::ValueType,
    },
    operators::{
        in_memory_transaction::transaction::{QueueStateReverter, SparseVectorStateReverter},
        transaction::UseAtomicTransaction,
    },
};

struct IndexerTransactionReverter {
    index_capacity_to_restore: ElementCount,

    indices_available_for_reuse_restorer: QueueStateReverter<Index>,

    mask_with_valid_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_private_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_valid_private_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_valid_public_indices_restorer: SparseVectorStateReverter<bool>,
}

pub(crate) struct AtomicInMemoryIndexerTransaction<'a> {
    indexer: &'a mut Indexer,

    indices_available_for_reuse: VecDeque<Index>,

    mask_with_valid_indices: SparseVector<bool>,
    mask_with_private_indices: SparseVector<bool>,
    mask_with_valid_private_indices: SparseVector<bool>,
    mask_with_valid_public_indices: SparseVector<bool>,
}

impl<'a> UseAtomicTransaction for AtomicInMemoryIndexerTransaction<'a> {
    fn revert(self) -> Result<(), crate::error::GraphComputingError> {
        todo!()
    }

    fn commit(self) -> Result<(), crate::error::GraphComputingError> {
        todo!()
    }
}

impl<'a> AtomicInMemoryIndexerTransaction<'a> {
    pub(crate) fn new(indexer: &'a mut Indexer) -> Self {
        todo!()
        // Self { indexer }
    }

    fn revert_private(&mut self) -> Result<(), GraphComputingError> {
        todo!()
    }
}
