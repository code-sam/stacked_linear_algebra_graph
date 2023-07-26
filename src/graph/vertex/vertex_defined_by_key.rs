use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

use super::vertex::{VertexKey, VertexKeyRef, VertexTypeKey, VertexTypeKeyRef};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByKeyTrait<T: ValueType> {
    fn type_key_ref(&self) -> &VertexTypeKeyRef;
    fn key_ref(&self) -> &VertexKeyRef;
    fn value_ref(&self) -> &T;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByKey<T: ValueType> {
    key: VertexKey,
    vertex_type: VertexTypeKey,
    value: T,
}

macro_rules! implement_vertex_defined_by_key_trait {
    ($value_type:ty) => {
        impl VertexDefinedByKeyTrait<$value_type> for VertexDefinedByKey<$value_type> {
            fn type_key_ref(&self) -> &VertexTypeKeyRef {
                self.vertex_type.as_str()
            }
            fn key_ref(&self) -> &VertexKeyRef {
                self.key.as_str()
            }
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_vertex_defined_by_key_trait);

macro_rules! implement_get_vertex_value_for_vertex_defined_by_key {
    ($value_type:ty) => {
        impl GetVertexValue<$value_type> for VertexDefinedByKey<$value_type> {
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_get_vertex_value_for_vertex_defined_by_key);

impl<T: ValueType + Clone> VertexDefinedByKey<T> {
    pub fn new(vertex_type: &VertexTypeKeyRef, key: &VertexKeyRef, value: &T) -> Self {
        Self {
            key: key.to_owned(),
            vertex_type: vertex_type.to_owned(),
            value: value.clone(),
        }
    }
}
