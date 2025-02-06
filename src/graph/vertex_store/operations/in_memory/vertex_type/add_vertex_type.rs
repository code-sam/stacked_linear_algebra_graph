use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::vertex_type::{add_vertex_type, AddVertexType};
use crate::{error::GraphComputingError, graph::vertex_store::VertexStore};

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for VertexStore {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let assigned_vertex_type_index = add_vertex_type::<T>(self)?;
        Ok(assigned_vertex_type_index.into())
    }
}
