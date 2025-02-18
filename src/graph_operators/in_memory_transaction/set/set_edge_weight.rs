use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};

use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};

use crate::graph::edge_store::operations::in_memory_transaction::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixStateRevertersByEdgeTypeMap;
use crate::graph::edge_store::operations::operations::edge_element::SetEdge;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::graph_operators::operator_traits::set::SetEdgeWeight;
use crate::error::GraphComputingError;

impl<
        'g,
        T: ValueType
            + SetSparseMatrixElementTyped<T>
            + Copy
            + Default
            + GetSparseMatrixElementValueTyped<T>
            + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>,
    > SetEdgeWeight<T> for InMemoryGraphTransaction<'g>
{
    fn set_edge_weight_from_edge(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.set_edge_weight(
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight_ref().to_owned(),
        )
    }

    fn set_edge_weight(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_transaction.set_edge(
            &self.vertex_store_transaction,
            edge_type,
            tail,
            head,
            weight,
        )
    }
}

#[cfg(test)]
mod tests {}
