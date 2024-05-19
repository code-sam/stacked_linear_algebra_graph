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

pub trait AddVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}

pub(crate) trait AddPrivateVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}
