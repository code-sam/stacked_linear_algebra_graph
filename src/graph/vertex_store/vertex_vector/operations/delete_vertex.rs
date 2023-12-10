use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::drop_sparse_vector_element;

use crate::{
    error::{GraphComputingError, LogicError},
    graph::{
        graph::{VertexIndex, VertexTypeIndex},
        indexer::IndexerTrait,
        vertex_store::{VertexStore, VertexStoreTrait, VertexVector},
    },
};

pub(crate) trait DeleteVertexValue {
    fn delete_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexForAllTypes {
    fn delete_vertex_for_all_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexValue for VertexStore {
    fn delete_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = match self
            .vertex_vector_for_all_vertex_types_mut_ref()
            .get_mut(*vertex_type_index)
        {
            Some(sparse_vertex_vector) => sparse_vertex_vector,
            None => {
                return Err(LogicError::new(
                    crate::error::LogicErrorType::IndexOutOfBounds,
                    format!("Vertex type index out of bounds: {}", vertex_type_index),
                    None,
                )
                .into());
            }
        };
        drop_sparse_vector_element(vertex_vector, *vertex_index)?;
        Ok(())
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_vertex_types_and_value_types(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                *vertex_element_index,
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }
}
