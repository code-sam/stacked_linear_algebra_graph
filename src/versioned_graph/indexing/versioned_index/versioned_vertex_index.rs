use std::fmt::Debug;

use uuid::Uuid;

use super::GetVersionedIndex;

pub trait GetVersionedVertexIndexIndex: GetVersionedIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VersionedVertexIndex {
    index: Uuid,
}

impl GetVersionedIndex for VersionedVertexIndex {
    fn versioned_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn versioned_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetVersionedVertexIndexIndex for VersionedVertexIndex {}

impl VersionedVertexIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
