use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;
use crate::graph::vertex_store::operations::MapAllVertexVectors;
use crate::graph::vertex_store::{
    operations::ResizeVertexVectors, ResizeWeightedAdjacencyMatrix, VertexStore, VertexVector,
};

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
