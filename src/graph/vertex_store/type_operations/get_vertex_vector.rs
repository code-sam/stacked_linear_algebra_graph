use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::IndexerTrait,
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{vertex_store::VertexStoreTrait, VertexMatrixStore, VertexStore},
    },
};

pub(crate) trait GetVertexVector {
    fn vertex_vector_ref_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrixStore, GraphComputingError>;
    fn vertex_vector_mut_ref_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrixStore, GraphComputingError>;
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &VertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError>;

    fn vertex_vector_ref_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrixStore;
    fn vertex_vector_mut_ref_by_index_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrixStore;

    fn vertex_vector_ref_by_key(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrixStore, GraphComputingError>;
    fn vertex_vector_mut_ref_by_key(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrixStore, GraphComputingError>;
}

impl GetVertexVector for VertexStore {
    fn vertex_vector_ref_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&VertexMatrixStore, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_vector_ref_by_index_unchecked(vertex_type_index))
    }
    fn vertex_vector_mut_ref_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&mut VertexMatrixStore, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        Ok(self.vertex_vector_mut_ref_by_index_unchecked(vertex_type_index))
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

    fn vertex_vector_ref_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &VertexMatrixStore {
        &self.vertex_matrix_for_all_vertex_types_ref()[*vertex_type_index]
    }
    fn vertex_vector_mut_ref_by_index_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> &mut VertexMatrixStore {
        &mut self.vertex_vector_for_all_vertex_types_mut_ref()[*vertex_type_index]
    }

    fn vertex_vector_ref_by_key(
        &self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&VertexMatrixStore, GraphComputingError> {
        Ok(self.vertex_vector_ref_by_index_unchecked(
            self.vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?,
        ))
    }

    fn vertex_vector_mut_ref_by_key(
        &mut self,
        vertex_key_ref: &VertexTypeKeyRef,
    ) -> Result<&mut VertexMatrixStore, GraphComputingError> {
        Ok(self.vertex_vector_mut_ref_by_index_unchecked(
            &self
                .vertex_type_indexer_ref()
                .try_index_for_key(vertex_key_ref)?
                .clone(),
        ))
    }
}
