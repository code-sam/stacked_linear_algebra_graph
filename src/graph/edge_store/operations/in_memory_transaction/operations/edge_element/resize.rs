use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_element::ResizeWeightedAdjacencyMatrix as ResizeWeightedAdjacencyMatrixForEdgeType;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::ResizeWeightedAdjacencyMatrix;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::ElementCount;
use crate::graph::indexing::GetEdgeTypeIndex;

impl ResizeWeightedAdjacencyMatrixForEdgeType for EdgeStore {
    fn resize(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_mut_ref(edge_type_index)?
            .resize(new_vertex_capacity)
    }
}
