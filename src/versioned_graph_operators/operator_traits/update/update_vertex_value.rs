use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex};

pub trait UpdateVertexValueVersioned<T: ValueType> {
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVersionedVertexTypeIndex,
        vertex_index: &impl GetVersionedVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
