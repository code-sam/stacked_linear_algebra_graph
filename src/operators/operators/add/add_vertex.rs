use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex::{GetVertexIndex, GetVertexValue};

pub trait AddVertex<T: ValueType> {
    fn add_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

pub(crate) trait AddPrivateVertex<T: ValueType> {
    fn add_private_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_private_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_private_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
