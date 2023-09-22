use crate::graph::{
    graph::VertexIndex,
    value_type::{implement_macro_for_all_native_value_types, ValueType},
};

use super::vertex::{VertexTypeKey, VertexTypeKeyRef};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByTypeKeyAndVertexIndexTrait<T: ValueType> {
    fn type_key_ref(&self) -> &VertexTypeKeyRef;
    fn index_ref(&self) -> &VertexIndex;
    fn value_ref(&self) -> &T;
}

pub struct VertexDefinedByTypeKeyAndVertexIndex<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeKey,
    value: T,
}

impl<T: ValueType> VertexDefinedByTypeKeyAndVertexIndexTrait<T>
    for VertexDefinedByTypeKeyAndVertexIndex<T>
{
    fn type_key_ref(&self) -> &VertexTypeKeyRef {
        &self.vertex_type
    }

    fn index_ref(&self) -> &VertexIndex {
        &self.index
    }

    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType> GetVertexValue<T> for VertexDefinedByTypeKeyAndVertexIndex<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType + Clone> VertexDefinedByTypeKeyAndVertexIndex<T> {
    pub fn new(vertex_type: &VertexTypeKeyRef, index: &VertexIndex, value: &T) -> Self {
        Self {
            index: index.clone(),
            vertex_type: vertex_type.to_string(),
            value: value.clone(),
        }
    }
}
