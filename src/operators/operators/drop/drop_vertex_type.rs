use crate::error::GraphComputingError;

use crate::graph::graph::{GetVertexStore, Graph};
use crate::graph::indexing::operations::FreeIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::GetVertexTypeIndexer;

pub trait DropVertexType {
    /// Deletes the vertex type, and all its edges
    fn drop_vertex_type(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DropPrivateVertexType {
    /// Deletes the vertex type, and all its edges
    fn drop_private_vertex_type(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {

}
