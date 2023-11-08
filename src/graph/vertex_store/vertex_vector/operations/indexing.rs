use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    is_element, try_is_element,
};

use crate::{
    error::GraphComputingError,
    graph::{graph::VertexIndex, value_type::ValueType, vertex_store::VertexVector},
};

pub(crate) trait IsElementInVertexVector<T: ValueType> {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError>;
}

impl<T: ValueType> IsElementInVertexVector<T> for VertexVector {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError> {
        Ok(is_element(self, *vertex_index)?)
    }

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError> {
        Ok(try_is_element(self, *vertex_index)?)
    }
}

// pub(crate) trait Indexing {
//     fn is_valid_index(&self, index: &VertexIndex) -> Result<bool, GraphComputingError>;
//     fn is_valid_key(&self, key: &VertexKeyRef) -> bool;

//     fn try_index_validity(&self, index: &VertexIndex) -> Result<(), GraphComputingError>;
//     fn try_key_validity(&self, key: &VertexKeyRef) -> Result<(), GraphComputingError>;

//     fn index_for_key(&self, key: &VertexKeyRef) -> Option<&Index>;
//     fn try_index_for_key(&self, key: &VertexKeyRef) -> Result<&Index, GraphComputingError>;
//     fn key_for_index(&self, index: &VertexIndex) -> Result<Key, GraphComputingError>;
// }

// impl Indexing for VertexStore {
//     fn is_valid_index(&self, index: &VertexIndex) -> Result<bool, GraphComputingError> {
//         self.indexer_ref().is_valid_index(index)
//     }

//     fn is_valid_key(&self, key: &VertexKeyRef) -> bool {
//         self.indexer_ref().is_valid_key(key)
//     }

//     fn try_index_validity(&self, index: &VertexIndex) -> Result<(), GraphComputingError> {
//         self.indexer_ref().try_index_validity(index)
//     }

//     fn try_key_validity(&self, key: &VertexKeyRef) -> Result<(), GraphComputingError> {
//         self.indexer_ref().try_key_validity(key)
//     }

//     fn index_for_key(&self, key: &VertexKeyRef) -> Option<&Index> {
//         self.indexer_ref().index_for_key(key)
//     }

//     fn try_index_for_key(&self, key: &VertexKeyRef) -> Result<&Index, GraphComputingError> {
//         self.indexer_ref().try_index_for_key(key)
//     }

//     fn key_for_index(&self, index: &VertexIndex) -> Result<Key, GraphComputingError> {
//         self.indexer_ref().key_for_index(index)
//     }
// }
