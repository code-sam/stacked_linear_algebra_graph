use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::graph::edge::{EdgeTypeIndex, GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::UpdateEdgeWeight as UpdateEdgeWeightInEdgeStore;

use crate::graph::graph::VertexIndex;
use crate::graph::value_type::ValueType;
use crate::{
    error::GraphComputingError,
    graph::graph::{Graph, GraphTrait},
};

// REVIEW update vs set
pub trait UpdateEdgeWeight<T: ValueType> {
    fn update_edge_weight_from_edge(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn update_edge_weight(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SetSparseMatrixElementTyped<T> + Copy> UpdateEdgeWeight<T> for Graph {
    fn update_edge_weight_from_edge(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.update_edge_weight(
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight_ref().to_owned(),
        )
    }

    fn update_edge_weight(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_mut_ref()
            .try_adjacency_matrix_mut_ref(edge_type)?
            .update_edge_weight_unchecked(tail, head, weight)
    }
}

#[cfg(test)]
mod tests {}
