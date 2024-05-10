use std::fmt::Debug;

use super::Index;

pub trait GetVertexTypeIndex: Debug {
    fn index_ref(&self) -> &Index;
    fn index(&self) -> Index;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VertexTypeIndex {
    index: Index,
}

impl GetVertexTypeIndex for VertexTypeIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl VertexTypeIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}
