use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::traits::traits::edge_type::map::{
    MapAdjacencyMatricesWithCachedAttributes, MapMutableAdjacencyMatrices,
};
use crate::graph::edge_store::{EdgeStore, GetAdjacencyMatrices, GetEdgeTypeIndicer};
use crate::graph::indexing::traits::GetValidIndices;
use crate::graph::indexing::EdgeTypeIndex;

impl<MappingFunction> MapAdjacencyMatricesWithCachedAttributes<MappingFunction> for EdgeStore
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
        self.adjacency_matrices_ref()
            .into_par_iter()
            .try_for_each(|adjacency_matrix| function_to_apply(adjacency_matrix))?;
        Ok(())
    }

    fn map_all_valid_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError> {
        // TODO: would par_iter() give better performance?
        self.edge_type_indexer_ref()
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| function_to_apply(&self.adjacency_matrices_ref()[i]))?;
        Ok(())
    }
}

impl<F> MapMutableAdjacencyMatrices<F> for EdgeStore
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
        self.adjacency_matrices_mut_ref()
            .into_par_iter()
            .try_for_each(|adjacency_matrix| function_to_apply(adjacency_matrix))?;
        Ok(())
    }

    fn map_mut_all_valid_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_adjacency_matrices(function_to_apply)
    }
}

pub(crate) fn map_mut_all_adjacency_matrices<F>(
    edge_store: &mut EdgeStore,
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
    edge_store: &mut EdgeStore,
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
    edge_store: &mut EdgeStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: FnMut(
            &EdgeTypeIndex,
            &mut WeightedAdjacencyMatrixWithCachedAttributes,
        ) -> Result<(), GraphComputingError>
        + Send
        + Sync,
{
    edge_store.indexed_map_mut_all_valid_adjacency_matrices(function_to_apply)
}
