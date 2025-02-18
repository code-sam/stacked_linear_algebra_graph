use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};

use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::{AdjacencyMatricesWithCachedAttributesStateRestorer, GetAdjacencyMatrixStateReverter, GetAdjacencyMatrixStateRevertersByEdgeTypeMap};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::transaction::in_memory::RegisterSparseMatrixChangeToRevert;

pub(crate) trait RegisterTypedEdgeWeightToRestore<T: ValueType> {
    fn register_edge_weight_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        edge_weight: T,
    );
}

impl<T> RegisterTypedEdgeWeightToRestore<T> for AdjacencyMatricesWithCachedAttributesStateRestorer
where
    T: ValueType
        + Clone
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>,
{
    fn register_edge_weight_to_restore(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        edge_weight: T,
    ) {
        self.adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(edge_type_index)
            .register_element_value_to_restore(tail.index(), head.index(), edge_weight)
    }
}
