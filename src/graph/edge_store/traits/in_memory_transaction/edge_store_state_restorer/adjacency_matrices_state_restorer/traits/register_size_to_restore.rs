use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::AdjacencyMatricesWithCachedAttributesStateRestorer;

pub(crate) trait RegisterAdjacencyMatrixSizeToRestore {
    fn register_adjacency_matrix_size_to_restore(&mut self, size: &Size);
}

impl RegisterAdjacencyMatrixSizeToRestore for AdjacencyMatricesWithCachedAttributesStateRestorer {
    fn register_adjacency_matrix_size_to_restore(&mut self, size: &Size) {
        if self.adjacency_matrix_size_to_restore.is_none() {
            self.adjacency_matrix_size_to_restore = Some(size.to_owned())
        }
    }
}
