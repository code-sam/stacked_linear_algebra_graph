use std::fmt::Debug;

use super::{GetIndex, Index};

pub trait GetEdgeTypeIndex: GetIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct EdgeTypeIndex {
    index: Index,
}

impl GetIndex for EdgeTypeIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl GetEdgeTypeIndex for EdgeTypeIndex {}

impl EdgeTypeIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}
