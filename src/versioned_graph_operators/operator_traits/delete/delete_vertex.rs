use crate::{
    error::GraphComputingError,
    graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex},
    versioned_graph::indexing::GetVersionedVertexIndexIndex,
};

pub trait DropVertexIndexVersioned {
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVersionedVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
