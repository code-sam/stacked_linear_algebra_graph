use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::vertex_type::{
    add_private_vertex_type, add_public_vertex_type, AddPrivateVertexType, AddPublicVertexType,
};
use crate::{error::GraphComputingError, graph::vertex_store::VertexStore};

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
