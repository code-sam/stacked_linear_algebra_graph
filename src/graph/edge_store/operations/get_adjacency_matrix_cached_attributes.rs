use std::fmt::Debug;

use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{
        edge_store::{
            adjacency_matrix_with_cached_attributes::GetCachedAttributesOfAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, GetAdjacencyMatrices,
        },
        indexing::GetEdgeTypeIndex,
    },
};

pub(crate) trait GetAdjacencyMatrixCachedAttributes {
    fn try_transposed_adjacency_matrix_ref(
        &mut self,
        edge_type_index: &(impl GetEdgeTypeIndex + Debug),
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;

    fn transposed_adjacency_matrix_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
}

impl GetAdjacencyMatrixCachedAttributes for EdgeStore {
    fn transposed_adjacency_matrix_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.adjacency_matrices_mut_ref()[*edge_type_index.index_ref()]
            .transposed_weighted_adjacency_matrix_ref()
            .unwrap()
    }

    fn try_transposed_adjacency_matrix_ref(
        &mut self,
        edge_type_index: &(impl GetEdgeTypeIndex + Debug),
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        match self
            .adjacency_matrices_mut_ref()
            .get_mut(*edge_type_index.index_ref())
        {
            Some(adjacency_matrix) => {
                Ok(adjacency_matrix.transposed_weighted_adjacency_matrix_ref()?)
            }
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {:?}", edge_type_index),
                None,
            )
            .into()),
        }
    }
}
