use crate::error::GraphComputingError;
use crate::graph::indexing::{GetAssignedIndexData, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    GetSparseVectorStateRevertersByVertexTypeMap, VertexStoreStateRestorer,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use super::{RegisterEmptyVertexToRestore, RegisterNewVertexIndexToRevert};

pub(crate) trait RegisterNewVertexToRevert<T> {
    fn register_new_vertex_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl<T> RegisterNewVertexToRevert<T> for VertexStoreStateRestorer
where
    T: ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn register_new_vertex_to_revert(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.register_new_vertex_index_to_revert(vertex_index)?;
        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self,
            vertex_type_index,
            vertex_index.index(),
        );
        Ok(())
    }
}
