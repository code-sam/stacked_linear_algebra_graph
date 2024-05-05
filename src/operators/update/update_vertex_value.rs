use crate::graph::graph::GetVertexStore;
use crate::graph::index::{VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::VertexStore;
use crate::{
    error::GraphComputingError,
    graph::{graph::Graph, vertex_store::UpdateVertex},
};

// REVIEW update vs set
pub trait UpdateVertexValue<T: ValueType> {
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

// REVIEW update vs set
pub(crate) trait UpdatePrivateVertexValue<T: ValueType> {
    fn update_private_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_value_unchecked(
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
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .update_public_vertex(vertex_type_index, vertex_index, value)
    }
}

impl<T> UpdatePrivateVertexValue<T> for Graph
where
    T: ValueType,
    VertexStore: UpdateVertex<T>,
{
    fn update_private_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .update_private_vertex(vertex_type_index, vertex_index, value)
    }

    fn update_vertex_value_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
