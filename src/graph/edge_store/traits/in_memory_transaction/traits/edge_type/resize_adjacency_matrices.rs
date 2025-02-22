use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction, RegisterAdjacencyMatrixSizeToRestore,
};
use crate::graph::edge_store::traits::traits::edge_type::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::indexing::ElementCount;

impl<'s> ResizeAdjacencyMatrices for InMemoryEdgeStoreTransaction<'s> {
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_state_restorer
            .register_adjacency_matrix_size_to_restore(&Size::new(
                new_vertex_capacity,
                new_vertex_capacity,
            ))?;

        self.edge_store_mut_ref()
            .resize_adjacency_matrices(new_vertex_capacity)
    }
}
