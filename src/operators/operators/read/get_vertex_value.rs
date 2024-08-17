use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;

pub trait GetVertexValue<T: ValueType> {
    fn vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
}

pub(crate) trait GetPrivateVertexValue<T: ValueType> {
    fn private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
