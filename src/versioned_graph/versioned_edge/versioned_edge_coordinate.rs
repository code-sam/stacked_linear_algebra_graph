
use crate::versioned_graph::indexing::{VersionedEdgeTypeIndex, VersionedVertexIndex};

use super::VersionedAdjacencyMatrixCoordinate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VersionedDirectedEdgeCoordinate {
    edge_type: VersionedEdgeTypeIndex,
    tail: VersionedVertexIndex,
    head: VersionedVertexIndex,
}

impl VersionedDirectedEdgeCoordinate {
    pub fn new(edge_type: VersionedEdgeTypeIndex, tail: VersionedVertexIndex, head: VersionedVertexIndex) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type,
            tail,
            head,
        }
    }
}

pub trait GetVersionedDirectedEdgeCoordinateIndex {
    fn edge_type_ref(&self) -> &VersionedEdgeTypeIndex;
    fn tail(&self) -> VersionedVertexIndex;
    fn tail_ref(&self) -> &VersionedVertexIndex;
    fn head(&self) -> VersionedVertexIndex;
    fn head_ref(&self) -> &VersionedVertexIndex;
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> VersionedAdjacencyMatrixCoordinate;
}

impl GetVersionedDirectedEdgeCoordinateIndex for VersionedDirectedEdgeCoordinate {
    fn edge_type_ref(&self) -> &VersionedEdgeTypeIndex {
        &self.edge_type
    }
    fn tail(&self) -> VersionedVertexIndex {
        self.tail.to_owned()
    }
    fn tail_ref(&self) -> &VersionedVertexIndex {
        &self.tail
    }
    fn head(&self) -> VersionedVertexIndex {
        self.head.to_owned()
    }
    fn head_ref(&self) -> &VersionedVertexIndex {
        &self.head
    }
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> VersionedAdjacencyMatrixCoordinate {
        VersionedAdjacencyMatrixCoordinate::new(self.tail, self.head)
    }
}
