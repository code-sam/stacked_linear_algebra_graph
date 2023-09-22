use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;

use crate::{
    error::{GraphComputingError, LogicError},
    graph::{
        graph::{VertexIndex, VertexTypeIndex},
        indexer::IndexerTrait,
        value_type::{SparseVertexVectorForValueType, ValueType},
        vertex::vertex::{VertexKeyRef, VertexTypeKeyRef},
        vertex_store::{
            DeleteVertexValueInVertexVector, SparseVertexVector, VertexStore, VertexStoreTrait,
            VertexVector,
        },
    },
};

pub(crate) trait DeleteVertexElement<T: ValueType> {
    fn delete_vertex_element_by_key(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn delete_vertex_element_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexForAllTypes {
    fn delete_vertex_for_all_vertex_types_and_value_types_by_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn delete_vertex_for_all_vertex_types_and_value_types_by_index(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexVectorForValueType<T>> DeleteVertexElement<T> for VertexStore
where
    VertexVector: SparseVertexVector<T>,
{
    fn delete_vertex_element_by_key(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError> {
        let vertex_type_index = *self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)?;
        let vertex_index = *self.element_indexer_ref().try_index_for_key(vertex_key)?;

        SparseVertexVector::<T>::sparse_vector_mut_ref(
            &mut self.vertex_vector_for_all_vertex_types_mut_ref()[vertex_type_index],
        )
        .drop_element(vertex_index)?;
        Ok(())
    }

    fn delete_vertex_element_by_index(
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
        SparseVertexVector::<T>::sparse_vector_mut_ref(vertex_vector)
            .drop_element(*vertex_index)?;
        Ok(())
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_vertex_types_and_value_types_by_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError> {
        let vertex_element_index = *self.element_indexer_ref().try_index_for_key(vertex_key)?;
        self.delete_vertex_for_all_vertex_types_and_value_types_by_index(&vertex_element_index)
    }

    fn delete_vertex_for_all_vertex_types_and_value_types_by_index(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_vertex_vectors(|vertex_vector: &mut VertexVector| {
            vertex_vector.delete_vertex_value_for_all_value_types(vertex_element_index)
        })?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }
}
