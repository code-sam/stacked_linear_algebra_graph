use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;

use crate::graph::indexing::indexer::indexer::GetIndexMask;
use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{index::Index, indexing::Indexer},
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

impl CheckIndex for Indexer {
    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        match self.mask_with_valid_indices_ref().element_value(index)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn try_index_validity(&self, index: &Index) -> Result<(), GraphComputingError> {
        if self.is_valid_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!("No valid index [{}], the index may have been freed.", index),
                None,
            )
            .into());
        }
    }

    fn is_valid_private_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        Ok(self.is_index_private(index)? && self.is_valid_index(index)?)
    }

    fn try_is_valid_private_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        if self.is_valid_private_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No valid private index [{}], the index may have been freed.",
                    index
                ),
                None,
            )
            .into());
        }
    }

    fn is_public_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.is_index_not_private(index)
    }

    fn try_is_public_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        if self.is_public_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No public index [{}], the index may have been freed.",
                    index
                ),
                None,
            )
            .into());
        }
    }

    fn is_valid_public_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        Ok(self.is_public_index(index)? && self.is_valid_index(index)?)
    }

    fn try_is_valid_public_index(&self, index: &Index) -> Result<(), GraphComputingError> {
        if self.is_valid_public_index(index)? {
            return Ok(());
        } else {
            return Err(LogicError::new(
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No valid public index [{}], the index may have been freed.",
                    index
                ),
                None,
            )
            .into());
        }
    }
}

impl Indexer {
    fn is_index_private(&self, index: &Index) -> Result<bool, GraphComputingError> {
        match self.mask_with_private_indices_ref().element_value(index)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn is_index_not_private(&self, index: &Index) -> Result<bool, GraphComputingError> {
        match self.mask_with_private_indices_ref().element_value(index)? {
            Some(_) => Ok(false),
            None => Ok(true),
        }
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
