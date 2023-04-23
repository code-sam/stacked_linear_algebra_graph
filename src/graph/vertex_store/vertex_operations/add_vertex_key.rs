use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex},
        indexer::{AssignedIndex, IndexerTrait},
        vertex::{VertexKeyRef, VertexTypeKeyRef},
        vertex_store::{
            type_operations::add_vertex_type::AddVertexType as AddVertexTypeToVertexStore,
            VertexStore, VertexStoreTrait,
        },
    },
};

pub(crate) trait AddVertexKey {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<AssignedIndex, GraphComputingError>;
}

impl AddVertexKey for VertexStore {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<AssignedIndex, GraphComputingError> {
        Ok(self.element_indexer_mut_ref().add_new_key(vertex_key)?)
    }
}
