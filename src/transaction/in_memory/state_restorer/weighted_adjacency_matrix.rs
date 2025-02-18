use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;
use crate::graph::{value_type::ValueType, weighted_adjacency_matrix::WeightedAdjacencyMatrix};
use crate::transaction::RestoreState;

use super::{restore_sparse_matrix_state, GetSparseMatrixSizeToRestore, SparseMatrixStateReverter};

impl<T: ValueType + Copy + SetSparseMatrixElementTyped<T>> RestoreState<WeightedAdjacencyMatrix>
    for SparseMatrixStateReverter<T>
{
    fn restore(
        self,
        instance_to_restore: &mut WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError> {
        restore_sparse_matrix_state(self, instance_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_size_to_restore(self.matrix_size_to_restore())
    }
}
