use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::GetVertexStore;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::GetVertexStoreStateRestorer;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::RegisterEmptyVertexToRestore;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::RegisterVertexValueToRestore;
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::traits::vertex_element::SetVertex;
use crate::graph::vertex_store::traits::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::traits::vertex_type::GetVertexVector;

impl<'s, T> SetVertex<T> for InMemoryVertexStoreTransaction<'s>
where
    T: ValueType
        + SetSparseVectorElementTyped<T>
        + Default
        + GetSparseVectorElementValueTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn set_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_store.vertex_vector_ref(vertex_type_index)?;
        self.vertex_store.try_vertex_index_validity(vertex_index)?;

        self.vertex_store_state_restorer
            .register_optional_vertex_value_to_restore(
                vertex_vector,
                vertex_type_index,
                vertex_index,
            )?;
        self.vertex_store_mut_ref()
            .set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn set_new_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store
            .try_vertex_type_index_validity(vertex_type_index)?;
        self.vertex_store.try_vertex_index_validity(vertex_index)?;
        self.try_is_empty_vertex_element(vertex_type_index, vertex_index)?;

        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self.vertex_store_state_restorer_mut_ref(),
            vertex_type_index,
            vertex_index.index(),
        );

        self.vertex_store_mut_ref()
            .set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn set_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_store.vertex_vector_ref(vertex_type_index)?;
        self.vertex_store_state_restorer
            .register_optional_vertex_value_to_restore(
                vertex_vector,
                vertex_type_index,
                vertex_index,
            )?;
        self.vertex_store_mut_ref()
            .set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph::GraphblasContext;
    use crate::graph::indexing::{GetAssignedIndexData, VertexIndex};
    use crate::graph::vertex_store::traits::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
    use crate::graph::vertex_store::traits::vertex_element::{
        AddVertex, CheckVertexIndex, CreateVertexIndex, GetVertexValue, SetVertex,
    };
    use crate::graph::vertex_store::traits::vertex_type::AddVertexType;
    use crate::graph::vertex_store::VertexStore;

    #[test]
    fn test_rollback_set_vertex() {
        let mut vertex_store = initialize_vertex_store();

        let vertex_type_1_index = AddVertexType::<u16>::apply(&mut vertex_store).unwrap();
        let vertex_index_1 = vertex_store
            .add_new_vertex(&vertex_type_1_index, 1)
            .unwrap();
        let vertex_index_2 = vertex_store
            .add_new_vertex(&vertex_type_1_index, 1)
            .unwrap();

        let _ = vertex_store
            .set_vertex(
                &vertex_type_1_index,
                &VertexIndex::new(vertex_index_1.index()),
                10,
            )
            .unwrap();

        {
            let mut transaction = InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

            let vertex_type_2_index = AddVertexType::<u16>::apply(&mut transaction).unwrap();

            let _ = transaction
                .set_vertex(
                    &vertex_type_1_index,
                    &VertexIndex::new(vertex_index_1.index()),
                    100,
                )
                .unwrap();

            let _ = transaction
                .set_vertex(
                    &vertex_type_1_index,
                    &VertexIndex::new(vertex_index_1.index()),
                    200,
                )
                .unwrap();

            let vertex_index_3 = transaction.new_vertex_index().unwrap();

            let _ = transaction
                .set_vertex(
                    &vertex_type_1_index,
                    &VertexIndex::new(vertex_index_3.index()),
                    1000,
                )
                .unwrap();

            let _ = transaction
                .set_vertex(
                    &vertex_type_1_index,
                    &VertexIndex::new(vertex_index_3.index()),
                    2000,
                )
                .unwrap();
        }

        assert_eq!(
            GetVertexValue::<i32>::vertex_value(
                &vertex_store,
                &vertex_type_1_index,
                &VertexIndex::new(vertex_index_1.index())
            )
            .unwrap()
            .unwrap(),
            10i32
        );
        assert!(vertex_store
            .is_empty_vertex_element(
                &vertex_type_1_index,
                &VertexIndex::new(vertex_index_2.index() + 1)
            )
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
