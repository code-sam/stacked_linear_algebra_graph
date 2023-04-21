use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;

use crate::graph::{
    graph::{EdgeTypeIndex, VertexIndex},
    vertex::{VertexKey, VertexKeyRef},
};

use super::{EdgeTypeKey, EdgeTypeKeyRef};

pub type AdjacencyMatrixCoordinate = Coordinate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectedEdgeCoordinateDefinedByIndices {
    edge_type: EdgeTypeIndex,
    tail: VertexIndex,
    head: VertexIndex,
}

impl DirectedEdgeCoordinateDefinedByIndices {
    pub fn new(edge_type: EdgeTypeIndex, tail: VertexIndex, head: VertexIndex) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type,
            tail,
            head,
        }
    }
}

pub trait DirectedEdgeCoordinateDefinedByIndicesTrait {
    fn edge_type_ref(&self) -> &EdgeTypeIndex;
    fn tail_ref(&self) -> &VertexIndex;
    fn head_ref(&self) -> &VertexIndex;
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> AdjacencyMatrixCoordinate;
}

impl DirectedEdgeCoordinateDefinedByIndicesTrait for DirectedEdgeCoordinateDefinedByIndices {
    fn edge_type_ref(&self) -> &EdgeTypeIndex {
        &self.edge_type
    }
    fn tail_ref(&self) -> &VertexIndex {
        &self.tail
    }
    fn head_ref(&self) -> &VertexIndex {
        &self.head
    }
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> AdjacencyMatrixCoordinate {
        AdjacencyMatrixCoordinate::new(self.tail, self.head)
    }
}

impl DirectedEdgeCoordinateDefinedByKeys {
    pub fn new(edge_type: &EdgeTypeKeyRef, tail: &VertexKeyRef, head: &VertexKeyRef) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type: edge_type.to_owned(),
            tail: tail.to_owned(),
            head: head.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectedEdgeCoordinateDefinedByKeys {
    edge_type: EdgeTypeKey,
    tail: VertexKey,
    head: VertexKey,
}

pub trait DirectedEdgeCoordinateDefinedByKeysTrait {
    fn edge_type_ref(&self) -> &EdgeTypeKeyRef;
    fn tail_ref(&self) -> &VertexKeyRef;
    fn head_ref(&self) -> &VertexKeyRef;
}

impl DirectedEdgeCoordinateDefinedByKeysTrait for DirectedEdgeCoordinateDefinedByKeys {
    fn edge_type_ref(&self) -> &EdgeTypeKeyRef {
        self.edge_type.as_str()
    }
    fn tail_ref(&self) -> &VertexKeyRef {
        self.tail.as_str()
    }
    fn head_ref(&self) -> &VertexKeyRef {
        self.head.as_str()
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
