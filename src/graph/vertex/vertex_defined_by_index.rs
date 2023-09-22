use crate::graph::graph::{VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{ValueType};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByIndexTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn index_ref(&self) -> &VertexIndex;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByIndex<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeIndex,
    value: T,
}

impl<T: ValueType> VertexDefinedByIndexTrait<T> for VertexDefinedByIndex<T> {
    fn type_index_ref(&self) -> &VertexTypeIndex {
        &self.vertex_type
    }

    fn index_ref(&self) -> &VertexIndex {
        &self.index
    }
}

impl<T: ValueType> GetVertexValue<T> for VertexDefinedByIndex<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType + Clone> VertexDefinedByIndex<T> {
    pub fn new(vertex_type: &VertexTypeIndex, index: &VertexIndex, value: &T) -> Self {
        Self {
            index: index.clone(),
            vertex_type: vertex_type.clone(),
            value: value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {}
