use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{VersionedVertexTypeIndex, VersionedVertexIndex};

pub trait GetUniqueVertexIndex {
    fn type_index_ref(&self) -> &VersionedVertexTypeIndex;
    fn index_ref(&self) -> &VersionedVertexIndex;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UniqueVertexDefinition<T: ValueType> {
    index: VersionedVertexIndex,
    vertex_type: VersionedVertexTypeIndex,
    value: T,
}

impl<T: ValueType> GetUniqueVertexIndex for UniqueVertexDefinition<T> {
    fn type_index_ref(&self) -> &VersionedVertexTypeIndex {
        &self.vertex_type
    }

    fn index_ref(&self) -> &VersionedVertexIndex {
        &self.index
    }
}

impl<T: ValueType> GetVertexValue<T> for UniqueVertexDefinition<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType + Clone> UniqueVertexDefinition<T> {
    pub fn new(vertex_type: VersionedVertexTypeIndex, index: VersionedVertexIndex, value: T) -> Self {
        Self {
            index: index,
            vertex_type: vertex_type,
            value: value,
        }
    }
}

pub trait GetVertexValue<T: ValueType> {
    fn value_ref(&self) -> &T;
}

#[cfg(test)]
mod tests {}
