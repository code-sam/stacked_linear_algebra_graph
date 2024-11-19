use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::AtomicInMemoryVertexStoreTransaction;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStoreStateRestorer;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::RegisterEmptyVertexToRestore;
use crate::graph::vertex_store::operations::vertex_element::AddVertex;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::VertexVector;
use crate::operators::in_memory_transaction::transaction::CreateSparseVectorStateReverter;

impl<'s, T> AddVertex<'s, T> for AtomicInMemoryVertexStoreTransaction<'s>
where
    T: 's
        + ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<'s, T>
        + CreateSparseVectorStateReverter<T>
        + SetSparseVectorElementTyped<T>,
{
    fn add_new_public_vertex(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_public_vertex(type_index, value)?;

        RegisterEmptyVertexToRestore::<'s, T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_public_vertex(
        &'s mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.try_is_valid_public_vertex_type_index(vertex_type_index)?;
        self.add_or_update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_private_vertex(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_private_vertex(type_index, value)?;

        RegisterEmptyVertexToRestore::<'s, T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_private_vertex(
        &'s mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.try_is_valid_private_vertex_type_index(vertex_type_index)?;
        self.add_or_update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_vertex_unchecked(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_vertex_unchecked(type_index, value)?;

        RegisterEmptyVertexToRestore::<'s, T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_vertex_unchecked(
        &'s mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        if self.is_valid_vertex_index(vertex_index)? {
            let vertex_vector: &mut VertexVector =
                self.vertex_vector_mut_ref_unchecked(vertex_type_index)?;
            T::set_graphblas_vector_value(vertex_vector, vertex_index.index(), value)?;
            return Ok(None);
        } else {
            let index = self.add_new_vertex_unchecked(vertex_type_index, value)?;
            return Ok(Some(index));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::indexing::VertexIndex;
    use crate::graph::vertex_store::operations::vertex_element::GetVertexValue;
    use crate::graph::vertex_store::operations::vertex_type::AddPublicVertexType;
    use crate::graph::vertex_store::VertexStore;

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    #[test]
    fn test_add_new_vertex() {
        let context = GraphblasContext::init_default().unwrap();

        let mut store = VertexStore::with_initial_capacity(context, 0, 0).unwrap();

        let mut vertex_type_indices = Vec::new();
        for _i in 0..2 {
            vertex_type_indices.push(AddPublicVertexType::<i32>::apply(&mut store).unwrap());
        }

        for i in 0..50 {
            store
                .add_new_public_vertex(&vertex_type_indices[1], i)
                .unwrap();
        }
    }

    #[test]
    fn test_vertex_value_type_casting() {
        let context = GraphblasContext::init_default().unwrap();

        let mut store = VertexStore::with_initial_capacity(context, 10, 10).unwrap();

        let vertex_type_index_bool = AddPublicVertexType::<bool>::apply(&mut store).unwrap();
        let vertex_type_index_u8 = AddPublicVertexType::<u8>::apply(&mut store).unwrap();

        let vertex_index_10_as_bool = VertexIndex::new(
            store
                .add_new_public_vertex(&vertex_type_index_bool, 10)
                .unwrap()
                .index(),
        );
        let vertex_index_minus_1_as_u8 = VertexIndex::new(
            store
                .add_new_public_vertex(&vertex_type_index_u8, -1)
                .unwrap()
                .index(),
        );
        let vertex_index_f1000_as_u8 = VertexIndex::new(
            store
                .add_new_public_vertex(&vertex_type_index_u8, 1000.0f32)
                .unwrap()
                .index(),
        );

        assert_eq!(
            true,
            GetVertexValue::<bool>::public_vertex_value(
                &store,
                &vertex_type_index_bool,
                &vertex_index_10_as_bool
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            10,
            GetVertexValue::<i32>::public_vertex_value(
                &store,
                &vertex_type_index_bool,
                &vertex_index_10_as_bool
            )
            .unwrap()
            .unwrap()
        );

        assert_eq!(
            u8::MAX,
            GetVertexValue::<u8>::public_vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_minus_1_as_u8
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            -1,
            GetVertexValue::<i32>::public_vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_minus_1_as_u8
            )
            .unwrap()
            .unwrap()
        );

        assert_eq!(
            u8::MAX,
            GetVertexValue::<u8>::public_vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_f1000_as_u8
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            1000.0f32,
            GetVertexValue::<f32>::public_vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_f1000_as_u8
            )
            .unwrap()
            .unwrap()
        );
    }
}
