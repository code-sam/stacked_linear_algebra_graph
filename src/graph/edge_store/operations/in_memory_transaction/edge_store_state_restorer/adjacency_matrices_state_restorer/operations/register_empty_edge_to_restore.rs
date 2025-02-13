use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::graph::edge_store::operations::in_memory_transaction::adjacency_matrices_state_restorer::state_restorer_for_adjacency_matrix_with_cached_attributes::StateRestorerForAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::{AdjacencyMatricesWithCachedAttributesStateRestorer, GetAdjacencyMatrixStateReverter, GetAdjacencyMatrixStateRevertersByEdgeTypeMap};
use crate::graph::edge_store::operations::in_memory_transaction::RegisterEmptyEdgeToRestore;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::operators::transaction::in_memory::RegisterSparseMatrixChangeToRevert;

impl<T> RegisterEmptyEdgeToRestore<T> for AdjacencyMatricesWithCachedAttributesStateRestorer
where
    T: ValueType
        + Clone
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>,
{
    fn register_empty_edge_coordinate_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        edge_coordinate: &impl GetCoordinateIndices,
    ) {
        let adjacency_matrix_state_reverter: &mut StateRestorerForAdjacencyMatrixWithCachedAttributes<T> =
            self.adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(edge_type_index);
        adjacency_matrix_state_reverter.register_empty_element_to_restore(
            edge_coordinate.row_index(),
            edge_coordinate.column_index(),
        )
    }

    fn register_empty_edge_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) {
        let adjacency_matrix_state_reverter: &mut StateRestorerForAdjacencyMatrixWithCachedAttributes<T> =
        self.adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(edge_type_index);
        adjacency_matrix_state_reverter
            .register_empty_element_to_restore(tail.index(), head.index())
    }
}
