use crate::error::GraphComputingError;
use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;

pub trait NewEdgeVersioned<T: ValueType> {
    fn new_edge_from_edge(
        &mut self,
        edge: impl GetVersionedDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn new_edge(
        &mut self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

pub trait NewEdge<T: ValueType> {
    fn new_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn new_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
