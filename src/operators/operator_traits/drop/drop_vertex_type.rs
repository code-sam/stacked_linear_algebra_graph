use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexTypeIndex;
use crate::versioned_graph::indexing::GetVersionedVertexTypeIndex;

pub trait DropVertexTypeVersioned {
    /// Deletes the vertex type, and all its edges
    fn drop_vertex_type(
        &mut self,
        vertex_type: &impl GetVersionedVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

pub trait DropVertexType {
    /// Deletes the vertex type, and all its edges
    fn drop_vertex_type(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
