use crate::graph::{
    indexing::{VertexIndex, VertexTypeIndex},
    value_type::ValueType,
};

pub trait GetVertexIndex {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn index_ref(&self) -> &VertexIndex;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinition<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeIndex,
    value: T,
}

impl<T: ValueType> GetVertexIndex for VertexDefinition<T> {
    fn type_index_ref(&self) -> &VertexTypeIndex {
        &self.vertex_type
    }

    fn index_ref(&self) -> &VertexIndex {
        &self.index
    }
}

impl<T: ValueType> GetVertexValue<T> for VertexDefinition<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType + Clone> VertexDefinition<T> {
    pub fn new(vertex_type: VertexTypeIndex, index: VertexIndex, value: T) -> Self {
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
