use crate::error::GraphComputingError;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait AddVertex<'s, T>
where
    T: ValueType,
{
    fn add_new_vertex(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_set_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;

    fn add_new_vertex_unchecked(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_set_vertex_unchecked(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;
}
