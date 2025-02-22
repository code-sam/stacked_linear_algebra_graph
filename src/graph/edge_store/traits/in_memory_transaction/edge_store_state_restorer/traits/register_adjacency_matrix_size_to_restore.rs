use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::traits::RegisterAdjacencyMatrixSizeToRestore as RegisterAdjacencyMatricesSizeToRestore;
use crate::graph::edge_store::traits::in_memory_transaction::{EdgeStoreStateRestorer, GetEdgeStoreStateReverters};

pub(crate) trait RegisterAdjacencyMatrixSizeToRestore<'t> {
    fn register_adjacency_matrix_size_to_restore(
        &mut self,
        size_to_restore: &Size,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterAdjacencyMatrixSizeToRestore<'t> for EdgeStoreStateRestorer {
    fn register_adjacency_matrix_size_to_restore(
        &mut self,
        size_to_restore: &Size,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrices_state_restorer_mut_ref()
            .register_adjacency_matrix_size_to_restore(size_to_restore);
        Ok(())
    }
}
