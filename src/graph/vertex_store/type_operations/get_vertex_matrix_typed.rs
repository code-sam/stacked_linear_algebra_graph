use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::IndexerTrait,
        value_type::ValueType,
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{
            vertex_store::VertexStoreTrait, SparseVertexMatrix, VertexMatrix, VertexStore,
        },
    },
};

pub(crate) trait GetVertexVectorTyped<T: ValueType>
where
    VertexMatrix: SparseVertexMatrix<T>,
{
    fn vertex_matrix_by_index_ref(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrix, GraphComputingError>;
    fn vertex_matrix_by_index_mut_ref(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrix, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &VertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_matrix_by_index_ref_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrix;
    fn vertex_matrix_by_index_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrix;

    fn vertex_matrix_by_key_ref(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrix, GraphComputingError>;
    fn vertex_matrix_by_key_mut_ref(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrix, GraphComputingError>;
}

impl<T: ValueType> GetVertexVectorTyped<T> for VertexStore
where
    VertexMatrix: SparseVertexMatrix<T>,
{
    fn vertex_matrix_by_index_ref(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrix, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_matrix_by_index_ref_unchecked(vertex_type_index))
    }
    fn vertex_matrix_by_index_mut_ref(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrix, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_matrix_by_index_mut_ref_unchecked(vertex_type_index))
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

    fn vertex_matrix_by_index_ref_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrix {
        &self.vertex_matrix_by_index_ref_unchecked(vertex_type_index)
    }
    fn vertex_matrix_by_index_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrix {
        &mut self.vertex_matrix_by_index_mut_ref_unchecked(vertex_type_index)
    }

    fn vertex_matrix_by_key_ref(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrix, GraphComputingError> {
        Ok(self.vertex_matrix_by_index_ref_unchecked(
            self.vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?,
        ))
    }

    fn vertex_matrix_by_key_mut_ref(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrix, GraphComputingError> {
        Ok(self.vertex_matrix_by_index_mut_ref_unchecked(
            &self
                .vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?
                .clone(),
        ))
    }
}
