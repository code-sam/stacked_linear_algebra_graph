use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStoreStateRestorer;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::RegisterEmptyVertexToRestore;
use crate::graph::vertex_store::operations::vertex_element::AddVertex;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::UpdateVertex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::operators::in_memory_transaction::transaction::CreateSparseVectorStateReverter;

impl<'s, T> AddVertex<'s, T> for InMemoryVertexStoreTransaction<'s>
where
    T: ValueType
        + Copy
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>
        + CreateSparseVectorStateReverter<T>
        + SetSparseVectorElementTyped<T>,
{
    fn add_new_public_vertex(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_public_vertex(type_index, value)?;

        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_public_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.try_is_valid_public_vertex_type_index(vertex_type_index)?;
        self.add_or_update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_private_vertex(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_private_vertex(type_index, value)?;

        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_private_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.try_is_valid_private_vertex_type_index(vertex_type_index)?;
        self.add_or_update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_vertex_unchecked(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_vertex_unchecked(type_index, value)?;

        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            type_index,
            vertex_index.index(),
        );

        Ok(vertex_index)
    }

    fn add_or_update_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        if self.is_valid_vertex_index(vertex_index)? {
            self.update_vertex_unchecked(vertex_type_index, vertex_index, value)?;
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

    use crate::graph::indexing::{VertexIndex, VertexTypeIndex};
    use crate::graph::vertex_store::operations::vertex_element::GetVertexValue;
    use crate::graph::vertex_store::operations::vertex_type::{
        AddPrivateVertexType, AddPublicVertexType,
    };
    use crate::graph::vertex_store::VertexStore;
    use crate::operators::transaction::UseTransaction;

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    #[test]
    fn test_add_new_vertex() {
        let mut vertex_store = initialize_vertex_store();

        let mut transaction = InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

        let vertex_type_index_1 = VertexTypeIndex::new(2);
        let vertex_index_1 = transaction
            .add_new_public_vertex(&vertex_type_index_1, 101)
            .unwrap();
        let vertex_index_2 = transaction
            .add_new_public_vertex(&vertex_type_index_1, 102)
            .unwrap();

        assert_eq!(
            101,
            transaction
                .public_vertex_value(
                    &vertex_type_index_1,
                    &VertexIndex::new(vertex_index_1.index())
                )
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            102,
            transaction
                .public_vertex_value(
                    &vertex_type_index_1,
                    &VertexIndex::new(vertex_index_2.index())
                )
                .unwrap()
                .unwrap()
        );

        transaction.commit().unwrap();

        assert_eq!(
            101,
            transaction
                .public_vertex_value(
                    &vertex_type_index_1,
                    &VertexIndex::new(vertex_index_1.index())
                )
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            102,
            transaction
                .public_vertex_value(
                    &vertex_type_index_1,
                    &VertexIndex::new(vertex_index_2.index())
                )
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn test_rollback_added_vertex() {
        let mut vertex_store = initialize_vertex_store();

        {
            let mut transaction =
                InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

            let vertex_type_index_1 = VertexTypeIndex::new(3);
            let _vertex_index_0 = transaction
                .add_new_public_vertex(&VertexTypeIndex::new(0), 100)
                .unwrap();
            let _vertex_index_1 = transaction
                .add_new_private_vertex(&vertex_type_index_1, 101)
                .unwrap();
            let _vertex_index_2 = transaction
                .add_new_private_vertex(&vertex_type_index_1, 102)
                .unwrap();
        }

        assert!(!vertex_store
            .is_valid_private_vertex_index(&VertexIndex::new(0))
            .unwrap());
        assert!(!vertex_store
            .is_valid_private_vertex_index(&VertexIndex::new(1))
            .unwrap());
    }

    fn initialize_vertex_store() -> VertexStore {
        let context = GraphblasContext::init_default().unwrap();

        let mut vertex_store = VertexStore::with_initial_capacity(context, 0, 0).unwrap();

        let mut public_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            public_vertex_type_indices
                .push(AddPublicVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_public_vertex(&public_vertex_type_indices[1], i)
                .unwrap();
        }

        let mut private_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            private_vertex_type_indices
                .push(AddPrivateVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_private_vertex(&private_vertex_type_indices[1], i)
                .unwrap();
        }

        vertex_store
    }
}
