use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction, RegisterVertexVectorToRestore,
};
use crate::graph::vertex_store::operations::vertex_type::{
    delete_private_vertex_type_unchecked, delete_public_vertex_type_unchecked, DeleteVertexType,
    GetVertexVector,
};

impl<'t> DeleteVertexType<'t> for InMemoryVertexStoreTransaction<'t> {
    fn delete_public_vertex_type(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        register_deleted_public_vertex_vector_to_restore(self, index)?;
        delete_public_vertex_type_unchecked(self.vertex_store_mut_ref(), index)
    }

    fn delete_public_vertex_type_unchecked(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        register_deleted_public_vertex_vector_to_restore_unchecked(self, index)?;
        delete_public_vertex_type_unchecked(self.vertex_store_mut_ref(), index)
    }

    fn delete_private_vertex_type(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        register_deleted_private_vertex_vector_to_restore(self, index)?;
        delete_private_vertex_type_unchecked(self.vertex_store_mut_ref(), index)
    }

    fn delete_private_vertex_type_unchecked(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        register_deleted_private_vertex_vector_to_restore_unchecked(self, index)?;
        delete_private_vertex_type_unchecked(self.vertex_store_mut_ref(), index)
    }
}

fn register_deleted_private_vertex_vector_to_restore<'s>(
    transaction: &mut InMemoryVertexStoreTransaction<'s>,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    let vertex_vector = transaction
        .vertex_store
        .private_vertex_vector_mut_ref(vertex_type_index)?;

    transaction
        .vertex_store_state_restorer
        .register_deleted_private_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
    Ok(())
}

fn register_deleted_public_vertex_vector_to_restore<'s>(
    transaction: &mut InMemoryVertexStoreTransaction<'s>,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    let vertex_vector = transaction
        .vertex_store
        .public_vertex_vector_mut_ref(vertex_type_index)?;

    transaction
        .vertex_store_state_restorer
        .register_deleted_public_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
    Ok(())
}

fn register_deleted_public_vertex_vector_to_restore_unchecked<'s>(
    transaction: &mut InMemoryVertexStoreTransaction<'s>,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    let vertex_vector = transaction
        .vertex_store
        .vertex_vector_mut_ref_unchecked(vertex_type_index)?;

    transaction
        .vertex_store_state_restorer
        .register_deleted_public_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
    Ok(())
}

fn register_deleted_private_vertex_vector_to_restore_unchecked<'s>(
    transaction: &mut InMemoryVertexStoreTransaction<'s>,
    vertex_type_index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    let vertex_vector = transaction
        .vertex_store
        .vertex_vector_mut_ref_unchecked(vertex_type_index)?;

    transaction
        .vertex_store_state_restorer
        .register_deleted_private_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
    Ok(())
}
