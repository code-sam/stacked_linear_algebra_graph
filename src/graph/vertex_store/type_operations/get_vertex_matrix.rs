use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::IndexerTrait,
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{vertex_store::VertexStoreTrait, VertexMatrix, VertexStore},
    },
};

pub(crate) trait GetVertexMatrix {
    fn vertex_matrix_ref_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrix, GraphComputingError>;
    fn vertex_matrix_mut_ref_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrix, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &VertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_matrix_ref_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrix;
    fn vertex_matrix_mut_ref_by_index_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrix;

    fn vertex_matrix_ref_by_key(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrix, GraphComputingError>;
    fn vertex_matrix_mut_ref_by_key(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrix, GraphComputingError>;
}

impl GetVertexMatrix for VertexStore {
    fn vertex_matrix_ref_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrix, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_matrix_ref_by_index_unchecked(vertex_type_index))
    }
    fn vertex_matrix_mut_ref_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrix, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_matrix_mut_ref_by_index_unchecked(vertex_type_index))
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

    fn vertex_matrix_ref_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrix {
        &self.vertex_matrix_mut_ref_by_index_unchecked(vertex_type_index)
    }
    fn vertex_matrix_mut_ref_by_index_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrix {
        &mut self.vertex_matrix_mut_ref_by_index_unchecked(vertex_type_index)
    }

    fn vertex_matrix_ref_by_key(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrix, GraphComputingError> {
        Ok(self.vertex_matrix_ref_by_index_unchecked(
            self.vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?,
        ))
    }

    fn vertex_matrix_mut_ref_by_key(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrix, GraphComputingError> {
        Ok(self.vertex_matrix_mut_ref_by_index_unchecked(
            &self
                .vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?
                .clone(),
        ))
    }
}
