use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait, VertexTypeIndex},
        vertex::VertexTypeKeyRef,
        vertex_store::type_operations::add_vertex_type::AddVertexType as AddVertexTypeToVertexStore,
    },
};

pub trait AddVertexType {
    fn add_new_vertex_type(
        &mut self,
        vertex_type: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl AddVertexType for Graph {
    fn add_new_vertex_type(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError> {
        self.vertex_store_mut_ref()
            .add_new_vertex_type(vertex_type_key)
    }
}
