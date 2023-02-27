use crate::error::GraphComputingError;
use crate::error::{UserError, UserErrorType};

use crate::graph::graph::{Graph, GraphTrait, VertexIndex};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::graph::vertex::VertexKey;
use crate::graph::vertex_store::operations::ReadVertex;

pub trait ReadVertexValue<T: ValueType> {
    fn vertex_value_by_key(&self, vertex_key: &VertexKey) -> Result<T, GraphComputingError>;
    fn vertex_value_by_index(&self, vertex_index: VertexIndex) -> Result<T, GraphComputingError>;
}

macro_rules! implement_read_vertex {
    ($value_type:ty) => {
        impl ReadVertexValue<$value_type> for Graph<$value_type> {
            fn vertex_value_by_key(
                &self,
                vertex_key: &VertexKey,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref().vertex_value_by_key(vertex_key)
            }

            fn vertex_value_by_index(
                &self,
                vertex_index: VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                self.vertex_store_ref().vertex_value_by_index(vertex_index)
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
