use crate::{
    error::GraphComputingError,
    graph::{
        indexing::VertexTypeIndex,
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::{
            operations::{
                add_private_vertex_type, add_public_vertex_type, AddPrivateVertexType,
                AddPublicVertexType,
            },
            VertexStore,
        },
    },
};

impl<'a, T: ValueType + GetValueTypeIdentifier> AddPublicVertexType<'a, T> for VertexStore {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let assigned_vertex_type_index = add_public_vertex_type::<T>(self)?;
        Ok(assigned_vertex_type_index.into())
    }
}

impl<'a, T: ValueType + GetValueTypeIdentifier> AddPrivateVertexType<'a, T> for VertexStore {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let assigned_vertex_type_index = add_private_vertex_type::<T>(self)?;
        Ok(assigned_vertex_type_index.into())
    }
}
