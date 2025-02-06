use std::fmt::Debug;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::GetEdgeTypeIndex;

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
