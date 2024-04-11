use crate::{
    error::GraphComputingError,
    graph::{
        graph::{GetVertexStore, Graph, VertexTypeIndex},
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::operations::add_vertex_type::AddVertexType as AddVertexTypeToVertexStore,
    },
};

pub trait AddVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddVertexTypeToVertexStore::<T>::new_vertex_type(self.vertex_store_mut_ref())
    }
}
