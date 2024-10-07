use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::vertex_vectors_state_restorer::RegisterVertexValueToRestore;
use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{VertexIndex, VertexTypeIndex},
        value_type::implement_macro_for_all_native_value_types,
        vertex_store::{operations::VertexStoreStateRestorer, VertexVector},
    },
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueUntyped;

pub(crate) trait RegisterVertexValueToRestoreTyped<'t> {
    fn register_vertex_value_to_restore(
        vertex_vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
        vertex_vector: &VertexVector,
        vertex_index: VertexIndex,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_vertex_value_to_restore_typed {
    ($value_type:ty) => {
        impl<'t> RegisterVertexValueToRestoreTyped<'t> for $value_type {
            fn register_vertex_value_to_restore(
                vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
                vertex_vector: &VertexVector,
                vertex_index: VertexIndex,
                vertex_type_index: VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let vertex_value_to_restore = unsafe {
                    <$value_type>::element_value(vertex_vector, vertex_index.index())?.unwrap()
                }; // TODO: would it be safer to match None? How could this error occur?

                RegisterVertexValueToRestore::<'t, $value_type>::register_vertex_value_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    vertex_index,
                    vertex_value_to_restore,
                );

                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_register_vertex_value_to_restore_typed);
