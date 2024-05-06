use crate::{
    error::GraphComputingError,
    graph::{
        indexing::ElementCount, vertex_store::{ResizeWeightedAdjacencyMatrix, VertexStore, VertexVector}
    },
};

use super::map::MapAllVertexVectors;

pub(crate) trait ResizeVertexVectors {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl ResizeVertexVectors for VertexStore {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_vertex_vectors(|vertex_vector: &mut VertexVector| {
            vertex_vector.resize(new_vertex_capacity)
        })?;
        Ok(())
    }
}
