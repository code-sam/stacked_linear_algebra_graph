use crate::graph::graph::GetVertexStore;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
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
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

// REVIEW update vs set
pub(crate) trait UpdatePrivateVertexValue<T: ValueType> {
    fn update_private_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_value_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
