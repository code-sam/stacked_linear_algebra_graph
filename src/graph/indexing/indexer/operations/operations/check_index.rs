use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::graph::indexing::Index;
use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::indexing::Indexer,
};

pub(crate) trait CheckIndex {
    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError>;

    fn is_valid_private_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn try_is_valid_private_index(&self, index: &Index) -> Result<(), GraphComputingError>;

    fn is_public_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn try_is_public_index(&self, index: &Index) -> Result<(), GraphComputingError>;

    fn is_valid_public_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn try_is_valid_public_index(&self, index: &Index) -> Result<(), GraphComputingError>;
}
