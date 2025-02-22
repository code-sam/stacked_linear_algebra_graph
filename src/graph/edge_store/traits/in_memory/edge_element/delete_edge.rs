use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    drop_sparse_matrix_element, drop_sparse_matrix_element_with_coordinate,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::traits::edge_element::{DeleteEdge, Indexing};
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;
use crate::graph::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;

impl DeleteEdge for EdgeStore {
    fn delete_weight_at_edge_coordinate(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices + Copy),
    ) -> Result<(), GraphComputingError> {
        self.delete_edge_weight(
            vertex_indexer,
            edge_type_index,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }

    fn delete_weight_at_unchecked_edge_coordinate(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + Copy),
    ) -> Result<(), GraphComputingError> {
        let adjancency_matrix_to_delete_from =
            self.adjacency_matrix_mut_ref_unchecked(edge_type_index)?;
        drop_sparse_matrix_element_with_coordinate(adjancency_matrix_to_delete_from, *coordinate)?;
        Ok(())
    }

    fn delete_edge_weight(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;

        self.delete_edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn delete_edge_weight_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        let adjancency_matrix_to_delete_from =
            self.adjacency_matrix_mut_ref_unchecked(edge_type_index)?;
        drop_sparse_matrix_element(adjancency_matrix_to_delete_from, tail.index(), head.index())?;
        Ok(())
    }
}
