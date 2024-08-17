use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::DeleteSparseVectorElement;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::indexer::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::Index;
use crate::graph::indexing::Queue;
use crate::{error::GraphComputingError, graph::indexing::Indexer};

use super::CheckIndex;

pub(crate) trait FreeIndex {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    // fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError>;

    // fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError>;
}

impl FreeIndex for Indexer {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    // fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
    //     if self.is_valid_index(&index)? {
    //         self.free_index_unchecked(index)
    //     } else {
    //         Ok(())
    //     }
    // }

    fn free_public_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        if self.is_valid_public_index(index)? {
            self.free_public_index_unchecked(index)
        } else {
            Ok(())
        }
    }

    fn free_private_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        if self.is_valid_private_index(index)? {
            self.free_private_index_unchecked(index)
        } else {
            Ok(())
        }
    }

    // fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
    //     self.mask_with_valid_indices_mut_ref().drop_element(index)?;

    //     self.mask_with_private_indices_mut_ref()
    //         .drop_element(index)?;

    //     self.indices_available_for_reuse_mut_ref().push_back(index);
    //     Ok(())
    // }

    fn free_public_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_mut_ref().drop_element(index)?;
        self.mask_with_valid_public_indices_mut_ref()
            .drop_element(index)?;

        self.indices_available_for_reuse_mut_ref().push_back(index);
        Ok(())
    }

    fn free_private_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        self.mask_with_valid_indices_mut_ref().drop_element(index)?;
        self.mask_with_private_indices_mut_ref()
            .drop_element(index)?;
        self.mask_with_valid_private_indices_mut_ref()
            .drop_element(index)?;

        self.indices_available_for_reuse_mut_ref().push_back(index);
        Ok(())
    }
}

// TODO: introduce unit tests
// especially one for freeing a public index, and then generating a public one; and vice-versa.
// The test should check that the respective masks are updates correctly.

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use crate::graph::indexing::operations::GeneratePrivateIndex;
    use crate::graph::indexing::{
        operations::{CheckIndex, FreeIndex, GeneratePublicIndex},
        GetAssignedIndexData,
    };

    use super::*;

    #[test]
    fn test_resuse_public_index_as_private_index() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 10).unwrap();

        let public_index = indexer.new_public_index().unwrap();
        indexer.free_public_index(public_index.index()).unwrap();

        let private_index = indexer.new_private_index().unwrap();

        assert_eq!(public_index.index(), private_index.index());

        assert!(indexer.is_index_private(private_index.index()).unwrap());
        assert!(indexer
            .is_valid_private_index(private_index.index())
            .unwrap());
        assert!(!indexer.is_public_index(private_index.index()).unwrap());
        assert!(!indexer
            .is_valid_public_index(private_index.index())
            .unwrap());
    }

    #[test]
    fn test_resuse_private_index_as_public_index() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 10).unwrap();

        let private_index = indexer.new_private_index().unwrap();
        indexer.free_private_index(private_index.index()).unwrap();

        let public_index = indexer.new_public_index().unwrap();

        assert_eq!(public_index.index(), private_index.index());

        assert!(!indexer.is_index_private(public_index.index()).unwrap());
        assert!(!indexer
            .is_valid_private_index(public_index.index())
            .unwrap());
        assert!(indexer.is_public_index(public_index.index()).unwrap());
        assert!(indexer.is_valid_public_index(public_index.index()).unwrap());
    }
}
