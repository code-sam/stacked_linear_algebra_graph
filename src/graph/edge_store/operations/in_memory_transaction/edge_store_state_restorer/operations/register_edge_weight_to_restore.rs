use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueUntyped;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::operations::in_memory_transaction::EdgeStoreStateRestorer;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::operations::RegisterTypedEdgeWeightToRestore;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, GetValueTypeIdentifierRef, ValueTypeIdentifier,
};
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::edge_store_state_restorer::GetEdgeStoreStateReverters;

pub(crate) trait RegisterEdgeWeightToRestore {
    fn register_edge_weight_to_restore(
        &mut self,
        adjacency_matrix_with_value_to_restore: &WeightedAdjacencyMatrixWithCachedAttributes,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterEdgeWeightToRestore for EdgeStoreStateRestorer {
    fn register_edge_weight_to_restore(
        &mut self,
        adjacency_matrix_with_value_to_restore: &WeightedAdjacencyMatrixWithCachedAttributes,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        match adjacency_matrix_with_value_to_restore.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_edge_weight_to_restore(
                    self,
                    edge_type_index,
                    adjacency_matrix_with_value_to_restore,
                    tail,
                    head,
                )?;
            }
        }
        Ok(())
    }
}

pub(crate) trait RegisterEdgeWeightToRestoreTyped {
    fn register_edge_weight_to_restore(
        edge_store_state_restorer: &mut EdgeStoreStateRestorer,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix_with_value_to_restore: &WeightedAdjacencyMatrixWithCachedAttributes,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_edge_weight_to_restore_typed {
    ($value_type:ty) => {
        impl RegisterEdgeWeightToRestoreTyped for $value_type {
            fn register_edge_weight_to_restore(
                edge_store_state_restorer: &mut EdgeStoreStateRestorer,
                edge_type_index: &impl GetEdgeTypeIndex,
                adjacency_matrix_with_value_to_restore: &WeightedAdjacencyMatrixWithCachedAttributes,
                tail: &impl GetVertexIndexIndex,
                head: &impl GetVertexIndexIndex,
            ) -> Result<(), GraphComputingError> {
                let edge_weight_to_restore = unsafe {
                    <$value_type>::element_value(
                        adjacency_matrix_with_value_to_restore.weighted_adjacency_matrix_ref(),
                        tail.index(),
                        head.index(),
                    )?
                    .unwrap()
                }; // TODO: would it be safer to match None? How could this error occur?

                RegisterTypedEdgeWeightToRestore::<$value_type>::register_edge_weight_to_restore(
                    edge_store_state_restorer.adjacency_matrices_state_restorer_mut_ref(),
                    edge_type_index,
                    tail,
                    head,
                    edge_weight_to_restore,
                );

                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_register_edge_weight_to_restore_typed);
