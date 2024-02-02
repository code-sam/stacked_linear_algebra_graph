use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{
        edge::EdgeTypeIndex,
        edge_store::{
            adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStore, EdgeStoreTrait,
        },
    },
};

pub(crate) trait GetAdjacencyMatrixCachedAttributes {
    fn try_adjacency_matrix_transpose_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_transpose_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
}

impl GetAdjacencyMatrixCachedAttributes for EdgeStore {
    fn adjacency_matrix_transpose_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.adjacency_matrices_ref()[*edge_type_index].weighted_adjacency_matrix_ref()
    }

    fn try_adjacency_matrix_transpose_ref(
        &mut self,
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
}
