use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::Index;
use crate::error::GraphComputingError;

use super::{AtomicInMemoryIndexerTransaction, GetIndexerUnderTransaction};

impl<'t> CheckIndex for AtomicInMemoryIndexerTransaction<'t> {
    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_index(index)
    }

    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_index_validity(index)
    }

    fn is_valid_private_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_private_index(index)
    }

    fn try_is_valid_private_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_public_index(index)
    }

    fn is_public_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_public_index(index)
    }

    fn try_is_public_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_public_index(index)
    }

    fn is_valid_public_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.indexer_ref().is_valid_public_index(index)
    }

    fn try_is_valid_public_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        self.indexer_ref().try_is_valid_public_index(index)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use crate::graph::indexing::{
        operations::{CheckIndex, FreeIndex, GeneratePrivateIndex, GeneratePublicIndex},
        Indexer,
    };

    #[test]
    fn test_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

        let n_indices = 10;
        for _i in 0..n_indices {
            indexer.new_private_index().unwrap();
        }

        for _i in 0..n_indices {
            indexer.new_public_index().unwrap();
        }

        indexer.free_private_index(0).unwrap();
        indexer.free_private_index(3).unwrap();
        indexer.free_private_index(4).unwrap();

        indexer.free_public_index(10).unwrap();
        indexer.free_public_index(13).unwrap();
        indexer.free_public_index(14).unwrap();

        indexer.new_public_index().unwrap();
        indexer.new_private_index().unwrap();

        assert!(indexer.is_valid_public_index(&0).unwrap());
        assert!(indexer.is_valid_public_index(&15).unwrap());
        assert_eq!(indexer.is_valid_public_index(&1).unwrap(), false);

        assert_eq!(indexer.is_valid_public_index(&3).unwrap(), false);
        assert_eq!(indexer.is_valid_index(&3).unwrap(), true);

        assert_eq!(indexer.is_valid_public_index(&5).unwrap(), false);
        assert_eq!(indexer.is_valid_index(&5).unwrap(), true);

        assert_eq!(indexer.is_valid_public_index(&1).unwrap(), false);
    }
}
