use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;

pub trait CheckVertexTypeIndex {
    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}
