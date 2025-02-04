use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
    GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes,
};
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction, RegisterAdjacencyMatrixToRestore,
};
use crate::graph::edge_store::operations::operations::edge_type::map::{
    MapAdjacencyMatricesWithCachedAttributes, MapMutableAdjacencyMatrices,
};
use crate::graph::indexing::EdgeTypeIndex;

impl<'s, MappingFunction> MapAdjacencyMatricesWithCachedAttributes<MappingFunction>
    for InMemoryEdgeStoreTransaction<'s>
where
    MappingFunction: Fn(&WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    /// Apply function to all adjacency matrices
    fn map_all_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .map_all_adjacency_matrices(function_to_apply)
    }

    fn map_all_valid_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .map_all_valid_adjacency_matrices(function_to_apply)
    }
}

impl<'s, F> MapMutableAdjacencyMatrices<F> for InMemoryEdgeStoreTransaction<'s>
where
    F: Fn(&mut WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    /// Apply function to all adjacency matrices
    fn map_mut_all_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        let register_adjacency_matrix_to_restore_and_apply_function =
            |edge_type_index: &EdgeTypeIndex,
             adjacency_matrix: &mut WeightedAdjacencyMatrixWithCachedAttributes|
             -> Result<(), GraphComputingError> {
                self.edge_store_state_restorer
                    .register_updated_adjacency_matrix_to_restore(
                        edge_type_index,
                        &adjacency_matrix.weighted_adjacency_matrix_ref(),
                    )?;

                function_to_apply(adjacency_matrix)
            };

        self.edge_store.indexed_map_mut_all_adjacency_matrices(
            register_adjacency_matrix_to_restore_and_apply_function,
        )
    }

    fn map_mut_all_valid_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        let register_adjacency_matrix_to_restore_and_apply_function =
            |edge_type_index: &EdgeTypeIndex,
             adjacency_matrix: &mut WeightedAdjacencyMatrixWithCachedAttributes|
             -> Result<(), GraphComputingError> {
                self.edge_store_state_restorer
                    .register_updated_adjacency_matrix_to_restore(
                        edge_type_index,
                        &adjacency_matrix.weighted_adjacency_matrix_ref(),
                    )?;

                function_to_apply(adjacency_matrix)
            };

        self.edge_store
            .indexed_map_mut_all_valid_adjacency_matrices(
                register_adjacency_matrix_to_restore_and_apply_function,
            )
    }
}

pub(crate) fn map_mut_all_adjacency_matrices<F>(
    edge_store: &mut InMemoryEdgeStoreTransaction,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    edge_store.map_mut_all_adjacency_matrices(function_to_apply)
}

pub(crate) fn map_mut_all_valid_adjacency_matrices<F>(
    edge_store: &mut InMemoryEdgeStoreTransaction,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    edge_store.map_mut_all_valid_adjacency_matrices(function_to_apply)
}

pub(crate) fn indexed_map_mut_all_valid_adjacency_matrices<F>(
    edge_store_transaction: &mut InMemoryEdgeStoreTransaction,
    mut function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: FnMut(
            &EdgeTypeIndex,
            &mut WeightedAdjacencyMatrixWithCachedAttributes,
        ) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    let register_adjacency_matrix_to_restore_and_apply_function =
        |edge_type_index: &EdgeTypeIndex,
         adjacency_matrix: &mut WeightedAdjacencyMatrixWithCachedAttributes|
         -> Result<(), GraphComputingError> {
            edge_store_transaction
                .edge_store_state_restorer
                .register_updated_adjacency_matrix_to_restore(
                    edge_type_index,
                    &adjacency_matrix.weighted_adjacency_matrix_ref(),
                )?;

            function_to_apply(edge_type_index, adjacency_matrix)
        };

    edge_store_transaction
        .edge_store
        .indexed_map_mut_all_valid_adjacency_matrices(
            register_adjacency_matrix_to_restore_and_apply_function,
        )
}
