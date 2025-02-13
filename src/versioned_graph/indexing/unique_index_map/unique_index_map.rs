use hashbrown::HashMap;
use uuid::Uuid;

use crate::graph::indexing::{BuildIndexHasher, ElementCount, Index};

pub(crate) struct UniqueIndexMap {
    unique_index_to_index_map: HashMap<Uuid, Index, BuildIndexHasher>,
    index_to_unique_index_map: Vec<Uuid>,
}

impl UniqueIndexMap {
    pub(crate) fn with_initial_capacity(initial_capacity: ElementCount) -> Self {
        Self {
            unique_index_to_index_map: HashMap::with_capacity_and_hasher(
                initial_capacity,
                BuildIndexHasher::default(),
            ),
            index_to_unique_index_map: Vec::with_capacity(initial_capacity),
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            unique_index_to_index_map: HashMap::default(),
            index_to_unique_index_map: Vec::new(),
        }
    }
}
