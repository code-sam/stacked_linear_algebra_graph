use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{
        edge::EdgeTypeIndex,
        edge_store::{
            adjacency_matrix_with_cached_attributes::GetCachedAttributesOfAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, EdgeStoreTrait,
        },
    },
};

pub(crate) trait GetAdjacencyMatrixCachedAttributes {
    fn try_transposed_adjacency_matrix_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;

    fn transposed_adjacency_matrix_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
}

impl GetAdjacencyMatrixCachedAttributes for EdgeStore {
    fn transposed_adjacency_matrix_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.adjacency_matrices_mut_ref()[*edge_type_index]
            .transposed_weighted_adjacency_matrix_ref()
            .unwrap()
    }

    fn try_transposed_adjacency_matrix_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        match self.adjacency_matrices_mut_ref().get_mut(*edge_type_index) {
            Some(adjacency_matrix) => {
                Ok(adjacency_matrix.transposed_weighted_adjacency_matrix_ref()?)
            }
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }
}
