use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStoreStateRestorer;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::RegisterNewVertexToRevert;
use crate::graph::vertex_store::operations::vertex_element::AddVertex;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;

impl<'s, T> AddVertex<'s, T> for InMemoryVertexStoreTransaction<'s>
where
    T: ValueType
        + Copy
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>
        + SetSparseVectorElementTyped<T>,
{
    fn add_new_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_type_index_validity(vertex_type_index)?;
        self.add_new_vertex_unchecked(vertex_type_index, value)
    }

    fn add_or_set_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.try_vertex_type_index_validity(vertex_type_index)?;
        self.add_or_set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self
            .vertex_store_mut_ref()
            .add_new_vertex_unchecked(vertex_type_index, value)?;

        RegisterNewVertexToRevert::<T>::register_new_vertex_to_revert(
            self.vertex_store_state_restorer_mut_ref(),
            vertex_type_index,
            &vertex_index,
        )?;

        Ok(vertex_index)
    }

    fn add_or_set_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        if self.is_valid_vertex_index(vertex_index)? {
            self.set_vertex_unchecked(vertex_type_index, vertex_index, value)?;
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

    use crate::graph::indexing::{GetAssignedIndexData, GetIndex, VertexIndex, VertexTypeIndex};
    use crate::graph::vertex_store::operations::vertex_element::GetVertexValue;
    use crate::graph::vertex_store::operations::vertex_type::AddVertexType;
    use crate::graph::vertex_store::VertexStore;
    use crate::operators::transaction::UseTransaction;

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    #[test]
    fn test_add_new_vertex() {
        let mut vertex_store = initialize_vertex_store();

        let mut transaction = InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

        let vertex_type_index_1 = VertexTypeIndex::new(2);
        let vertex_index_1 = transaction
            .add_new_vertex(&vertex_type_index_1, 101)
            .unwrap();
        let vertex_index_2 = transaction
            .add_new_vertex(&vertex_type_index_1, 102)
            .unwrap();

        assert_eq!(
            101,
            GetVertexValue::<i32>::vertex_value(
                &transaction,
                &vertex_type_index_1,
                &VertexIndex::new(vertex_index_1.index())
            )
            .unwrap()
            .unwrap()
        );

        transaction.commit().unwrap();

        assert_eq!(
            101,
            GetVertexValue::<i32>::vertex_value(
                &transaction,
                &vertex_type_index_1,
                &VertexIndex::new(vertex_index_1.index())
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            102,
            GetVertexValue::<i32>::vertex_value(
                &transaction,
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

        let vertex_type_1_index = AddVertexType::<u16>::apply(&mut vertex_store).unwrap();
        let vertex_index_1 = vertex_store
            .add_new_vertex(&vertex_type_1_index, 1)
            .unwrap();
        let vertex_index_2 = vertex_store
            .add_new_vertex(&vertex_type_1_index, 1)
            .unwrap();

        {
            let mut transaction = InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

            let vertex_type_2_index = AddVertexType::<u16>::apply(&mut transaction).unwrap();

            let _ = transaction
                .add_new_vertex(&vertex_type_1_index, 100)
                .unwrap();
            let _ = transaction
                .add_new_vertex(&vertex_type_2_index, 101)
                .unwrap();
            let _ = transaction
                .add_new_vertex(&vertex_type_2_index, 102)
                .unwrap();
        }

        assert!(vertex_store
            .is_valid_vertex_type_index(&vertex_type_1_index)
            .unwrap());
        assert!(vertex_store
            .is_valid_vertex_index(&VertexIndex::new(vertex_index_2.index()))
            .unwrap());

        assert!(!vertex_store
            .is_valid_vertex_type_index(&VertexTypeIndex::new(vertex_type_1_index.index() + 1))
            .unwrap());
        assert!(!vertex_store
            .is_valid_vertex_index(&VertexIndex::new(vertex_index_2.index() + 1))
            .unwrap());
    }

    fn initialize_vertex_store() -> VertexStore {
        let context = GraphblasContext::init_default().unwrap();

        let mut vertex_store = VertexStore::with_initial_capacity(context, 0, 0).unwrap();

        let mut public_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            public_vertex_type_indices
                .push(AddVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_vertex(&public_vertex_type_indices[1], i)
                .unwrap();
        }

        let mut private_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            private_vertex_type_indices
                .push(AddVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_vertex(&private_vertex_type_indices[1], i)
                .unwrap();
        }

        vertex_store
    }
}
