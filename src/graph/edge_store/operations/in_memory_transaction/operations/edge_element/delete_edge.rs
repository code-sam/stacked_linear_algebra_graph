use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::drop_sparse_matrix_element;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::GetWeightedAdjacencyMatrix;
use crate::graph::edge_store::operations::in_memory_transaction::{
    InMemoryEdgeStoreTransaction, RegisterEdgeWeightToRestore,
};
use crate::graph::edge_store::operations::operations::edge_element::DeleteEdge;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;

impl<'s> DeleteEdge for InMemoryEdgeStoreTransaction<'s> {
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

    fn delete_edge_weight(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store
            .try_edge_type_index_validity(edge_type_index)?;

        vertex_indexer.try_vertex_index_validity(tail)?;
        vertex_indexer.try_vertex_index_validity(head)?;

        self.delete_edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn delete_weight_at_unchecked_edge_coordinate(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices + Copy),
    ) -> Result<(), GraphComputingError> {
        self.delete_edge_weight_unchecked(
            edge_type_index,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }

    fn delete_edge_weight_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        let adjacency_matrix_to_delete_from = self
            .edge_store
            .adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index);
        self.edge_store_state_restorer
            .register_edge_weight_to_restore(
                adjacency_matrix_to_delete_from,
                edge_type_index,
                tail,
                head,
            )?;

        drop_sparse_matrix_element(
            adjacency_matrix_to_delete_from.weighted_adjacency_matrix_mut_ref(),
            tail.index(),
            head.index(),
        )?;
        Ok(())
    }
}
