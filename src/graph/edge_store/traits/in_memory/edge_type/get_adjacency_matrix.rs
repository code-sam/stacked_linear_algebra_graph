use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::{
    GetAdjacencyMatrix, GetAdjacencyMatrixWithCachedAttributes,
};
use crate::graph::edge_store::{
    adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix,
    weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, GetAdjacencyMatrices,
    GetEdgeTypeIndicer,
};
use crate::graph::indexing::{traits::CheckIndex, ElementCount, GetEdgeTypeIndex};

impl GetAdjacencyMatrix for EdgeStore {
    fn adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index())?;
        Ok(self.adjacency_matrix_ref_unchecked(edge_type_index))
    }

    fn adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index())?;
        self.adjacency_matrix_mut_ref_unchecked(edge_type_index)
    }

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.adjacency_matrices_ref()[*edge_type_index.index_ref()].weighted_adjacency_matrix_ref()
    }

    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        Ok(
            self.adjacency_matrices_mut_ref()[*edge_type_index.index_ref()]
                .weighted_adjacency_matrix_mut_ref(),
        )
    }

    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        GetAdjacencyMatrices::adjacency_matrix_size_ref(self)
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        GetAdjacencyMatrices::mask_to_select_entire_adjacency_matrix_ref(self)
    }
}

impl GetAdjacencyMatrixWithCachedAttributes for EdgeStore {
    fn adjacency_matrix_with_cached_attributes_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index())?;
        Ok(self.adjacency_matrix_with_cached_attributes_ref_unchecked(edge_type_index))
    }

    fn adjacency_matrix_with_cached_attributes_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index())?;
        self.adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index)
    }

    fn adjacency_matrix_with_cached_attributes_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrixWithCachedAttributes {
        &self.adjacency_matrices_ref()[*edge_type_index.index_ref()]
    }

    fn adjacency_matrix_with_cached_attributes_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        Ok(&mut self.adjacency_matrices_mut_ref()[*edge_type_index.index_ref()])
    }
}
