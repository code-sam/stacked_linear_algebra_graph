use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex};

pub trait GetVertexValueVersioned<T: ValueType> {
    fn vertex_value(
        &self,
        vertex_type_index: &impl GetVersionedVertexTypeIndex,
        vertex_index: &impl GetVersionedVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVersionedVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value(
        &self,
        vertex_type_index: &impl GetVersionedVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
}


#[cfg(test)]
mod tests {}
