use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{is_valid_index, try_index_validity, CheckIndex};
use crate::graph::indexing::Index;
use crate::graph::indexing::Indexer;

impl CheckIndex for Indexer {
    fn is_valid_index(&self, index: Index) -> Result<bool, GraphComputingError> {
        is_valid_index(self, index)
    }

    fn try_index_validity(&self, index: Index) -> Result<(), GraphComputingError> {
        try_index_validity(self, index)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use crate::graph::indexing::{
        operations::{CheckIndex, FreeIndex, GenerateIndex},
        Indexer,
    };

    #[test]
    fn test_valid_indices() {
        let mut indexer =
            Indexer::with_initial_capacity(GraphBLASContext::init_default().unwrap(), 0).unwrap();

        let n_indices = 10;
        for _i in 0..n_indices {
            indexer.new_index().unwrap();
        }

        for _i in 0..n_indices {
            indexer.new_index().unwrap();
        }

        indexer.free_valid_index(0).unwrap();
        indexer.free_valid_index(3).unwrap();
        indexer.free_valid_index(4).unwrap();

        indexer.free_valid_index(10).unwrap();
        indexer.free_valid_index(13).unwrap();
        indexer.free_valid_index(14).unwrap();

        indexer.new_index().unwrap();
        indexer.new_index().unwrap();

        assert!(indexer.is_valid_index(0).unwrap());
        assert!(indexer.is_valid_index(15).unwrap());
        assert_eq!(indexer.is_valid_index(1).unwrap(), false);

        assert_eq!(indexer.is_valid_index(3).unwrap(), false);
        assert_eq!(indexer.is_valid_index(3).unwrap(), true);

        assert_eq!(indexer.is_valid_index(5).unwrap(), false);
        assert_eq!(indexer.is_valid_index(5).unwrap(), true);

        assert_eq!(indexer.is_valid_index(1).unwrap(), false);
    }
}
