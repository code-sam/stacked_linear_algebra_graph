use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_element::GetEdgeVerticesMask;
use crate::graph::indexing::GetEdgeTypeIndex;

impl<'s> GetEdgeVerticesMask for InMemoryEdgeStoreTransaction<'s> {
    fn select_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.edge_store_ref()
            .select_vertices_with_outgoing_edges(edge_type_index)
    }

    fn select_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.edge_store_ref()
            .select_vertices_with_incoming_edges(edge_type_index)
    }

    // TODO: wrap mask into a business struct
    fn select_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.edge_store_ref()
            .select_connected_vertices(edge_type_index)
    }

    fn select_vertices_with_outgoing_edges_using_transpose(
        &mut self,
        edge_type_index: &(impl GetEdgeTypeIndex + Debug),
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.edge_store_mut_ref()
            .select_vertices_with_outgoing_edges_using_transpose(edge_type_index)
    }

    fn select_connected_vertices_using_transpose(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.edge_store_mut_ref()
            .select_connected_vertices_using_transpose(edge_type_index)
    }
}
