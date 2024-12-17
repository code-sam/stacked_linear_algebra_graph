use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::{ElementCount, GetEdgeTypeIndex};

pub(crate) trait GetAdjacencyMatrix {
    fn public_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn public_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn private_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn private_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix;

    fn adjacency_matrix_size_ref(&self) -> &ElementCount;
    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix;
}

pub(crate) trait GetAdjacencyMatrixWithCachedAttributes {
    fn public_adjacency_matrix_with_cached_attributes_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;
    fn public_adjacency_matrix_with_cached_attributes_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;

    fn private_adjacency_matrix_with_cached_attributes_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;
    fn private_adjacency_matrix_with_cached_attributes_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;

    fn adjacency_matrix_with_cached_attributes_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrixWithCachedAttributes;
    fn adjacency_matrix_with_cached_attributes_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrixWithCachedAttributes;
}
