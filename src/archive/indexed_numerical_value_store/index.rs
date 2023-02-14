use std::fmt::Debug;

use graphblas_sparse_linear_algebra::util::ElementIndex;

use crate::graph::edge::EdgeTypeIndex;
use crate::graph::vertex::VertexIndex;

// DESIGN NOTE: The IndexedDataStoreIndex is relatively private to signal that a user is not expected to generate index values on his/her own.
// However, the current implementation does not include cryptographic signing, or similar, to prevent a user from
// generating IndexedDataStoreIndex instances by forging them in memory.

pub type Index = ElementIndex;

pub(crate) trait IndexTrait {
    fn index_ref(&self) -> &Index;
    fn index(self) -> Index;
}

// TODO: review what the value of this abstraction is. What is the performance overhead, and is the overhead necessary?
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct IndexedDataStoreIndex {
    index: Index,
}

impl IndexedDataStoreIndex {
    pub(super) fn new(index: Index) -> Self {
        Self { index }
    }
}

impl IndexTrait for IndexedDataStoreIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }
    fn index(self) -> Index {
        self.index
    }
}

impl IndexTrait for VertexIndex {
    fn index(self) -> Index {
        self.index()
    }
    fn index_ref(&self) -> &Index {
        self.index_ref()
    }
}
impl From<VertexIndex> for IndexedDataStoreIndex {
    fn from(vertex_index: VertexIndex) -> Self {
        Self {
            index: vertex_index.index(),
        }
    }
}
impl From<IndexedDataStoreIndex> for VertexIndex {
    fn from(index: IndexedDataStoreIndex) -> Self {
        Self::new(index.index())
    }
}

impl IndexTrait for EdgeTypeIndex {
    fn index(self) -> Index {
        self.index()
    }
    fn index_ref(&self) -> &Index {
        self.index_ref()
    }
}
impl From<EdgeTypeIndex> for IndexedDataStoreIndex {
    fn from(index: EdgeTypeIndex) -> Self {
        Self {
            index: index.index(),
        }
    }
}
impl From<IndexedDataStoreIndex> for EdgeTypeIndex {
    fn from(index: IndexedDataStoreIndex) -> Self {
        Self::new(index.index())
    }
}
