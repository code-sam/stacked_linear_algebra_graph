use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex};


pub trait UpdateVertexValue<T: ValueType> {
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait UpdateVertexValueUnchecked<T: ValueType> {
    fn update_vertex_value_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
