use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;

use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::{AdjacencyMatricesWithCachedAttributesStateRestorer, GetAdjacencyMatrixStateReverter, GetAdjacencyMatrixStateRevertersByEdgeTypeMap};
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::value_type::ValueType;
use crate::operators::in_memory_transaction::transaction::RegisterSparseMatrixChangeToRevert;

pub(crate) trait RegisterTypedAdjacencyMatrixToRestore<'a, T: ValueType> {
    fn register_adjacency_matrix_to_restore(
        &'a mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: SparseMatrix<T>,
    );
}

impl<'a, T> RegisterTypedAdjacencyMatrixToRestore<'a, T>
    for AdjacencyMatricesWithCachedAttributesStateRestorer
where
    T: 'a
        + ValueType
        + Clone
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>,
{
    fn register_adjacency_matrix_to_restore(
        &'a mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        adjacency_matrix: SparseMatrix<T>,
    ) {
        self.adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(edge_type_index)
            .register_sparse_matrix_state_to_restore(adjacency_matrix)
    }
}
