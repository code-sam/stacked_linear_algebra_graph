use crate::graph::graph::{VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::graph::vertex::vertex::GetVertexValue;

pub trait VertexDefinedByIndexTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn index_ref(&self) -> &VertexIndex;
    fn value_ref(&self) -> &T;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByIndex<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeIndex,
    value: T,
}

macro_rules! implement_vertex_defined_by_index_trait {
    ($value_type:ty) => {
        impl VertexDefinedByIndexTrait<$value_type> for VertexDefinedByIndex<$value_type> {
            fn type_index_ref(&self) -> &VertexTypeIndex {
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
implement_macro_for_all_native_value_types!(implement_vertex_defined_by_index_trait);

macro_rules! implement_get_vertex_value_for_vertex_defined_by_index {
    ($value_type:ty) => {
        impl GetVertexValue<$value_type> for VertexDefinedByIndex<$value_type> {
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_get_vertex_value_for_vertex_defined_by_index);

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
