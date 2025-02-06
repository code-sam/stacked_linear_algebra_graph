use std::fmt::Debug;

use super::{AssignedIndex, GetAssignedIndexData, GetIndex, Index};

pub trait GetVertexIndexIndex: GetIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VertexIndex {
    index: Index,
}

impl GetIndex for VertexIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl GetVertexIndexIndex for VertexIndex {}

impl VertexIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}

impl From<AssignedIndex> for VertexIndex {
    fn from(assigned_index: AssignedIndex) -> Self {
        VertexIndex::new(assigned_index.index())
    }
}
