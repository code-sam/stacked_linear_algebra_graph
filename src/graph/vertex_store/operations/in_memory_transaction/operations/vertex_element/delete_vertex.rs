use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{
    CheckIndex, GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices,
};
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, RegisterVertexValueToRestore,
};
use crate::graph::vertex_store::operations::vertex_element::{
    DeleteVertexForAllTypes, DeleteVertexValue,
};
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::GetVertexTypeIndexer;

impl<'s> DeleteVertexValue for AtomicInMemoryVertexStoreTransaction<'s> {
    fn delete_public_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_private_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())?;
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

impl<'s> DeleteVertexForAllTypes for AtomicInMemoryVertexStoreTransaction<'s> {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
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

    fn delete_vertex_for_all_valid_public_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        // TODO: iterate in parallel
        for vertex_type_index in self
            .vertex_store
            .vertex_type_indexer_ref()
            .iter_valid_public_indices()?
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
            .delete_vertex_for_all_valid_public_vertex_types_and_value_types(vertex_index)
    }

    fn delete_vertex_for_all_valid_private_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        // TODO: iterate in parallel
        for vertex_type_index in self
            .vertex_store
            .vertex_type_indexer_ref()
            .iter_valid_private_indices()?
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
            .delete_vertex_for_all_valid_private_vertex_types_and_value_types(vertex_index)
    }
}
