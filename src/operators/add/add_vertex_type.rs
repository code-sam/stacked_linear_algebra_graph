use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait, VertexTypeIndex},
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::operations::add_vertex_type::AddVertexType as AddVertexTypeToVertexStore,
    },
};

pub trait AddVertexType<T: ValueType> {
    fn add_new_vertex_type(
        &mut self,
        vertex_type: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for Graph {
    fn add_new_vertex_type(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError> {
        AddVertexTypeToVertexStore::<T>::add_new_vertex_type(
            self.vertex_store_mut_ref(),
            vertex_type_key,
        )
    }
}
