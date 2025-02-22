use crate::graph::vertex_store::traits::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::RegisterTypedVertexValueToRestore;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::error::GraphComputingError;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueUntyped;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

use super::RegisterEmptyVertexToRestore;

pub(crate) trait RegisterVertexValueToRestore {
    fn register_vertex_value_to_restore(
        &mut self,
        vertex_vector: &VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_optional_vertex_value_to_restore(
        &mut self,
        vertex_vector: &VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterVertexValueToRestore for VertexStoreStateRestorer {
    fn register_vertex_value_to_restore(
        &mut self,
        vertex_vector: &VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
        }
        Ok(())
    }

    fn register_optional_vertex_value_to_restore(
        &mut self,
        vertex_vector: &VertexVector,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_optional_vertex_value_to_restore(
                    self,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
        }
        Ok(())
    }
}

pub(crate) trait RegisterVertexValueToRestoreTyped {
    fn register_vertex_value_to_restore(
        vertex_vertex_store_state_restorer: &mut VertexStoreStateRestorer,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_optional_vertex_value_to_restore(
        vertex_vertex_store_state_restorer: &mut VertexStoreStateRestorer,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_vector: &VertexVector,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_vertex_value_to_restore_typed {
    ($value_type:ty) => {
        impl RegisterVertexValueToRestoreTyped for $value_type {
            fn register_vertex_value_to_restore(
                vertex_store_state_restorer: &mut VertexStoreStateRestorer,
                vertex_type_index: &impl GetVertexTypeIndex,
                vertex_vector: &VertexVector,
                vertex_index: &impl GetVertexIndexIndex,
            ) -> Result<(), GraphComputingError> {
                let vertex_value_to_restore = unsafe {
                    <$value_type>::element_value(vertex_vector, vertex_index.index())?.unwrap()
                }; // TODO: would it be safer to match None? How could this error occur?

                RegisterTypedVertexValueToRestore::<$value_type>::register_vertex_value_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    vertex_index,
                    vertex_value_to_restore,
                );

                Ok(())
            }

            fn register_optional_vertex_value_to_restore(
                vertex_store_state_restorer: &mut VertexStoreStateRestorer,
                vertex_type_index: &impl GetVertexTypeIndex,
                vertex_vector: &VertexVector,
                vertex_index: &impl GetVertexIndexIndex,
            ) -> Result<(), GraphComputingError> {
                let vertex_value_to_restore = unsafe { <$value_type>::element_value(vertex_vector, vertex_index.index()) }?;

                match vertex_value_to_restore {
                    Some(value_to_restore) => {
                        RegisterTypedVertexValueToRestore::<$value_type>::register_vertex_value_to_restore(
                            vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                            vertex_type_index,
                            vertex_index,
                            value_to_restore,
                        );
                    },
                    None => {
                        RegisterEmptyVertexToRestore::<$value_type>::register_empty_vertex_to_restore(
                            vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                            vertex_type_index,
                            vertex_index.index(),
                        );
                    }
                }

                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_register_vertex_value_to_restore_typed);
