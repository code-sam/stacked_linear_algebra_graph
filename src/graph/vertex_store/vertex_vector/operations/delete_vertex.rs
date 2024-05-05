use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::drop_sparse_vector_element;

use crate::{
    error::GraphComputingError,
    graph::{
        index::{VertexIndex, VertexTypeIndex},
        indexing::operations::{CheckIndex, FreeIndex},
        vertex_store::{
            operations::{
                get_vertex_vector::GetVertexVector,
                map::{MapPrivateVertexVectors, MapPublicVertexVectors, MapValidVertexVectors},
            },
            GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore, VertexVector,
        },
    },
};

pub(crate) trait DeleteVertexValue {
    fn delete_public_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexForAllTypes {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_valid_public_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_valid_private_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexValue for VertexStore {
    fn delete_public_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index)?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_private_vertex_element(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index)?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_unchecked(vertex_type_index);
        drop_sparse_vector_element(vertex_vector, *vertex_index)?;
        Ok(())
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                *vertex_element_index,
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }

    fn delete_vertex_for_all_valid_public_vertex_types_and_value_types(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_public_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                *vertex_element_index,
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }

    fn delete_vertex_for_all_valid_private_vertex_types_and_value_types(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_private_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                *vertex_element_index,
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }
}
