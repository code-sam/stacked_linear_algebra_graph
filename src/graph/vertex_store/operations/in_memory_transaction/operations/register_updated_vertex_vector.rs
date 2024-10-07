use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueUntyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetGraphblasSparseVector;
use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData, GetVertexIndexIndex, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_vectors_state_restorer::{RegisterEmptyVertexToRestore, RegisterVertexValueToRestore};
use crate::graph::vertex_store::operations::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer, GetVertexVector, GetVertexVectorNativeValueType, VertexStoreStateRestorer
};
use crate::graph::vertex_store::VertexVector;

pub(crate) trait RegisterUpdatedVertexVector<'t> {
    fn register_updated_vertex_vector(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterUpdatedVertexVector<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_updated_vertex_vector(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_vertex_vector_to_restore(vertex_type_index)?;
        Ok(())
    }
}
