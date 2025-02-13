use std::fmt::Debug;

use uuid::Uuid;

use crate::graph::indexing::Index;

use super::GetUniqueIndex;

pub trait GetUniqueEdgeTypeIndex: GetUniqueIndex {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UniqueEdgeTypeIndex {
    index: Uuid,
}

impl GetUniqueIndex for UniqueEdgeTypeIndex {
    fn unique_index_ref(&self) -> &Uuid {
        &self.index
    }

    fn unique_index(&self) -> Uuid {
        self.index.to_owned()
    }
}

impl GetUniqueEdgeTypeIndex for UniqueEdgeTypeIndex {}

impl UniqueEdgeTypeIndex {
    pub fn new(index: Uuid) -> Self {
        Self { index }
    }
}
