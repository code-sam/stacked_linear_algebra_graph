use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, RegisterDeletedVertexType,
};
use crate::graph::vertex_store::operations::{
    delete_private_vertex_type, delete_private_vertex_type_unchecked, delete_public_vertex_type,
    delete_public_vertex_type_unchecked, DeleteVertexType,
};

impl<'t> DeleteVertexType<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn delete_public_vertex_type(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_public_vertex_type(self.vertex_store_mut_ref(), index)?;
        self.register_deleted_public_vertex_type(index)
    }

    fn delete_public_vertex_type_unchecked(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_public_vertex_type_unchecked(self.vertex_store_mut_ref(), index)?;
        self.register_deleted_public_vertex_type(index)
    }

    fn delete_private_vertex_type(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_private_vertex_type(self.vertex_store_mut_ref(), index)?;
        self.register_deleted_private_vertex_type(index)
    }

    fn delete_private_vertex_type_unchecked(
        &'t mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_private_vertex_type_unchecked(self.vertex_store_mut_ref(), index)?;
        self.register_deleted_private_vertex_type(index)
    }
}
