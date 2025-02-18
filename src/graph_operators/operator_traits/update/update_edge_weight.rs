use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;


pub trait UpdateEdgeWeight<T: ValueType> {
    fn update_edge_weight_from_edge(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn update_edge_weight(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait UpdateEdgeWeightUnchecked<T: ValueType> {
    fn update_edge_weight_from_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn update_edge_weight_unchecked(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
