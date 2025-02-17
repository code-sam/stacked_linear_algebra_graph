use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedVertexTypeIndex, VersionedVertexIndex};

pub trait NewVertexVersioned<T: ValueType> {
    fn new_vertex(
        &mut self,
        vertex_type: &impl GetVersionedVertexTypeIndex,
        value: T,
    ) -> Result<VersionedVertexIndex, GraphComputingError>;
}

pub trait NewVertex<T: ValueType> {
    fn new_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
