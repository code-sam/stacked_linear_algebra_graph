// use std::marker::PhantomData;
// use std::sync::Arc;

// use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
//     MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
// };
// use graphblas_sparse_linear_algebra::context::Context;

// use crate::error::GraphComputingError;
// use crate::graph::graph::VertexIndex;
// use crate::graph::index::ElementCount;
// use crate::graph::index::Index;
// use crate::graph::value_type::NativeDataType as GraphNativeDataType;
// use crate::graph::value_type::ValueType;
// use crate::graph::value_type::{
//     implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
// };
// use crate::graph::vertex::VertexKeyRef;
// use crate::graph::vertex_store::{VertexStore, VertexStoreTrait};

// use crate::graph::indexer::{Indexer, IndexerTrait, Key, KeyRef};

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
