use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixStateRevertersByEdgeTypeMap;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::traits::RegisterTypedAdjacencyMatrixToRestore;
use crate::graph::edge_store::traits::in_memory_transaction::{
    EdgeStoreStateRestorer, GetEdgeStoreStateReverters, InMemoryEdgeStoreTransaction
};
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::indexing::traits::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::value_type::{
    GetValueTypeIdentifierRef, ValueType, ValueTypeIdentifier
};
use crate::graph::edge_store::weighted_adjacency_matrix::IntoSparseMatrixAndClearValuesForValueType;
use crate::graph::edge_store::weighted_adjacency_matrix::ToSparseMatrix;
use crate::graph::weighted_adjacency_matrix::{ToSparseMatrixForValueType, WeightedAdjacencyMatrix};

pub(crate) trait RegisterAdjacencyMatrixToRestore<'t> {
    fn register_deleted_adjacency_matrix_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: &mut WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError>;

    fn register_updated_adjacency_matrix_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: &WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterAdjacencyMatrixToRestore<'t> for EdgeStoreStateRestorer {
    fn register_deleted_adjacency_matrix_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: &mut WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_state_restorer_mut_ref()
            .register_freed_index_to_restore(edge_type_index.index())?;

        self.register_deleted_adjacency_matrix_to_restore(adjacency_matrix, edge_type_index)?;
        Ok(())
    }

    fn register_updated_adjacency_matrix_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: &WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError> {
        match adjacency_matrix.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_adjacency_matrix_to_restore(self, adjacency_matrix, edge_type_index)?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
        }
        Ok(())
    }
}

impl EdgeStoreStateRestorer {
    fn register_deleted_adjacency_matrix_to_restore(
        &mut self,
        adjacency_matrix: &mut WeightedAdjacencyMatrix,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(match adjacency_matrix.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_deleted_adjacency_matrix_to_restore(
                    self,
                    adjacency_matrix,
                    edge_type_index,
                )?;
            }
        })
    }
}

pub(crate) trait RegisterUntypedAdjacencyMatrixToRestore<'t> {
    fn register_deleted_adjacency_matrix_to_restore(
        edge_store_state_restorer: &'t mut EdgeStoreStateRestorer,
        adjacency_matrix: &mut WeightedAdjacencyMatrix,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_adjacency_matrix_to_restore(
        edge_store_state_restorer: &'t mut EdgeStoreStateRestorer,
        adjacency_matrix: &WeightedAdjacencyMatrix,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t, T> RegisterUntypedAdjacencyMatrixToRestore<'t> for T
where
    T: 't
        + ValueType
        + Clone
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>
        + IntoSparseMatrixAndClearValuesForValueType<T>
        + ToSparseMatrixForValueType<T>,
{
    fn register_deleted_adjacency_matrix_to_restore(
        edge_store_state_restorer: &'t mut EdgeStoreStateRestorer,
        adjacency_matrix_to_restore: &mut WeightedAdjacencyMatrix,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let sparse_adjacency_matrix =
            <T>::into_sparse_matrix_and_clear_values(adjacency_matrix_to_restore)?;

        RegisterTypedAdjacencyMatrixToRestore::<'t, T>::register_adjacency_matrix_to_restore(
            edge_store_state_restorer.adjacency_matrices_state_restorer_mut_ref(),
            edge_type_index,
            sparse_adjacency_matrix,
        );

        Ok(())
    }

    fn register_adjacency_matrix_to_restore(
        edge_store_state_restorer: &'t mut EdgeStoreStateRestorer,
        adjacency_matrix_to_restore: &WeightedAdjacencyMatrix,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let sparse_adjacency_matrix = adjacency_matrix_to_restore.to_sparse_matrix()?;

        RegisterTypedAdjacencyMatrixToRestore::<'t, T>::register_adjacency_matrix_to_restore(
            edge_store_state_restorer.adjacency_matrices_state_restorer_mut_ref(),
            edge_type_index,
            sparse_adjacency_matrix,
        );

        Ok(())
    }
}

impl<'s> InMemoryEdgeStoreTransaction<'s> {
    fn register_updated_adjacency_matrix_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let adjacency_matrix = self.edge_store.adjacency_matrix_ref(edge_type_index)?;

        self.edge_store_state_restorer
            .register_updated_adjacency_matrix_to_restore(edge_type_index, adjacency_matrix)?;
        Ok(())
    }

    fn register_updated_adjacency_matrix_to_restore_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let adjacency_matrix = self
            .edge_store
            .adjacency_matrix_ref_unchecked(edge_type_index);

        self.edge_store_state_restorer
            .register_updated_adjacency_matrix_to_restore(edge_type_index, adjacency_matrix)?;
        Ok(())
    }
}
