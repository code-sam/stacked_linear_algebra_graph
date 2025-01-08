use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait SetVertex<T: ValueType> {
    fn set_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn set_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}
