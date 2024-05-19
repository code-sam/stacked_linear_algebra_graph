use crate::{
    error::GraphComputingError,
    graph::{
        graph::{GetVertexStore, Graph},
        indexing::VertexTypeIndex,
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::operations::add_vertex_type::{
            AddPrivateVertexType as AddPrivateVertexTypeToVertexStore,
            AddPublicVertexType as AddPublicVertexTypeToVertexStore,
        },
    },
};

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddPublicVertexTypeToVertexStore::<T>::apply(self.vertex_store_mut_ref())
    }
}

impl<T: ValueType + GetValueTypeIdentifier> AddPrivateVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddPrivateVertexTypeToVertexStore::<T>::apply(self.vertex_store_mut_ref())
    }
}
