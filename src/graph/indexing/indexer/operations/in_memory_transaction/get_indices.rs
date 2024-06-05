use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    operations::GetVectorElementIndices, SparseVector,
};

use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::ElementIndex;
use crate::{error::GraphComputingError, graph::indexing::Indexer};

pub(crate) trait GetValidIndices {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}

pub(crate) trait GetValidPublicIndices {
    fn mask_with_valid_public_indices_ref(&mut self) -> &SparseVector<bool>;
    fn valid_public_indices(&mut self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}

pub(crate) trait GetValidPrivateIndices {
    fn mask_with_valid_private_indices_ref(&mut self) -> &SparseVector<bool>;
    fn valid_private_indices(&mut self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}

impl GetValidIndices for Indexer {
    // The mask is updated at each push() and free() operation.
    // benefit: mask is pre-computed, resulting in faster query operations
    // downside: slower push() and free() operations
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
        GetIndexMask::mask_with_valid_indices_ref(self)
    }

    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        // self.key_to_index_map.values().into_iter().collect()
        Ok(GetIndexMask::mask_with_valid_indices_ref(self).element_indices()?)
    }
}

impl GetValidPublicIndices for Indexer {
    // The mask is updated at each push() and free() operation.
    // benefit: mask is pre-computed, resulting in faster query operations
    // downside: slower push() and free() operations
    fn mask_with_valid_public_indices_ref(&mut self) -> &SparseVector<bool> {
        GetIndexMask::mask_with_valid_public_indices_ref(self)
    }

    fn valid_public_indices(&mut self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        // self.key_to_index_map.values().into_iter().collect()
        Ok(GetIndexMask::mask_with_valid_public_indices_ref(self).element_indices()?)
    }
}

impl GetValidPrivateIndices for Indexer {
    // The mask is updated at each push() and free() operation.
    // benefit: mask is pre-computed, resulting in faster query operations
    // downside: slower push() and free() operations
    fn mask_with_valid_private_indices_ref(&mut self) -> &SparseVector<bool> {
        GetIndexMask::mask_with_valid_private_indices_ref(self)
    }

    fn valid_private_indices(&mut self) -> Result<Vec<ElementIndex>, GraphComputingError> {
        // self.key_to_index_map.values().into_iter().collect()
        Ok(GetIndexMask::mask_with_valid_private_indices_ref(self).element_indices()?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use crate::graph::indexing::{
        operations::{FreeIndex, GeneratePrivateIndex, GeneratePublicIndex},
        Indexer,
    };

    #[test]
    fn test_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(&GraphBLASContext::init_default().unwrap(), &0).unwrap();

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

        assert_eq!(
            crate::graph::indexing::operations::GetValidIndices::valid_indices(&indexer).unwrap(),
            vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 15, 16, 17, 18, 19]
        )
    }
}
