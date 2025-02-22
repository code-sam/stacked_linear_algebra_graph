use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};

use crate::graph::edge_store::traits::traits::edge_element::SetEdge;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::graph_operators::operator_traits::set::SetEdgeWeight;
use crate::{error::GraphComputingError, graph::graph::Graph};

impl<T: ValueType + SetSparseMatrixElementTyped<T> + Copy> SetEdgeWeight<T> for Graph {
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
        self.public_edge_store
            .set_edge(&self.public_vertex_store, edge_type, tail, head, weight)
    }
}

#[cfg(test)]
mod tests {}
