use std::fmt::Debug;

use super::Index;

pub trait GetVertexIndexIndex: Debug {
    fn index_ref(&self) -> &Index;
    fn index(&self) -> Index;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VertexIndex {
    index: Index,
}

impl GetVertexIndexIndex for VertexIndex {
    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn index(&self) -> Index {
        self.index.to_owned()
    }
}

impl VertexIndex {
    pub fn new(index: Index) -> Self {
        Self { index }
    }
}
