use crate::graph::graph::ElementIndex;

use crate::graph::indexed_data_store::index::Index as IndexedDataStoreIndex;

// TODO: change to EdgeTypeKey?
pub type EdgeType = String;
pub type EdgeTypeRef = str;

// pub enum Edge {
//     Directed(DirectedEdge),
// }

// TODO: add constructor with indices
// TODO: consider modelling a DirectedEdge as an enum. Each variant can model a different state/representation. E.g. defintion by keys, by indices, with existing vertices, with new vertices, etc.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeTypeIndex {
    index: IndexedDataStoreIndex,
}

impl EdgeTypeIndex {
    pub(crate) fn new(index: IndexedDataStoreIndex) -> Self {
        EdgeTypeIndex { index }
    }
    pub(crate) fn index(self) -> ElementIndex {
        self.index
    }
    pub(crate) fn index_ref(&self) -> &IndexedDataStoreIndex {
        &self.index
    }
}
