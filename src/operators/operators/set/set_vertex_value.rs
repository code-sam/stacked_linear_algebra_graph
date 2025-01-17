use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;

pub trait SetVertexValue<T: ValueType> {
    fn set_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait SetVertexValueUnchecked<T: ValueType> {
    fn set_vertex_value_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
