use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{
        edge::EdgeTypeIndex,
        edge_store::{
            adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, GetAdjacencyMatrices,
        },
        index::ElementCount,
    },
    operators::options::{self, GetOperatorOptions, OperatorOptions},
};

pub(crate) trait GetAdjacencyMatrix {
    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix;

    fn adjacency_matrix_size_ref(&self) -> &ElementCount;
    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix;
}

impl GetAdjacencyMatrix for EdgeStore {
    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.adjacency_matrices_ref()[*edge_type_index].weighted_adjacency_matrix_ref()
    }

    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix {
        self.adjacency_matrices_mut_ref()[*edge_type_index].weighted_adjacency_matrix_mut_ref()
    }

    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        match self.adjacency_matrices_ref().get(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix.weighted_adjacency_matrix_ref()),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }

    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        match self.adjacency_matrices_mut_ref().get_mut(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix.weighted_adjacency_matrix_mut_ref()),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }

    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        GetAdjacencyMatrices::adjacency_matrix_size_ref(self)
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        GetAdjacencyMatrices::mask_to_select_entire_adjacency_matrix_ref(self)
    }
}
