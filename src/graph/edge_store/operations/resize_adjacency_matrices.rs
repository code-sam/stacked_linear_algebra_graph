use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            weighted_adjacency_matrix::{
                operations::ResizeWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
            },
            EdgeStore, GetAdjacencyMatrices,
        },
        indexing::ElementCount,
    },
};

use super::map::MapMutableAdjacencyMatrices;

pub(crate) trait ResizeAdjacencyMatrices {
    ///
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl ResizeAdjacencyMatrices for EdgeStore {
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_adjacency_matrices(|adjacency_matrix: &mut WeightedAdjacencyMatrix| {
            // TODO: improve cache invalidation logic, such that, where possible, chached attributes are resized instead of invalidated
            adjacency_matrix.resize(new_vertex_capacity)
        })?;
        *self.adjacency_matrix_size_mut_ref() = new_vertex_capacity;
        Ok(())
    }
}
