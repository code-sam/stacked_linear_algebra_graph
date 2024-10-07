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

pub(crate) trait RegisterUpdatedVertex<'t> {
    fn register_updated_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterUpdatedVertex<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_updated_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_vertex_value_to_restore(vertex_type_index, vertex_index)?;
        Ok(())
    }
}
