use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            adjacency_matrix_with_cached_attributes::{
                GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes,
            },
            weighted_adjacency_matrix::WeightedAdjacencyMatrix,
            EdgeStore, GetAdjacencyMatrices, GetEdgeTypeIndicer,
        },
        indexing::operations::{GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices},
    },
};

pub(crate) trait MapAdjacencyMatricesWithCachedAttributes<MappingFunction> {
    fn map_all_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError>;

    fn map_all_valid_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError>;

    // fn map_all_valid_public_adjacency_matrices(
    //     &self,
    //     function_to_apply: MappingFunction,
    // ) -> Result<(), GraphComputingError>;

    // fn map_all_valid_private_adjacency_matrices(
    //     &self,
    //     function_to_apply: MappingFunction,
    // ) -> Result<(), GraphComputingError>;
}

pub(crate) trait MapMutableAdjacencyMatrices<F> {
    fn map_mut_all_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;

    fn map_mut_all_valid_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;

    fn map_mut_all_valid_public_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;

    fn map_mut_all_valid_private_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;
}

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

    // fn map_all_valid_public_adjacency_matrices(
    //     &self,
    //     function_to_apply: MappingFunction,
    // ) -> Result<(), GraphComputingError> {
    //     // TODO: would par_iter() give better performance?
    //     self.edge_type_indexer_ref()
    //         .valid_public_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| function_to_apply(&self.adjacency_matrices_ref()[i]))?;
    //     Ok(())
    // }

    // fn map_all_valid_private_adjacency_matrices(
    //     &self,
    //     function_to_apply: MappingFunction,
    // ) -> Result<(), GraphComputingError> {
    //     // TODO: would par_iter() give better performance?
    //     self.edge_type_indexer_ref()
    //         .valid_private_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| function_to_apply(&self.adjacency_matrices_ref()[i]))?;
    //     Ok(())
    // }
}

impl<F> MapMutableAdjacencyMatrices<F> for EdgeStore
where
    F: Fn(&mut WeightedAdjacencyMatrix) -> Result<(), GraphComputingError> + Send + Sync,
{
    /// Apply function to all adjacency matrices
    fn map_mut_all_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrices_mut_ref()
            .into_par_iter()
            .try_for_each(|adjacency_matrix| {
                function_to_apply(adjacency_matrix.weighted_adjacency_matrix_mut_ref())
            })?;
        Ok(())
    }

    fn map_mut_all_valid_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        // TODO: would par_iter() give better performance?
        self.edge_type_indexer_ref()
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(
                    &mut self.adjacency_matrices_mut_ref()[i].weighted_adjacency_matrix_mut_ref(),
                )
            })?;
        Ok(())
    }

    fn map_mut_all_valid_public_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        // TODO: would par_iter() give better performance?
        self.edge_type_indexer_mut_ref()
            .valid_public_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(
                    &mut self.adjacency_matrices_mut_ref()[i].weighted_adjacency_matrix_mut_ref(),
                )
            })?;
        Ok(())
    }

    fn map_mut_all_valid_private_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError> {
        // TODO: would par_iter() give better performance?
        self.edge_type_indexer_mut_ref()
            .valid_private_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(
                    &mut self.adjacency_matrices_mut_ref()[i].weighted_adjacency_matrix_mut_ref(),
                )
            })?;
        Ok(())
    }
}
