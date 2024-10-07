use graphblas_sparse_linear_algebra::collections::sparse_vector::GetGraphblasSparseVector;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator};

use crate::operators::transaction::RestoreState;
use crate::graph::vertex_store::{AsSparseVector, VertexVector};
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_vectors_state_restorer::{restore_vertex_vectors_state, GetLengthToRestore, GetVertexVectorStateReverter, GetVertexVectorStateReverters, VertexVectorsStateRestorer};

impl RestoreState<Vec<VertexVector>> for VertexVectorsStateRestorer {
    fn restore(
        mut self,
        vectors_to_restore: &mut Vec<VertexVector>,
    ) -> Result<(), crate::error::GraphComputingError> {
        restore_vertex_vectors_state(self, vectors_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_length_to_restore(self.length_to_restore())
    }
}
