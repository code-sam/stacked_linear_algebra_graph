use graphblas_sparse_linear_algebra::collections::{
    sparse_vector::operations::GetSparseVectorLength, Collection,
};

use crate::{
    error::GraphComputingError,
    graph::indexing::{
        operations::{index_capacity, number_of_indexed_elements, GetIndexerStatus},
        ElementCount, GetIndexMask, Indexer,
    },
};

impl GetIndexerStatus for Indexer {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        number_of_indexed_elements(self)
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        index_capacity(self)
    }
}
