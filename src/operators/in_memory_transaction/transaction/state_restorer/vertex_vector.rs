use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::{value_type::ValueType, vertex_store::VertexVector};
use crate::operators::transaction::RestoreState;

use super::{
    restore_sparse_vector_state, GetSparseVectorStateToRestore, SparseVectorStateReverter
};

impl<T: ValueType + Copy + SetSparseVectorElementTyped<T>> RestoreState<VertexVector>
    for SparseVectorStateReverter<T>
{
    fn restore(self, instance_to_restore: &mut VertexVector) -> Result<(), GraphComputingError> {
        restore_sparse_vector_state(self, instance_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_length_to_restore(self.length_to_restore())
    }
}
