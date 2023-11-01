use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            weighted_adjacency_matrix::{
                operations::ResizeWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
            },
            EdgeStore, EdgeStoreTrait,
        },
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
        self.map_mut_all_adjacency_matrices(|adjacency_matrix: &mut WeightedAdjacencyMatrix| {
            adjacency_matrix.resize(*new_vertex_capacity)
            // .sparse_matrix_mut_ref()
            // .resize(&(new_vertex_capacity, new_vertex_capacity).into())
        })?;
        *self.adjacency_matrix_size_mut_ref() = *new_vertex_capacity;
        Ok(())
    }
}
