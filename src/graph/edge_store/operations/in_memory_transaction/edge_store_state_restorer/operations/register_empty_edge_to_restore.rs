use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::graph::edge_store::operations::in_memory_transaction::{EdgeStoreStateRestorer, GetEdgeStoreStateReverters};
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixStateRevertersByEdgeTypeMap;
use crate::graph::value_type::ValueType;
use crate::graph::indexing::GetEdgeTypeIndex;

pub(crate) trait RegisterEmptyEdgeToRestore<T: ValueType> {
    fn register_empty_edge_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        edge_coordinate: &impl GetCoordinateIndices,
    );
}

impl<T> RegisterEmptyEdgeToRestore<T> for EdgeStoreStateRestorer
where
    T: ValueType
        + Clone
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>,
{
    fn register_empty_edge_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        edge_coordinate: &impl GetCoordinateIndices,
    ) {
        RegisterEmptyEdgeToRestore::<T>::register_empty_edge_to_restore(
            self.adjacency_matrices_state_restorer_mut_ref(),
            edge_type_index,
            edge_coordinate,
        );
    }
}
