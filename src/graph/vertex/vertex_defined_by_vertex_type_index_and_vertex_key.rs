use crate::graph::{
    graph::VertexTypeIndex,
    value_type::{implement_macro_for_all_native_value_types, ValueType},
};

use super::vertex::{VertexKey, VertexKeyRef, VertexTypeKeyRef};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByTypeIndexAndVertexKeyTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn key_ref(&self) -> &VertexTypeKeyRef;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByTypeIndexAndVertexKey<T: ValueType> {
    key: VertexKey,
    vertex_type: VertexTypeIndex,
    value: T,
}

impl<T: ValueType> VertexDefinedByTypeIndexAndVertexKeyTrait<T>
    for VertexDefinedByTypeIndexAndVertexKey<T>
{
    fn type_index_ref(&self) -> &VertexTypeIndex {
        &self.vertex_type
    }

    fn key_ref(&self) -> &VertexKeyRef {
        &self.key
    }
}

impl<T: ValueType> GetVertexValue<T> for VertexDefinedByTypeIndexAndVertexKey<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T: ValueType + Clone> VertexDefinedByTypeIndexAndVertexKey<T> {
    pub fn new(vertex_type: &VertexTypeIndex, key: &VertexKeyRef, value: &T) -> Self {
        Self {
            key: key.to_string(),
            vertex_type: vertex_type.clone(),
            value: value.clone(),
        }
    }
}
