use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::RegisterEmptyVertexToRestore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateReverters};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::transaction::GetVertexStoreStateRestorer;
use crate::graph::vertex_store::operations::GetVertexVectorNativeValueType;

pub(crate) trait RegisterNewVertex<'t> {
    fn register_new_public_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;

    fn register_new_private_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterNewVertex<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_new_public_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_new_public_index_to_revert(vertex_index)?;

        self.register_new_vertex(vertex_type_index, vertex_index);
        Ok(())
    }

    fn register_new_private_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_new_private_index_to_revert(vertex_index)?;

        self.register_new_vertex(vertex_type_index, vertex_index);
        Ok(())
    }
}

impl<'t> AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_new_vertex(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    ) {
        match self
            .vertex_store_ref()
            .vertex_vector_native_value_type_unchecked(&vertex_type_index)
        {
            ValueTypeIdentifier::Bool => bool::register_empty_vertex_element_to_restore(
                self,
                vertex_type_index,
                vertex_index,
            ),
            ValueTypeIdentifier::Int8 => {
                i8::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_empty_vertex_element_to_restore(self, vertex_type_index, vertex_index)
            }
            ValueTypeIdentifier::ISize => isize::register_empty_vertex_element_to_restore(
                self,
                vertex_type_index,
                vertex_index,
            ),
            ValueTypeIdentifier::USize => usize::register_empty_vertex_element_to_restore(
                self,
                vertex_type_index,
                vertex_index,
            ),
        }
    }
}

trait RegisterEmptyVertexToRestoreTyped<'t> {
    fn register_empty_vertex_element_to_restore(
        transaction: &'t mut AtomicInMemoryVertexStoreTransaction<'t>,
        vertex_type_index: VertexTypeIndex,
        vertex_index: &impl GetAssignedIndexData,
    );
}

macro_rules! implement_register_empty_vertex_to_restore_typed {
    ($value_type:ty) => {
        impl<'t> RegisterEmptyVertexToRestoreTyped<'t> for $value_type {
            fn register_empty_vertex_element_to_restore(
                transaction: &'t mut AtomicInMemoryVertexStoreTransaction<'t>,
                vertex_type_index: VertexTypeIndex,
                vertex_index: &impl GetAssignedIndexData,
            ) {
                RegisterEmptyVertexToRestore::<'t, $value_type>::register_empty_vertex_to_restore(
                    transaction
                        .vertex_store_state_restorer_mut_ref()
                        .vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    vertex_index.index(),
                );
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_register_empty_vertex_to_restore_typed);
