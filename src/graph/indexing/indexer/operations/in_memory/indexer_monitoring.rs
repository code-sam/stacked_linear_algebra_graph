use graphblas_sparse_linear_algebra::collections::{
    sparse_vector::operations::GetSparseVectorLength, Collection,
};

use crate::{
    error::GraphComputingError,
    graph::indexing::{
        operations::{GetIndexerStatus, GetValidIndices},
        ElementCount, Indexer,
    },
};

impl GetIndexerStatus for Indexer {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self
            .mask_with_valid_indices_ref()
            .number_of_stored_elements()?)
    }

    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.mask_with_valid_indices_ref().length()?)
    }
}
