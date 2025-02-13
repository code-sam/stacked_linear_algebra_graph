use std::fmt::Debug;

use uuid::Uuid;

use super::GetUniqueIndex;

pub trait GetUniqueVertexIndexIndex: GetUniqueIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UniqueVertexIndex {
    index: Uuid,
}

impl GetUniqueIndex for UniqueVertexIndex {
    fn unique_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn unique_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetUniqueVertexIndexIndex for UniqueVertexIndex {}

impl UniqueVertexIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
