use super::index::{ElementCount, Index};

#[derive(Debug)]
pub(crate) struct AssignedIndex {
    index: Index,
    new_index_capacity: Option<ElementCount>,
}

impl AssignedIndex {
    pub(super) fn new(index: Index, new_index_capacity: Option<ElementCount>) -> Self {
        Self {
            index,
            new_index_capacity,
        }
    }
}

pub(crate) trait GetAssignedIndexData {
    fn index(&self) -> Index;
    fn index_ref(&self) -> &Index;
    fn new_index_capacity(&self) -> Option<ElementCount>;
}

impl GetAssignedIndexData for AssignedIndex {
    fn index(&self) -> Index {
        self.index.to_owned()
    }

    fn index_ref(&self) -> &Index {
        &self.index
    }

    fn new_index_capacity(&self) -> Option<ElementCount> {
        self.new_index_capacity
    }
}
