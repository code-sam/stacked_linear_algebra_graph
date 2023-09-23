use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::{AssignedIndexTrait, IndexerTrait},
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{
            vertex_store::VertexStoreTrait, VertexStore, VertexVector, VertexVectorTrait,
        },
    },
};

pub(crate) trait AddVertexType {
    fn add_new_vertex_type(
        &mut self,
        key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl AddVertexType for VertexStore {
    fn add_new_vertex_type(
        &mut self,
        key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError> {
        let new_type_index = self.vertex_type_indexer_mut_ref().add_new_key(key)?;

        self.synchronize_vector_with_vertex_vectors(&new_type_index);

        self.add_new_vertex_vector(&new_type_index)?;

        Ok(*new_type_index.index_ref())
    }
}

impl VertexStore {
    fn synchronize_vector_with_vertex_vectors(
        &mut self,
        new_type_index: &crate::graph::indexer::AssignedIndex,
    ) {
        if let Some(new_capacity) = new_type_index.new_index_capacity() {
            let current_capacity = self.vertex_vector_for_all_vertex_types_ref().len();
            self.vertex_vector_for_all_vertex_types_mut()
                .reserve(new_capacity - current_capacity);
        }
    }

    fn add_new_vertex_vector(
        &mut self,
        new_type_index: &crate::graph::indexer::AssignedIndex,
    ) -> Result<(), GraphComputingError> {
        let new_vertex_vector = VertexVector::new(
            self.graphblas_context_ref(),
            &self
                .element_indexer_ref()
                .mask_with_valid_indices_ref()
                .length()?,
        )?;
        Ok(
            if *new_type_index.index_ref() >= self.vertex_vector_for_all_vertex_types_ref().len() {
                self.vertex_vector_for_all_vertex_types_mut()
                    .push(new_vertex_vector);
            } else {
                self.vertex_vector_for_all_vertex_types_mut_ref()[*new_type_index.index_ref()] =
                    new_vertex_vector;
            },
        )
    }
}
