use std::fmt::Debug;

use uuid::Uuid;

pub trait GetVersionedIndex: Debug {
    fn versioned_index_ref(&self) -> &Uuid;
    fn versioned_index(&self) -> Uuid;
}
