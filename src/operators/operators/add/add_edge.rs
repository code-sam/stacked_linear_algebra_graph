use crate::error::GraphComputingError;
use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;

pub trait AddEdge<T: ValueType> {
    fn add_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait AddPrivateEdge<T: ValueType> {
    fn add_private_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_private_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
