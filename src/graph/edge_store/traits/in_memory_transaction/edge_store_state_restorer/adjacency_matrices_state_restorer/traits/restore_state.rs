use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::resize_sparse_matrix;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes};
use crate::graph::edge_store::traits::in_memory_transaction::adjacency_matrices_state_restorer::state_restorer_for_adjacency_matrix_with_cached_attributes::StateRestorerForAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixSizeToRestore;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixVectorLengthToRestore;
use crate::graph::edge_store::traits::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::AdjacencyMatricesWithCachedAttributesStateRestorer;
use crate::graph::indexing::ElementIndexMap;
use crate::graph::value_type::ValueType;
use crate::transaction::RestoreState;

impl RestoreState<Vec<WeightedAdjacencyMatrixWithCachedAttributes>>
    for AdjacencyMatricesWithCachedAttributesStateRestorer
{
    fn restore(
        self,
        adjacency_matrices_to_restore: &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes>,
    ) -> Result<(), crate::error::GraphComputingError> {
        restore_weighted_adjacency_matrices_state(self, adjacency_matrices_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_edge_type_length_and_adjacency_matrix_size_to_restore(
            self.adjacency_matrix_vector_length_to_restore(),
            self.adjacency_matrix_size_to_restore(),
        )
    }
}

fn restore_weighted_adjacency_matrices_state(
    adjacency_matrix_with_cached_attributes_state_restorer: AdjacencyMatricesWithCachedAttributesStateRestorer,
    adjacency_matrices_to_restore: &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes>,
) -> Result<(), crate::error::GraphComputingError> {
    let adjacency_matrix_vector_length_to_restore =
        adjacency_matrix_with_cached_attributes_state_restorer
            .adjacency_matrix_vector_length_to_restore
            .clone();

    let adjacency_matrix_size_to_restore = adjacency_matrix_with_cached_attributes_state_restorer
        .adjacency_matrix_size_to_restore
        .clone();

    let adjacency_matrix_state_reverters =
        adjacency_matrix_with_cached_attributes_state_restorer.adjacency_matrix_state_reverters;

    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_bool,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_i8,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_i16,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_i32,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_i64,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_u8,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_u16,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_u32,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_u64,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_f32,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_f64,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_isize,
        adjacency_matrices_to_restore,
    )?;
    restore_weighted_adjacency_matrices(
        adjacency_matrix_state_reverters.adjacency_matrix_state_reverters_usize,
        adjacency_matrices_to_restore,
    )?;

    adjacency_matrices_to_restore.truncate(adjacency_matrix_vector_length_to_restore);

    restore_adjacency_matrix_size(
        adjacency_matrix_size_to_restore,
        adjacency_matrices_to_restore,
    )?;

    Ok(())
}

fn restore_weighted_adjacency_matrices<T>(
    typed_adjacency_matrix_state_reverters_for_edge_type: ElementIndexMap<
        StateRestorerForAdjacencyMatrixWithCachedAttributes<T>,
    >,
    weighted_adjacency_matrices_to_restore: &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes>,
) -> Result<(), GraphComputingError>
where
    T: ValueType,
    StateRestorerForAdjacencyMatrixWithCachedAttributes<T>:
        RestoreState<WeightedAdjacencyMatrixWithCachedAttributes>,
{
    for (edge_type_index, adjacency_matrix_state_reverter) in
        typed_adjacency_matrix_state_reverters_for_edge_type.into_iter()
    {
        adjacency_matrix_state_reverter
            .restore(&mut weighted_adjacency_matrices_to_restore[edge_type_index])?;
    }
    Ok(())
}

fn restore_adjacency_matrix_size(
    adjacency_matrix_size_to_restore: Option<Size>,
    adjacency_matrices_to_restore: &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes>,
) -> Result<(), GraphComputingError> {
    Ok(match adjacency_matrix_size_to_restore {
        Some(size_to_restore) => {
            // TODO: consider resizing in parallel
            for adjacency_matrix in adjacency_matrices_to_restore.iter_mut() {
                resize_sparse_matrix(
                    adjacency_matrix.weighted_adjacency_matrix_mut_ref(),
                    size_to_restore,
                )?;
            }
        }
        None => (),
    })
}
