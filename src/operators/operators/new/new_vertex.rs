use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;

pub trait NewVertex<T: ValueType> {
    fn new_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
