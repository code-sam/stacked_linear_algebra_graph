use graphblas_sparse_linear_algebra::collections::{
    sparse_vector::operations::GetSparseVectorLength, Collection,
};

use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, Indexer},
};

use super::GetValidIndices;

pub(crate) trait GetIndexerStatus {
    fn number_of_indexed_elements(&self) -> Result<ElementCount, GraphComputingError>;
    fn index_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}
