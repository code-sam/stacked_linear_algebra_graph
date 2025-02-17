
use crate::versioned_graph::indexing::VersionedVertexIndex;

pub struct VersionedAdjacencyMatrixCoordinate {
    tail: VersionedVertexIndex,
    head: VersionedVertexIndex,
}

impl VersionedAdjacencyMatrixCoordinate {
    pub fn new(tail: VersionedVertexIndex, head: VersionedVertexIndex) -> Self {
        Self { tail, head }
    }
}

pub trait GetVersionedAdjacencyMatrixCoordinateIndices {
    fn tail(&self) -> VersionedVertexIndex;
    fn tail_ref(&self) -> &VersionedVertexIndex;

    fn head(&self) -> VersionedVertexIndex;
    fn head_ref(&self) -> &VersionedVertexIndex;
}

impl GetVersionedAdjacencyMatrixCoordinateIndices for VersionedAdjacencyMatrixCoordinate {
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
}
