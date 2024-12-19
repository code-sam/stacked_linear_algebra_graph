use crate::error::GraphComputingError;

use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;

pub(crate) trait AddEdge<T: ValueType> {
    fn add_public_weighted_directed_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn add_public_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_private_weighted_directed_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn add_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_weighted_directed_edge_unchecked(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn add_edge_unchecked(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}
