use crate::{
    error::GraphComputingError,
    graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex}, versioned_graph::indexing::GetVersionedVertexIndexIndex,
};

pub trait DropVertexIndexVersioned {
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVersionedVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

pub trait DropVertexIndex {
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

pub trait DeleteVertexValue {
    fn delete_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_element_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
