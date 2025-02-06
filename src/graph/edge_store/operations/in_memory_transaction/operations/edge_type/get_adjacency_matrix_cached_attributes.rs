use std::fmt::Debug;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{GetEdgeStore, InMemoryEdgeStoreTransaction};
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::indexing::GetEdgeTypeIndex;

impl<'s> GetAdjacencyMatrixCachedAttributes for InMemoryEdgeStoreTransaction<'s> {
    fn transposed_adjacency_matrix_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.edge_store_mut_ref()
            .transposed_adjacency_matrix_ref_unchecked(edge_type_index)
    }

    fn try_transposed_adjacency_matrix_ref(
        &mut self,
        edge_type_index: &(impl GetEdgeTypeIndex + Debug),
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_store_mut_ref()
            .try_transposed_adjacency_matrix_ref(edge_type_index)
    }
}
