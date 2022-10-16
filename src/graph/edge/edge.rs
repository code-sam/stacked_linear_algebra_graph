use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::graph::graph::Graph;
use crate::graph::vertex::{VertexIndex, VertexKey};

use super::adjacency_matrix::EdgeCoordinate;
use super::edge_type::{EdgeType, EdgeTypeIndex, EdgeTypeRef};

// pub enum Edge {
//     Directed(DirectedEdge),
// }

// TODO: add constructor with indices
// TODO: consider modelling a DirectedEdge as an enum. Each variant can model a different state/representation. E.g. defintion by keys, by indices, with existing vertices, with new vertices, etc.

// enum DirectedEdge {
//     DefinedByIndicesAndKeys(DirectedEdgeDefinedByIndicesAndKeys),
//     DefinedByIndices(DirectedEdgeDefinedByIndices),
//     DefinedByKeys(DirectedEdgeDefinedByKeys)
// }

// Design note: The Vertex and Edge structs do not contain a reference to the Graph.
// Such a reference would be an immutable reference to the graph.
// Then, the graph cannot be modified as long as this edge/vertex exists.

pub(crate) trait EdgeToEdgeCoordinate {
    fn key_defined_edge_to_edge_coordinate(
        &self,
        edge: &DirectedEdgeDefinedByKeys,
    ) -> Result<EdgeCoordinate, GraphComputingError>;
    fn index_defined_edge_to_edge_coordinate(
        &self,
        edge: &DirectedEdgeDefinedByIndices,
    ) -> Result<EdgeCoordinate, GraphComputingError>;
}

impl EdgeToEdgeCoordinate for Graph {
    fn key_defined_edge_to_edge_coordinate(
        &self,
        edge: &DirectedEdgeDefinedByKeys,
    ) -> Result<EdgeCoordinate, GraphComputingError> {
        // let mut from_vertex_index = self
        //     .vertex_key_to_vertex_index_map
        //     .get(edge.originates_from_vertex());
        let from_vertex_index;
        match self
            .vertex_key_to_vertex_index_map_ref()
            .get(edge.originates_from_vertex())
        {
            Some(index) => from_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for from-vertex with key: {}",
                        edge.originates_from_vertex()
                    ),
                    None,
                )
                .into())
            }
        }
        // let to_vertex_index = self
        //     .vertex_key_to_vertex_index_map
        //     .get(edge.goes_to_vertex())
        //     .unwrap();
        let to_vertex_index;
        match self
            .vertex_key_to_vertex_index_map_ref()
            .get(edge.points_to_vertex())
        {
            Some(index) => to_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for to-vertex with key: {}",
                        edge.points_to_vertex()
                    ),
                    None,
                )
                .into())
            }
        }

        Ok(EdgeCoordinate::new(
            *from_vertex_index.index_ref(),
            *to_vertex_index.index_ref(),
        ))
    }

    fn index_defined_edge_to_edge_coordinate(
        &self,
        edge: &DirectedEdgeDefinedByIndices,
    ) -> Result<EdgeCoordinate, GraphComputingError> {
        Ok(EdgeCoordinate::new(
            edge.originates_from_vertex().index(),
            edge.points_to_vertex().index(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectedEdgeDefinedByKeys {
    from_vertex: VertexKey,
    edge_type: EdgeType,
    to_vertex: VertexKey,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectedEdgeDefinedByIndices {
    from_vertex: VertexIndex,
    edge_type: EdgeTypeIndex,
    to_vertex: VertexIndex,
}

impl DirectedEdgeDefinedByKeys {
    pub fn new(from_vertex: VertexKey, edge_type: EdgeType, to_vertex: VertexKey) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            from_vertex,
            edge_type,
            to_vertex,
        }
    }

    pub fn edge_type_ref(&self) -> &EdgeTypeRef {
        &self.edge_type.as_str()
    }
    pub fn originates_from_vertex(&self) -> &VertexKey {
        &self.from_vertex
    }
    pub fn points_to_vertex(&self) -> &VertexKey {
        &self.to_vertex
    }
}

impl DirectedEdgeDefinedByIndices {
    pub fn new(from_vertex: VertexIndex, edge_type: EdgeTypeIndex, to_vertex: VertexIndex) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            from_vertex,
            edge_type,
            to_vertex,
        }
    }

    pub fn edge_type(&self) -> &EdgeTypeIndex {
        &self.edge_type
    }
    pub fn originates_from_vertex(&self) -> &VertexIndex {
        &self.from_vertex
    }
    pub fn points_to_vertex(&self) -> &VertexIndex {
        &self.to_vertex
    }
}

// TODO: is this struct useful? What would be it's use-case?
// pub struct DirectedEdgeSpecificationUsingIndicesAndKeys {
//     edge_type_index: EdgeTypeIndex,
//     from_vertex_index: VertexIndex,
//     to_vertex_index: VertexIndex,

//     edge_type: EdgeType,
//     from_vertex_key: VertexKey,
//     to_vertex_key: VertexKey,
// }

// impl DirectedEdgeTrait for DirectedEdgeDefinedByIndices {
//     fn edge_type_ref(&self) -> &EdgeTypeRef {

//     }
//     fn edge_type_index(&self) -> &EdgeTypeIndex {
//         &self.edge_type
//     }

//     fn originates_from_vertex_with_key(&self) -> &VertexKey;
//     fn originates_from_vertex_with_index(&self) -> &VertexIndex {
//         &self.from_vertex
//     }

//     fn points_to_vertex_with_key(&self) -> &VertexKey;
//     fn points_to_vertex_with_index(&self) -> &VertexIndex {
//         &self.to_vertex
//     }
// }

// impl DirectedEdgeTrait for DirectedEdgeDefinedByKeys {
//     fn edge_type_ref(&self) -> &EdgeTypeRef;
//     fn edge_type_index(&self) -> &EdgeTypeIndex;

//     fn originates_from_vertex(&self) -> &Vertex;
//     fn originates_from_vertex_with_key(&self) -> &VertexKey;
//     fn originates_from_vertex_with_index(&self) -> &VertexIndex;

//     fn points_to_vertex(&self) -> &Vertex;
//     fn points_to_vertex_with_key(&self) -> &VertexKey;
//     fn points_to_vertex_with_index(&self) -> &VertexIndex;
// }

// // REVIEW: should an edge have a value, or even properties?
// // TODO: use const generics
// #[derive(Debug, Clone)]
// pub struct DirectedEdge {
//     // key: EdgeKey,
//     edge_type: EdgeType,
//     from_vertex: VertexKey,
//     to_vertex: VertexKey,
// }
