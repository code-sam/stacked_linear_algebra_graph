use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::vertex_vector::ToSparseVector;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::RegisterTypedVertexVectorToRestore;
use crate::graph::vertex_store::vertex_vector::IntoSparseVector;
use crate::graph::vertex_store::vertex_vector::IntoSparseVectorAndClearValuesForValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

pub(crate) trait RegisterVertexVectorToRestore<'t> {
    fn register_deleted_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &mut VertexVector,
    ) -> Result<(), GraphComputingError>;

    fn register_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterVertexVectorToRestore<'t> for VertexStoreStateRestorer {
    fn register_deleted_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &mut VertexVector,
    ) -> Result<(), GraphComputingError> {
        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_deleted_vertex_vector_to_restore(
                    self,
                    vertex_vector,
                    vertex_type_index,
                )?;
            }
        }
        Ok(())
    }

    fn register_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
    ) -> Result<(), GraphComputingError> {
        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_vertex_vector_to_restore(self, vertex_vector, vertex_type_index)?;
            }
        }
        Ok(())
    }
}

pub(crate) trait RegisterUntypedVertexVectorToRestore<'t> {
    fn register_deleted_vertex_vector_to_restore(
        vertex_vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
        vertex_vector: &mut VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_vertex_vector_to_restore(
        vertex_vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
        vertex_vector: &VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_untyped_vertex_value_to_restore_typed {
    ($value_type:ty) => {
        impl<'t> RegisterUntypedVertexVectorToRestore<'t> for $value_type {
            fn register_deleted_vertex_vector_to_restore(
                vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
                vertex_vector_to_restore: &mut VertexVector,
                vertex_type_index: &impl GetVertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let sparse_vertex_vector = <$value_type>::into_sparse_vector_and_clear_values(vertex_vector_to_restore)?;

                RegisterTypedVertexVectorToRestore::<'t, $value_type>::register_vertex_vector_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    sparse_vertex_vector,
                );

                Ok(())
            }

            fn register_vertex_vector_to_restore(
                vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
                vertex_vector_to_restore: &VertexVector,
                vertex_type_index: &impl GetVertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let sparse_vertex_vector = vertex_vector_to_restore.to_sparse_vector()?;

                RegisterTypedVertexVectorToRestore::<'t, $value_type>::register_vertex_vector_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    sparse_vertex_vector,
                );

                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_register_untyped_vertex_value_to_restore_typed
);
