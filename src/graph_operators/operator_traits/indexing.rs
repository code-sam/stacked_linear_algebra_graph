use crate::error::GraphComputingError;
use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::versioned_graph::indexing::{
    GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex,
};
use crate::versioned_graph::versioned_edge::GetVersionedDirectedEdgeCoordinateIndex;

pub trait CheckIndex {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_vertex_type_index(
        &self,
        vertex_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_edge_type_index(
        &self,
        vertex_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_optional_vertex_type_index_validity(
        &self,
        vertex_type_index: Option<&impl GetVertexTypeIndex>,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&impl GetEdgeTypeIndex>,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_edge(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn is_valid_edge_coordinate(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_validity(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_coordinate_validity(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
