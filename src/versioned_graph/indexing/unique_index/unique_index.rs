use std::fmt::Debug;

use uuid::Uuid;

pub trait GetUniqueIndex: Debug {
    fn unique_index_ref(&self) -> &Uuid;
    fn unique_index(&self) -> Uuid;
}
