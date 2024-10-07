use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::{indexing::{VertexIndex, VertexTypeIndex}, value_type::ValueType};
use crate::operators::in_memory_transaction::transaction::{CreateSparseVectorStateReverter, RegisterSparseVectorChangeToRevert};

pub(crate) trait RegisterVertexValueToRestore<'a, T: ValueType> {
    fn register_vertex_value_to_restore(
        &'a mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
        vertex_value: T,
    );
}

impl<'a, T> RegisterVertexValueToRestore<'a, T> for VertexVectorsStateRestorer
where
    T: 'a
        + ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<'a, T>
        + CreateSparseVectorStateReverter<T>,
{
    fn register_vertex_value_to_restore(
        &'a mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: VertexIndex,
        vertex_value: T,
    ) {
        self.vertex_vector_state_reverter_mut_ref(vertex_type_index)
            .register_element_value_to_restore(vertex_index.index(), vertex_value)
    }
}
