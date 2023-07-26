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

macro_rules! implement_vertex_defined_by_type_key_and_vertex_index_trait {
    ($value_type:ty) => {
        impl VertexDefinedByTypeKeyAndVertexIndexTrait<$value_type>
            for VertexDefinedByTypeKeyAndVertexIndex<$value_type>
        {
            fn type_key_ref(&self) -> &VertexTypeKeyRef {
                &self.vertex_type
            }

            fn index_ref(&self) -> &VertexIndex {
                &self.index
            }

            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_vertex_defined_by_type_key_and_vertex_index_trait
);

macro_rules! implement_get_vertex_value_for_vertex_defined_by_type_key_and_vertex_index {
    ($value_type:ty) => {
        impl GetVertexValue<$value_type> for VertexDefinedByTypeKeyAndVertexIndex<$value_type> {
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_get_vertex_value_for_vertex_defined_by_type_key_and_vertex_index
);

impl<T: ValueType + Clone> VertexDefinedByTypeKeyAndVertexIndex<T> {
    pub fn new(vertex_type: &VertexTypeKeyRef, index: &VertexIndex, value: &T) -> Self {
        Self {
            index: index.clone(),
            vertex_type: vertex_type.to_string(),
            value: value.clone(),
        }
    }
}
