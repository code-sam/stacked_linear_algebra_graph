use crate::graph::indexing::operations::free_index_unchecked;
use crate::graph::indexing::operations::free_valid_index;
use crate::graph::indexing::operations::FreeIndex;
use crate::graph::indexing::Index;
use crate::{error::GraphComputingError, graph::indexing::Indexer};

impl FreeIndex for Indexer {
    // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
    fn free_valid_index(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_valid_index(self, index)
    }

    fn free_index_unchecked(&mut self, index: Index) -> Result<(), GraphComputingError> {
        free_index_unchecked(self, index)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use crate::graph::indexing::operations::{CheckIndex, FreeIndex, GenerateIndex};
    use crate::graph::indexing::GetAssignedIndexData;

    use super::*;

    #[test]
    fn test_reuse_index() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 10).unwrap();

        let initial_index = indexer.new_index().unwrap();
        indexer.free_valid_index(initial_index.index()).unwrap();

        let reused_index = indexer.new_index().unwrap();

        assert_eq!(initial_index.index(), reused_index.index());

        assert!(indexer.is_valid_index(initial_index.index()).unwrap());
    }
}
