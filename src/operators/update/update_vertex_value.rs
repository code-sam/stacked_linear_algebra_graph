use crate::graph::graph::{GetVertexStore, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::VertexStore;
use crate::{
    error::GraphComputingError,
    graph::{graph::Graph, vertex_store::UpdateVertex},
};

// REVIEW update vs set
pub trait UpdateVertexValue<T: ValueType> {
    fn update_vertex_value_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> UpdateVertexValue<T> for Graph
where
    T: ValueType,
    VertexStore: UpdateVertex<T>,
{
    fn update_vertex_value_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .update_vertex(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
