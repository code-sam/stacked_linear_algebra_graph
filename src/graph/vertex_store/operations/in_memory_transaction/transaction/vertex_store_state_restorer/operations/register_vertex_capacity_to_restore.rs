use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::indexing::{ElementCount, GetVertexTypeIndex};
use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::RegisterVertexVectorCapacityToRestore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

pub(crate) trait RegisterVertexCapacityToRestore<'t> {
    fn register_vertex_capacity_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterVertexCapacityToRestore<'t> for VertexStoreStateRestorer {
    fn register_vertex_capacity_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
        current_vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError> {
        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_untyped_vertex_capacity_to_restore(
                    self,
                    vertex_type_index,
                    current_vertex_capacity,
                )?;
            }
        }
        Ok(())
    }
}

trait RegisterUntypedVertexVectorCapacityToRestore<'t> {
    fn register_untyped_vertex_capacity_to_restore(
        vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_untyped_vertex_vector_capacity_to_restore_typed {
    ($value_type:ty) => {
        impl<'t> RegisterUntypedVertexVectorCapacityToRestore<'t> for $value_type {
            fn register_untyped_vertex_capacity_to_restore(
                vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
                vertex_type_index: &impl GetVertexTypeIndex,
                vertex_capacity: &ElementCount,
            ) -> Result<(), GraphComputingError> {
                RegisterVertexVectorCapacityToRestore::<'t, $value_type>::register_vertex_vector_capacity_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    vertex_capacity
                );
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_register_untyped_vertex_vector_capacity_to_restore_typed
);
