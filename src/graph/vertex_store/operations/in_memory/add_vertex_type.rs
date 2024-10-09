use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorLength;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::GetGraphblasContext,
        indexing::{
            operations::{GeneratePrivateIndex, GeneratePublicIndex, GetValidIndices},
            AssignedIndex, GetAssignedIndexData, VertexTypeIndex,
        },
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::{
            operations::{AddPrivateVertexType, AddPublicVertexType},
            CreateVertexVector, GetVertexElementIndexer, GetVertexTypeIndexer, GetVertexVectors,
            VertexStore, VertexVector,
        },
    },
};

impl<'a, T: ValueType + GetValueTypeIdentifier> AddPublicVertexType<'a, T> for VertexStore {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let assigned_vertex_type_index = self.add_public_vertex_type_in_memory::<T>()?;
        Ok(assigned_vertex_type_index.into())
    }
}

impl<'a, T: ValueType + GetValueTypeIdentifier> AddPrivateVertexType<'a, T> for VertexStore {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let assigned_vertex_type_index = self.add_private_vertex_type_in_memory::<T>()?;
        Ok(assigned_vertex_type_index.into())
    }
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

    pub(crate) fn add_public_vertex_type_in_memory<T: ValueType + GetValueTypeIdentifier>(
        &mut self,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let new_type_index = self.vertex_type_indexer_mut_ref().new_public_index()?;
        self.add_vertex_type::<T>(&new_type_index)?;
        Ok(new_type_index)
    }

    pub(crate) fn add_private_vertex_type_in_memory<T: ValueType + GetValueTypeIdentifier>(
        &mut self,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let new_type_index = self.vertex_type_indexer_mut_ref().new_private_index()?;
        self.add_vertex_type::<T>(&new_type_index)?;
        Ok(new_type_index)
    }
}
