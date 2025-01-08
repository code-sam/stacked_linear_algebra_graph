use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::{
    GetAdjacencyMatrix, GetAdjacencyMatrixWithCachedAttributes,
};
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::{ElementCount, GetEdgeTypeIndex};

impl<'s> GetAdjacencyMatrix for InMemoryEdgeStoreTransaction<'s> {
    fn adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_store_ref().adjacency_matrix_ref(edge_type_index)
    }

    fn adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        self.edge_store_mut_ref()
            .adjacency_matrix_mut_ref(edge_type_index)
    }

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type_index)
    }

    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix {
        self.edge_store_mut_ref()
            .adjacency_matrix_mut_ref_unchecked(edge_type_index)
    }

    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        GetAdjacencyMatrix::adjacency_matrix_size_ref(self.edge_store_ref())
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        GetAdjacencyMatrix::mask_to_select_entire_adjacency_matrix_ref(self.edge_store_ref())
    }
}

impl<'s> GetAdjacencyMatrixWithCachedAttributes for InMemoryEdgeStoreTransaction<'s> {
    fn adjacency_matrix_with_cached_attributes_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        self.edge_store_ref()
            .adjacency_matrix_with_cached_attributes_ref(edge_type_index)
    }

    fn adjacency_matrix_with_cached_attributes_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        self.edge_store_mut_ref()
            .adjacency_matrix_with_cached_attributes_mut_ref(edge_type_index)
    }

    fn adjacency_matrix_with_cached_attributes_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrixWithCachedAttributes {
        self.edge_store_ref()
            .adjacency_matrix_with_cached_attributes_ref_unchecked(edge_type_index)
    }

    fn adjacency_matrix_with_cached_attributes_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrixWithCachedAttributes {
        self.edge_store_mut_ref()
            .adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index)
    }
}
