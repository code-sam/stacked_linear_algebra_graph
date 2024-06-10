use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    operations::GetSparseVectorElementIndices, SparseVector,
};

use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::ElementIndex;
use crate::{error::GraphComputingError, graph::indexing::Indexer};

pub(crate) trait GetValidIndices {
    fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}

pub(crate) trait GetValidPublicIndices {
    fn mask_with_valid_public_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_public_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}

pub(crate) trait GetValidPrivateIndices {
    fn mask_with_valid_private_indices_ref(&self) -> &SparseVector<bool>;
    fn valid_private_indices(&self) -> Result<Vec<ElementIndex>, GraphComputingError>;
}
