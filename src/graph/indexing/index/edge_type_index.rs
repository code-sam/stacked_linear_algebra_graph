use std::fmt::Debug;

use std::fmt::Display;

use super::Index;

pub trait GetEdgeTypeIndex: Debug {
    fn index_ref(&self) -> &Index;
    fn index(&self) -> Index;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct EdgeTypeIndex {
    index: Index,
}

impl GetEdgeTypeIndex for EdgeTypeIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl EdgeTypeIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}
