use crate::error::GraphComputingError;
use crate::graph::indexing::operations::GetValidIndices;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction, RegisterVertexValueToRestore,
};
use crate::graph::vertex_store::operations::vertex_element::{
    CheckVertexIndex, DeleteVertexForAllTypes, DeleteVertexValue,
};
use crate::graph::vertex_store::operations::vertex_type::{CheckVertexTypeIndex, GetVertexVector};
use crate::graph::vertex_store::GetVertexTypeIndexer;

impl<'s> DeleteVertexValue for InMemoryVertexStoreTransaction<'s> {
    fn delete_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_type_index_validity(vertex_type_index)?;

        self.vertex_store_ref()
            .try_vertex_index_validity(vertex_index)?;

        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .vertex_vector_ref_unchecked(vertex_type_index);
        self.vertex_store_state_restorer
            .register_vertex_value_to_restore(vertex_vector, vertex_type_index, vertex_index)?;

        self.vertex_store_mut_ref()
            .delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }
}

impl<'s> DeleteVertexForAllTypes for InMemoryVertexStoreTransaction<'s> {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_index_validity(vertex_index)?;

        // TODO: iterate in parallel
        for vertex_type_index in self
            .vertex_store
            .vertex_type_indexer_ref()
            .iter_valid_indices()?
        {
            let vertex_type_index = VertexTypeIndex::new(vertex_type_index);

            let vertex_vector = self
                .vertex_store
                .vertex_vector_ref_unchecked(&vertex_type_index);

            self.vertex_store_state_restorer
                .register_vertex_value_to_restore(
                    vertex_vector,
                    &vertex_type_index,
                    vertex_index,
                )?;
        }

        self.vertex_store_mut_ref()
            .delete_vertex_for_all_valid_vertex_types_and_value_types(vertex_index)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    use crate::graph::indexing::{GetAssignedIndexData, VertexIndex};
    use crate::graph::vertex_store::operations::vertex_element::{AddVertex, GetVertexValue};
    use crate::graph::vertex_store::operations::vertex_type::AddVertexType;
    use crate::graph::vertex_store::VertexStore;

    use super::*;

    #[test]
    fn roll_back_deleted_vertices() {
        let mut vertex_store = initialize_vertex_store();

        let public_vertex_type_index = VertexTypeIndex::new(0);
        let private_vertex_type_index = VertexTypeIndex::new(3);

        let public_vertex_index_0 = VertexIndex::new(
            vertex_store
                .add_new_vertex(&public_vertex_type_index, 100)
                .unwrap()
                .index(),
        );
        let private_vertex_index_0 = VertexIndex::new(
            vertex_store
                .add_new_vertex(&private_vertex_type_index, 200)
                .unwrap()
                .index(),
        );

        {
            let mut transaction = InMemoryVertexStoreTransaction::new(&mut vertex_store).unwrap();

            transaction
                .delete_vertex_element(&public_vertex_type_index, &public_vertex_index_0)
                .unwrap();
            transaction
                .delete_vertex_element(&private_vertex_type_index, &private_vertex_index_0)
                .unwrap();

            assert_eq!(
                None::<i32>,
                transaction
                    .vertex_value(&public_vertex_type_index, &public_vertex_index_0)
                    .unwrap()
            );
            assert_eq!(
                None::<i32>,
                transaction
                    .vertex_value(&private_vertex_type_index, &private_vertex_index_0)
                    .unwrap()
            );
        }

        assert_eq!(
            Some(100),
            vertex_store
                .vertex_value(&public_vertex_type_index, &public_vertex_index_0)
                .unwrap()
        );
        assert_eq!(
            Some(200),
            vertex_store
                .vertex_value(&private_vertex_type_index, &private_vertex_index_0)
                .unwrap()
        );
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
