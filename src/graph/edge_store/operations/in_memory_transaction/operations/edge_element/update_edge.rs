use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::operations::in_memory_transaction::GetEdgeStore;
use crate::graph::edge_store::operations::in_memory_transaction::InMemoryEdgeStoreTransaction;
use crate::graph::edge_store::operations::in_memory_transaction::RegisterEdgeWeightToRestore;
use crate::graph::edge_store::operations::operations::edge_element::Indexing;
use crate::graph::edge_store::operations::operations::edge_element::SetEdge;
use crate::graph::edge_store::operations::operations::edge_element::UpdateEdge;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrixWithCachedAttributes;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixStateRevertersByEdgeTypeMap;

impl<'s, T> UpdateEdge<T> for InMemoryEdgeStoreTransaction<'s>
where
    T: ValueType
        + GetEdgeTypeIndex
        + Copy
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>
        + SetSparseMatrixElementTyped<T>,
{
    fn update_weighted_directed_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.update_edge(
            vertex_indexer,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn update_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;
        self.edge_store_ref()
            .try_is_edge(edge_type_index, tail, head)?;

        self.update_edge_unchecked(edge_type_index, tail, head, weight)
    }

    fn update_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.update_edge_unchecked(
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn update_edge_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        RegisterEdgeWeightToRestore::register_edge_weight_to_restore(
            &mut self.edge_store_state_restorer,
            self.edge_store
                .adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index)?,
            edge_type_index,
            tail,
            head,
        )?;

        self.edge_store_mut_ref()
            .update_edge_unchecked(edge_type_index, tail, head, weight)
    }
}
