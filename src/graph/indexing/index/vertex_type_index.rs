use std::fmt::Debug;

use super::{AssignedIndex, GetAssignedIndexData, GetIndex, Index};

pub trait GetVertexTypeIndex: GetIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Default)]
pub struct VertexTypeIndex {
    index: Index,
}

impl GetIndex for VertexTypeIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl GetVertexTypeIndex for VertexTypeIndex {}

impl VertexTypeIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}

impl From<AssignedIndex> for VertexTypeIndex {
    fn from(assigned_index: AssignedIndex) -> Self {
        VertexTypeIndex::new(assigned_index.index())
    }
}
