use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorLength;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::{
    operations::{GenerateIndex, GetValidIndices},
    AssignedIndex, GetAssignedIndexData, VertexTypeIndex,
};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::{
    CreateVertexVector, GetVertexElementIndexer, GetVertexTypeIndexer, GetVertexVectors,
    VertexStore, VertexVector,
};

pub(crate) trait AddVertexType<'a, T: ValueType> {
    fn apply(&'a mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}

pub(crate) fn add_vertex_type<T: ValueType + GetValueTypeIdentifier>(
    vertex_store: &mut VertexStore,
) -> Result<AssignedIndex, GraphComputingError> {
    let new_type_index = vertex_store.vertex_type_indexer_mut_ref().new_index()?;
    vertex_store.add_vertex_type::<T>(&new_type_index)?;
    Ok(new_type_index)
}

impl VertexStore {
    fn add_vertex_type<T: ValueType + GetValueTypeIdentifier>(
        &mut self,
        new_type_index: &AssignedIndex,
    ) -> Result<(), GraphComputingError> {
        self.synchronize_vector_with_vertex_vectors(&new_type_index);

        let new_vertex_vector = <VertexVector as CreateVertexVector<T>>::new(
            self.graphblas_context(),
            self.element_indexer_ref()
                .mask_with_valid_indices_ref()
                .length()?,
        )?;

        self.add_new_vertex_vector(new_vertex_vector, &new_type_index)?;

        Ok(())
    }

    fn synchronize_vector_with_vertex_vectors(&mut self, new_type_index: &AssignedIndex) {
        if let Some(new_capacity) = new_type_index.new_index_capacity() {
            let current_capacity = self.vertex_vector_for_all_vertex_types_ref().len();
            self.vertex_vector_for_all_vertex_types_mut()
                .reserve(new_capacity - current_capacity);
        }
    }

    fn add_new_vertex_vector(
        &mut self,
        new_vertex_vector: VertexVector,
        new_type_index: &AssignedIndex,
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
