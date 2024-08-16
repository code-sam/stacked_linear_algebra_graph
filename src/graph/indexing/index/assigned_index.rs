use super::index::{ElementCount, Index};

#[derive(Clone, Debug)]
pub(crate) struct AssignedIndex {
    index: Index,
    new_index_capacity: Option<ElementCount>,
    is_reused: bool,
}

impl AssignedIndex {
    pub(crate) fn new(
        index: Index,
        new_index_capacity: Option<ElementCount>,
        is_reused: bool,
    ) -> Self {
        Self {
            index,
            new_index_capacity,
            is_reused,
        }
    }
}

pub(crate) trait GetAssignedIndexData {
    fn index(&self) -> Index;
    fn index_ref(&self) -> &Index;
    fn new_index_capacity(&self) -> Option<ElementCount>;
    fn is_reused(&self) -> bool;
}

impl GetAssignedIndexData for AssignedIndex {
    fn index(&self) -> Index {
        self.index
    }

    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn new_index_capacity(&self) -> Option<ElementCount> {
        self.new_index_capacity
    }

    fn is_reused(&self) -> bool {
        self.is_reused
    }
}
