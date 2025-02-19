use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;

pub trait GetEdgeWeightVersioned<T: ValueType> {
    fn edge_weight(
        &self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetVersionedDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    // These still require valid indices
    fn edge_weight_or_default(
        &self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn edge_weight_or_default_for_coordinate(
        &self,
        edge_coordinate: &impl GetVersionedDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight(
        &self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetVersionedDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
