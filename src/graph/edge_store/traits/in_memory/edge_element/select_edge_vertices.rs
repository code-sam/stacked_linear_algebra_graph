use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::traits::edge_element::GetEdgeVerticesMask;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::weighted_adjacency_matrix::traits::{select_connected_vertices, SelectEdgeVertices};

impl GetEdgeVerticesMask for EdgeStore {
    fn select_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .select_vertices_with_outgoing_edges()
    }

    fn select_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .select_vertices_with_incoming_edges()
    }

    // TODO: wrap mask into a business struct
    fn select_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .select_connected_vertices()
    }

    fn select_vertices_with_outgoing_edges_using_transpose(
        &mut self,
        edge_type_index: &(impl GetEdgeTypeIndex + Debug),
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        self.try_transposed_adjacency_matrix_ref(edge_type_index)?
            .select_vertices_with_incoming_edges()
    }

    fn select_connected_vertices_using_transpose(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let vertices_with_incoming_edges =
            self.select_vertices_with_incoming_edges(edge_type_index)?;
        let vertices_with_outgoing_edges = self
            .transposed_adjacency_matrix_ref_unchecked(edge_type_index)
            .select_vertices_with_incoming_edges()?;

        select_connected_vertices(&vertices_with_incoming_edges, &vertices_with_outgoing_edges)
    }
}
