use crate::error::GraphComputingError;
use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;

pub trait CheckVersionedIndex {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVersionedVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_vertex_type_index(
        &self,
        vertex_index: &impl GetVersionedVertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_edge_type_index(
        &self,
        vertex_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVersionedVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVersionedVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_optional_vertex_type_index_validity(
        &self,
        vertex_type_index: Option<&impl GetVersionedVertexTypeIndex>,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&impl GetVersionedEdgeTypeIndex>,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_edge(
        &self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn is_valid_edge_coordinate(
        &self,
        edge: &impl GetVersionedDirectedEdgeCoordinateIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_validity(
        &self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
        tail: &impl GetVersionedVertexIndexIndex,
        head: &impl GetVersionedVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_coordinate_validity(
        &self,
        edge: &impl GetVersionedDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
