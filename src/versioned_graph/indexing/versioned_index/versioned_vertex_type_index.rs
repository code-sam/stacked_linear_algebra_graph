use std::fmt::Debug;

use uuid::Uuid;

use super::GetVersionedIndex;


pub trait GetVersionedVertexTypeIndex: GetVersionedIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Default)]
pub struct VersionedVertexTypeIndex {
    index: Uuid,
}

impl GetVersionedIndex for VersionedVertexTypeIndex {
    fn versioned_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn versioned_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetVersionedVertexTypeIndex for VersionedVertexTypeIndex {}

impl VersionedVertexTypeIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
