use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;

pub trait UpdateEdgeWeightVersioned<T: ValueType> {
    fn update_edge_weight_from_edge(
        &mut self,
        edge: &(impl GetVersionedDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn update_edge_weight(
        &mut self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
