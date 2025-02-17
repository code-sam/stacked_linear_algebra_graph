use std::fmt::Debug;

use uuid::Uuid;

use super::GetVersionedIndex;

pub trait GetVersionedEdgeTypeIndex: GetVersionedIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VersionedEdgeTypeIndex {
    index: Uuid,
}

impl GetVersionedIndex for VersionedEdgeTypeIndex {
    fn versioned_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn versioned_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetVersionedEdgeTypeIndex for VersionedEdgeTypeIndex {}

impl VersionedEdgeTypeIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
