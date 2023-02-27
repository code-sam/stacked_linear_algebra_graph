use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;

use crate::graph::{
    graph::{EdgeTypeIndex, VertexIndex},
    vertex::VertexKey,
};

use super::{EdgeTypeKey, EdgeTypeKeyRef};

pub type AdjacencyMatrixCoordinate = Coordinate;

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeCoordinateDefinedByKeys {
    edge_type: EdgeTypeKey,
    from_vertex: VertexKey,
    to_vertex: VertexKey,
}

// TODO: Wikipedia uses head and tail for tp and from respectively

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeCoordinateDefinedByIndices {
    edge_type: EdgeTypeIndex,
    from_vertex: VertexIndex,
    to_vertex: VertexIndex,
}

impl EdgeCoordinateDefinedByKeys {
    pub fn new(from_vertex: VertexKey, edge_type: EdgeTypeKey, to_vertex: VertexKey) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type,
            from_vertex,
            to_vertex,
        }
    }

    pub fn edge_type_ref(&self) -> &EdgeTypeKeyRef {
        &self.edge_type.as_str()
    }
    pub fn originates_from_vertex(&self) -> &VertexKey {
        &self.from_vertex
    }
    pub fn points_to_vertex(&self) -> &VertexKey {
        &self.to_vertex
    }
}

impl EdgeCoordinateDefinedByIndices {
    pub fn new(edge_type: EdgeTypeIndex, from_vertex: VertexIndex, to_vertex: VertexIndex) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type,
            from_vertex,
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
    // TODO: consider caching
    pub fn adjacency_matrix_coordinate_ref(&self) -> AdjacencyMatrixCoordinate {
        AdjacencyMatrixCoordinate::new(self.from_vertex, self.to_vertex)
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
