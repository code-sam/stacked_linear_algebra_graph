use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{EdgeStore, EdgeStoreTrait},
        index::ElementCount,
    },
};

pub(crate) trait ResizeAdjacencyMatrices {
    ///
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl ResizeAdjacencyMatrices for EdgeStore {
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError> {
        EdgeStoreTrait::resize_adjacency_matrices(self, new_vertex_capacity.to_owned())?;
        Ok(())
    }
}
