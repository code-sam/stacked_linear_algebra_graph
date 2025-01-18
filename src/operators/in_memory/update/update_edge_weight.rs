use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};

use crate::graph::edge_store::operations::operations::edge_element::UpdateEdge;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::operators::operators::update::UpdateEdgeWeight;
use crate::{error::GraphComputingError, graph::graph::Graph};

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
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.public_edge_store
            .update_edge(&self.public_vertex_store, edge_type, tail, head, weight)
    }
}

#[cfg(test)]
mod tests {}
