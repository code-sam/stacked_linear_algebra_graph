use std::mem;

use crate::graph::indexing::operations::in_memory_transaction::indexer_operations::{
    free_private_index, free_private_index_unchecked, free_public_index,
    free_public_index_unchecked, new_private_index, new_public_index, set_index_capacity,
};
// use crate::graph::indexing::indexer::operations::in_memory_transaction::indexer_operations::
use crate::graph::indexing::operations::{
    FreeIndex, GeneratePrivateIndex, GeneratePublicIndex, SetIndexCapacity,
};
use crate::graph::indexing::{AssignedIndex, ElementCount, Index};
use crate::operators::transaction::RestoreState;
use crate::{
    error::GraphComputingError, graph::indexing::Indexer,
    operators::transaction::UseAtomicTransaction,
};

use super::IndexerStateRestorer;

pub(crate) trait UseIndexerTransaction: UseAtomicTransaction {}

pub(crate) struct AtomicInMemoryIndexerTransaction<'a> {
    indexer: &'a mut Indexer,
    indexer_state_restorer: IndexerStateRestorer,
}

impl<'a> UseAtomicTransaction for AtomicInMemoryIndexerTransaction<'a> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        let reset_indexer_state_restorer =
            self.indexer_state_restorer.with_reset_state_to_restore();
        let indexer_state_restorer = mem::replace(
            &mut self.indexer_state_restorer,
            reset_indexer_state_restorer,
        );

        indexer_state_restorer.restore(&mut self.indexer)
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        self.indexer_state_restorer = IndexerStateRestorer::new_for_indexer(self.indexer)?;
        Ok(())
    }
}

impl<'t> Drop for AtomicInMemoryIndexerTransaction<'t> {
    fn drop(&mut self) {
        self.revert().unwrap();
    }
}

pub(in crate::graph::indexing::indexer::operations::in_memory_transaction) trait GetIndexerUnderTransaction
{
    fn indexer_ref(&self) -> &Indexer;
    fn indexer_mut_ref(&mut self) -> &mut Indexer;
}

pub(in crate::graph::indexing::indexer::operations::in_memory_transaction) trait GetIndexerStateRestorer
{
    fn indexer_state_restorer_ref(&self) -> &IndexerStateRestorer;
    fn indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer;
}

impl<'t> GetIndexerUnderTransaction for AtomicInMemoryIndexerTransaction<'t> {
    fn indexer_ref(&self) -> &Indexer {
        &self.indexer
    }

    fn indexer_mut_ref(&mut self) -> &mut Indexer {
        &mut self.indexer
    }
}

impl<'t> GetIndexerStateRestorer for AtomicInMemoryIndexerTransaction<'t> {
    fn indexer_state_restorer_ref(&self) -> &IndexerStateRestorer {
        &self.indexer_state_restorer
    }

    fn indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer {
        &mut self.indexer_state_restorer
    }
}

impl<'a> AtomicInMemoryIndexerTransaction<'a> {
    pub(crate) fn new(indexer: &'a mut Indexer) -> Result<Self, GraphComputingError> {
        let indexer_state_restorer = IndexerStateRestorer::new_for_indexer(indexer)?;
        Ok(Self {
            indexer,
            indexer_state_restorer,
        })
    }
}

impl<'t> FreeIndex for AtomicInMemoryIndexerTransaction<'t> {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_public_index(self.indexer, &mut self.indexer_state_restorer, index)
    }

    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_private_index(self.indexer, &mut self.indexer_state_restorer, index)
    }

    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_public_index_unchecked(self.indexer, &mut self.indexer_state_restorer, index)
    }

    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_private_index_unchecked(self.indexer, &mut self.indexer_state_restorer, index)
    }
}

impl<'a> GeneratePublicIndex for AtomicInMemoryIndexerTransaction<'a> {
    fn new_public_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        new_public_index(self.indexer, &mut self.indexer_state_restorer)
    }
}

impl<'a> GeneratePrivateIndex for AtomicInMemoryIndexerTransaction<'a> {
    fn new_private_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        new_private_index(self.indexer, &mut self.indexer_state_restorer)
    }
}

impl<'t> SetIndexCapacity for AtomicInMemoryIndexerTransaction<'t> {
    fn set_index_capacity(&mut self, capacity: ElementCount) -> Result<(), GraphComputingError> {
        set_index_capacity(self.indexer, &mut self.indexer_state_restorer, capacity)
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_use_successful_transaction() {
    //     let mut indexer =
    //         Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

    //     let transaction = AtomicInMemoryIndexerTransaction::new(&mut indexer).unwrap();

    //     let n_indices = 10;
    //     for _i in 0..n_indices {
    //         transaction.new_private_index().unwrap();
    //     }

    //     for _i in 0..n_indices {
    //         transaction.new_public_index().unwrap();
    //     }

    //     transaction.free_private_index(0).unwrap();
    //     transaction.free_private_index(3).unwrap();
    //     transaction.free_private_index(4).unwrap();

    //     transaction.free_public_index(10).unwrap();
    //     transaction.free_public_index(13).unwrap();
    //     transaction.free_public_index(14).unwrap();

    //     transaction.new_public_index().unwrap();
    //     transaction.new_private_index().unwrap();

    //     transaction.commit().unwrap();

    //     assert_eq!(
    //         crate::graph::indexing::operations::GetValidIndices::valid_indices(&indexer).unwrap(),
    //         vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 15, 16, 17, 18, 19]
    //     )
    // }
}
