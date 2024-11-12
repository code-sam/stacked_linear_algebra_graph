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
