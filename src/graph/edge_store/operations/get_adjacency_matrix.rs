use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, GetAdjacencyMatrices,
            GetEdgeTypeIndicer,
        },
        indexing::{operations::CheckIndex, ElementCount, GetEdgeTypeIndex},
    },
};

pub(crate) trait GetAdjacencyMatrix {
    fn try_public_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn try_public_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn try_private_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn try_private_adjacency_matrix_mut_ref(
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

impl GetAdjacencyMatrix for EdgeStore {
    fn try_public_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_public_index(edge_type_index.index())?;
        Ok(self.adjacency_matrix_ref_unchecked(edge_type_index))
    }

    fn try_public_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_public_index(edge_type_index.index())?;
        Ok(self.adjacency_matrix_mut_ref_unchecked(edge_type_index))
    }

    fn try_private_adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_private_index(edge_type_index.index())?;
        Ok(self.adjacency_matrix_ref_unchecked(edge_type_index))
    }

    fn try_private_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_private_index(edge_type_index.index())?;
        Ok(self.adjacency_matrix_mut_ref_unchecked(edge_type_index))
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
    ) -> &mut WeightedAdjacencyMatrix {
        self.adjacency_matrices_mut_ref()[*edge_type_index.index_ref()]
            .weighted_adjacency_matrix_mut_ref()
    }

    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        GetAdjacencyMatrices::adjacency_matrix_size_ref(self)
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        GetAdjacencyMatrices::mask_to_select_entire_adjacency_matrix_ref(self)
    }
}
