use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::{GetVertexElementIndexer, IsElementInVertexVector, VertexStore};

impl CheckVertexIndex for VertexStore {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_index(vertex_index.index())
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())
    }

    fn is_valid_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_vector_ref(vertex_type_index)?
            .is_vertex_element(vertex_index)
    }

    fn try_is_valid_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_vector_ref(vertex_type_index)?
            .try_is_vertex_element(vertex_index)
    }

    fn is_empty_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(!self
            .vertex_vector_ref(vertex_type_index)?
            .is_vertex_element(vertex_index)?)
    }

    fn try_is_empty_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        match self.is_empty_vertex_element(vertex_type_index, vertex_index)? {
            true => Ok(()),
            false => Err(LogicError::new(
                LogicErrorType::VertexElementNotEmpty,
                String::from("Vertex type is valid but the vertex element is not empty"),
                None,
            )
            .into()),
        }
    }
}
