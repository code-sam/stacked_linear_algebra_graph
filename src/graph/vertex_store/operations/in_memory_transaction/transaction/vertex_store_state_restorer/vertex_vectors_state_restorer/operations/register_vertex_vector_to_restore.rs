use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::VertexVector;
use crate::graph::{indexing::{VertexIndex, VertexTypeIndex}, value_type::ValueType};
use crate::operators::in_memory_transaction::transaction::{CreateSparseVectorStateReverter, RegisterSparseVectorChangeToRevert};

pub(crate) trait RegisterTypedVertexVectorToRestore<'a, T: ValueType> {
    fn register_vertex_vector_to_restore(
        &'a mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: SparseVector<T>,
    );
}

impl<'a, T> RegisterTypedVertexVectorToRestore<'a, T> for VertexVectorsStateRestorer
where
    T: 'a
        + ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<'a, T>
        + CreateSparseVectorStateReverter<T>,
{
    fn register_vertex_vector_to_restore(
        &'a mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: SparseVector<T>,
    ) {
        self.vertex_vector_state_reverter_mut_ref(vertex_type_index)
            .register_sparse_vector_state_to_restore(vertex_vector)
    }
}
