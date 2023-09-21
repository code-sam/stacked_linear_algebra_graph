use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::IndexerTrait,
        value_type::ValueType,
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{
            vertex_store::VertexStoreTrait, SparseVertexVector, VertexStore, VertexVector,
        },
    },
};

pub(crate) trait GetVertexVectorTyped<T: ValueType>
where
    VertexVector: SparseVertexVector<T>,
{
    fn vertex_vector_by_index_ref(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError>;
    fn vertex_vector_by_index_mut_ref(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &VertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_vector_by_index_ref_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexVector;
    fn vertex_vector_by_index_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexVector;

    fn vertex_vector_by_key_ref(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexVector, GraphComputingError>;
    fn vertex_vector_by_key_mut_ref(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexVector, GraphComputingError>;
}

impl<T: ValueType> GetVertexVectorTyped<T> for VertexStore
where
    VertexVector: SparseVertexVector<T>,
{
    fn vertex_vector_by_index_ref(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_vector_by_index_ref_unchecked(vertex_type_index))
    }
    fn vertex_vector_by_index_mut_ref(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_vector_by_index_mut_ref_unchecked(vertex_type_index))
    }
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &VertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError> {
    //     store.vertex_vectors
    //     store.vertex_type_indexer_ref()
    //         .try_index_validity(vertex_type_index)?;
    //     Ok(store.vertex_vector_by_index_mut_ref_unchecked(vertex_type_index))
    // }

    fn vertex_vector_by_index_ref_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexVector {
        &self.vertex_vector_for_all_vertex_types_ref()[*vertex_type_index]
    }
    fn vertex_vector_by_index_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexVector {
        &mut self.vertex_vector_for_all_vertex_types_mut_ref()[*vertex_type_index]
    }

    fn vertex_vector_by_key_ref(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexVector, GraphComputingError> {
        Ok(self.vertex_vector_by_index_ref_unchecked(
            self.vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?,
        ))
    }

    fn vertex_vector_by_key_mut_ref(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexVector, GraphComputingError> {
        Ok(self.vertex_vector_by_index_mut_ref_unchecked(
            &self
                .vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?
                .clone(),
        ))
    }
}
