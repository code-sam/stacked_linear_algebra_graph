use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorLength;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::{GetAssignedIndexData, IndexerTrait},
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::{
            vertex_store::VertexStoreTrait, CreateVertexVector, VertexStore, VertexVector,
        },
    },
};

pub(crate) trait AddVertexType<T: ValueType> {
    fn new_vertex_type(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for VertexStore {
    fn new_vertex_type(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let new_type_index = self.vertex_type_indexer_mut_ref().new_index()?;

        self.synchronize_vector_with_vertex_vectors(&new_type_index);

        let new_vertex_vector = <VertexVector as CreateVertexVector<T>>::new(
            self.graphblas_context_ref(),
            &self
                .element_indexer_ref()
                .mask_with_valid_indices_ref()
                .length()?,
        )?;

        self.add_new_vertex_vector(new_vertex_vector, &new_type_index)?;

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
        new_vertex_vector: VertexVector,
        new_type_index: &crate::graph::indexer::AssignedIndex,
    ) -> Result<(), GraphComputingError> {
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
