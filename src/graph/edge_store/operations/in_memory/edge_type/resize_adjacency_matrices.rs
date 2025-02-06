use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
    GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes,
};
use crate::graph::edge_store::operations::operations::edge_type::map::MapMutableAdjacencyMatrices;
use crate::graph::edge_store::operations::operations::edge_type::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::ResizeWeightedAdjacencyMatrix;
use crate::graph::edge_store::{EdgeStore, GetAdjacencyMatrices};
use crate::graph::indexing::ElementCount;

impl ResizeAdjacencyMatrices for EdgeStore {
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_adjacency_matrices(
            |adjacency_matrix: &mut WeightedAdjacencyMatrixWithCachedAttributes| {
                // TODO: improve cache invalidation logic, such that, where possible, chached attributes are resized instead of invalidated
                adjacency_matrix
                    .weighted_adjacency_matrix_mut_ref()
                    .resize(new_vertex_capacity)
            },
        )?;
        *self.adjacency_matrix_size_mut_ref() = new_vertex_capacity;
        Ok(())
    }
}
