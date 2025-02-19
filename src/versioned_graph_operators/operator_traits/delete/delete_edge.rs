use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::GetVersionedVertexIndexIndex;

pub trait DeleteEdgeVersioned {
    fn delete_edge(
        &mut self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_edge_for_coordinate(
        &mut self,
        edge_to_delete: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
