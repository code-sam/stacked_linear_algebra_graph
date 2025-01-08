use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::vertex_store::GetVertexVectors as GetVertexVectorFromVertexStore;
use crate::graph::vertex_store::{GetVertexTypeIndexer, VertexStore, VertexVector};

pub(crate) trait GetVertexVector<'a> {
    fn vertex_vector_ref(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError>;
    fn vertex_vector_mut_ref(
        &'a mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&'a mut VertexVector, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &impl GetVertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_vector_ref_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &VertexVector;
    fn vertex_vector_mut_ref_unchecked(
        &'a mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError>;
}

pub(crate) fn vertex_vector_ref<'s>(
    vertex_store: &'s VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<&'s VertexVector, GraphComputingError> {
    vertex_store
        .vertex_type_indexer_ref()
        .try_index_validity(vertex_type_index.index())?;
    Ok(vertex_store.vertex_vector_ref_unchecked(vertex_type_index))
}

pub(crate) fn vertex_vector_mut_ref<'s>(
    vertex_store: &'s mut VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<&'s mut VertexVector, GraphComputingError> {
    vertex_store
        .vertex_type_indexer_ref()
        .try_index_validity(vertex_type_index.index())?;
    Ok(vertex_store.vertex_vector_mut_ref_unchecked(vertex_type_index)?)
}

pub(crate) fn vertex_vector_ref_unchecked<'s>(
    vertex_store: &'s VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> &'s VertexVector {
    &vertex_store.vertex_vector_for_all_vertex_types_ref()[*vertex_type_index.index_ref()]
}

pub(crate) fn vertex_vector_mut_ref_unchecked<'s>(
    vertex_store: &'s mut VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> &'s mut VertexVector {
    &mut vertex_store.vertex_vector_for_all_vertex_types_mut_ref()[*vertex_type_index.index_ref()]
}

pub(crate) trait GetVertexVectorNativeValueType {
    fn vertex_vector_native_value_type(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&ValueTypeIdentifier, GraphComputingError>;
    fn vertex_vector_native_value_type_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &ValueTypeIdentifier;
}

pub(crate) fn vertex_vector_native_value_type<'s>(
    vertex_store: &'s VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<&'s ValueTypeIdentifier, GraphComputingError> {
    vertex_store
        .vertex_type_indexer_ref()
        .try_index_validity(vertex_type_index.index())?;
    Ok(vertex_store.vertex_vector_native_value_type_unchecked(vertex_type_index))
}

pub(crate) fn vertex_vector_native_value_type_unchecked<'s>(
    vertex_store: &'s VertexStore,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> &'s ValueTypeIdentifier {
    vertex_store
        .vertex_vector_ref_unchecked(vertex_type_index)
        .value_type_identifier_ref()
}
