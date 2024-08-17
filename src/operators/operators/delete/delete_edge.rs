use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;

pub trait DeleteEdge {
    fn delete_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
    fn delete_edge_for_coordinate(
        &mut self,
        edge_to_delete: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeletePrivateEdge {
    fn delete_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
    fn delete_private_edge_for_coordinate(
        &mut self,
        edge_to_delete: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
