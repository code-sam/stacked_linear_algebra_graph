use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::drop_sparse_vector_element;

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{
            operations::{CheckIndex, FreeIndex},
            GetVertexIndexIndex, GetVertexTypeIndex,
        },
        vertex_store::{
            operations::{GetVertexVector, MapPrivateVertexVectors, MapPublicVertexVectors},
            GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore, VertexVector,
        },
    },
};

pub(crate) trait DeleteVertexValue {
    fn delete_public_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexForAllTypes {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_valid_public_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_valid_private_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexValue for VertexStore {
    fn delete_public_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_private_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_unchecked(vertex_type_index);
        drop_sparse_vector_element(vertex_vector, *vertex_index.index_ref())?;
        Ok(())
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        self.delete_vertex_for_all_valid_public_vertex_types_and_value_types(vertex_index)?;
        self.delete_vertex_for_all_valid_private_vertex_types_and_value_types(vertex_index)
    }

    fn delete_vertex_for_all_valid_public_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_public_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                vertex_index.index(),
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_public_index_unchecked(vertex_index.index())
    }

    fn delete_vertex_for_all_valid_private_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_private_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                vertex_index.index(),
            )?)
        })?;
        self.element_indexer_mut_ref()
            .free_private_index_unchecked(vertex_index.index())
    }
}
