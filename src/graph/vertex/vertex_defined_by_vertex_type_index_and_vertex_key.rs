use crate::graph::{
    graph::VertexTypeIndex,
    value_type::{implement_macro_for_all_native_value_types, ValueType},
};

use super::vertex::{VertexKey, VertexKeyRef, VertexTypeKeyRef};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByTypeIndexAndVertexKeyTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn key_ref(&self) -> &VertexTypeKeyRef;
    fn value_ref(&self) -> &T;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByTypeIndexAndVertexKey<T: ValueType> {
    key: VertexKey,
    vertex_type: VertexTypeIndex,
    value: T,
}

macro_rules! implement_vertex_defined_by_type_index_and_vertex_key_trait {
    ($value_type:ty) => {
        impl VertexDefinedByTypeIndexAndVertexKeyTrait<$value_type>
            for VertexDefinedByTypeIndexAndVertexKey<$value_type>
        {
            fn type_index_ref(&self) -> &VertexTypeIndex {
                &self.vertex_type
            }

            fn key_ref(&self) -> &VertexKeyRef {
                &self.key
            }

            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_vertex_defined_by_type_index_and_vertex_key_trait
);

macro_rules! implement_get_vertex_value_for_vertex_defined_by_type_index_and_vertex_key {
    ($value_type:ty) => {
        impl GetVertexValue<$value_type> for VertexDefinedByTypeIndexAndVertexKey<$value_type> {
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_get_vertex_value_for_vertex_defined_by_type_index_and_vertex_key
);

impl<T: ValueType + Clone> VertexDefinedByTypeIndexAndVertexKey<T> {
    pub fn new(vertex_type: &VertexTypeIndex, key: &VertexKeyRef, value: &T) -> Self {
        Self {
            key: key.to_string(),
            vertex_type: vertex_type.clone(),
            value: value.clone(),
        }
    }
}
