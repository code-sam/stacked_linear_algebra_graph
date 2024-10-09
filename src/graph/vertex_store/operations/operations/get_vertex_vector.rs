use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::vertex_store::GetVertexVectors as GetVertexVectorFromVertexStore;
use crate::{
    error::GraphComputingError,
    graph::{
        indexing::operations::CheckIndex,
        vertex_store::{GetVertexTypeIndexer, VertexStore, VertexVector},
    },
};

pub(crate) trait GetVertexVector {
    fn public_vertex_vector_ref(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError>;
    fn public_vertex_vector_mut_ref(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &impl GetVertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn private_vertex_vector_ref(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError>;
    fn private_vertex_vector_mut_ref(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &impl GetVertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_vector_ref_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &VertexVector;
    fn vertex_vector_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &mut VertexVector;
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
