use crate::error::GraphComputingError;
use crate::graph::graph::GetVertexStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::UpdateVertex;
use crate::graph::vertex_store::VertexStore;
use crate::operators::operators::update::UpdateVertexValue;

impl<T> UpdateVertexValue<T> for Graph
where
    T: ValueType,
    VertexStore: UpdateVertex<T>,
{
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .update_vertex(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
