use std::fmt::Debug;

use uuid::Uuid;

use super::GetUniqueIndex;

pub trait GetUniqueVertexTypeIndex: GetUniqueIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Default)]
pub struct VertexTypeIndex {
    index: Uuid,
}

impl GetUniqueIndex for VertexTypeIndex {
    fn unique_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn unique_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetUniqueVertexTypeIndex for VertexTypeIndex {}

impl VertexTypeIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
