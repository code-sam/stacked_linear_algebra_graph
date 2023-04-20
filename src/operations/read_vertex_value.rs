use crate::error::GraphComputingError;
use crate::error::{UserError, UserErrorType};

use crate::graph::graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::graph::vertex::{VertexKey, VertexKeyRef, VertexTypeKeyRef};
use crate::graph::vertex_store::vertex_operations::ReadVertex;

pub trait ReadVertexValue<T: ValueType> {
    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    // fn vertex_value_by_vertex_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<T, GraphComputingError>;
}

macro_rules! implement_read_vertex {
    ($value_type:ty) => {
        impl ReadVertexValue<$value_type> for Graph {
            fn vertex_value_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                self.vertex_store_ref()
                    .vertex_value_by_key(vertex_type_key, vertex_key)
            }

            fn vertex_value_or_default_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref()
                    .vertex_value_or_default_by_key(vertex_type_key, vertex_key)
            }

            fn try_vertex_value_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref()
                    .try_vertex_value_by_key(vertex_type_key, vertex_key)
            }

            // fn vertex_value_by_vertex_type_index_and_vertex_key(
            //     &self,
            //     vertex_type_index: &VertexTypeIndex,
            //     vertex_key: &VertexKeyRef,
            // ) -> Result<$value_type, GraphComputingError> {
            //     self.vertex_store_ref()
            //         .vertex_value_by_type_index_and_vertex_key(vertex_type_index, vertex_key)
            // }

            fn vertex_value_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                self.vertex_store_ref()
                    .vertex_value_by_index(vertex_type_index, vertex_index)
            }

            fn vertex_value_or_default_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref()
                    .vertex_value_or_default_by_index(vertex_type_index, vertex_index)
            }

            fn try_vertex_value_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref()
                    .try_vertex_value_by_index(vertex_type_index, vertex_index)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_read_vertex);

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::graph::vertex::VertexValue;

    // TODO
    // #[test]
    // fn new_graph() {
    //     let graph = Graph::new(10, 20);
    // }
}
