use crate::error::GraphComputingError;
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    RegisterNewVertexTypeToRevert,
};
use crate::graph::vertex_store::operations::vertex_type::{
    add_public_vertex_type, AddPrivateVertexType, AddPublicVertexType,
};

impl<'t, T: ValueType + GetValueTypeIdentifier> AddPublicVertexType<'t, T>
    for AtomicInMemoryVertexStoreTransaction<'t>
{
    fn apply(&'t mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let new_vertex_type_index = add_public_vertex_type::<T>(self.vertex_store_mut_ref())?;
        self.vertex_store_state_restorer_mut_ref()
            .register_new_public_vertex_type_to_revert(&new_vertex_type_index)?;
        Ok(VertexTypeIndex::from(new_vertex_type_index))
    }
}

impl<'t, T: ValueType + GetValueTypeIdentifier> AddPrivateVertexType<'t, T>
    for AtomicInMemoryVertexStoreTransaction<'t>
{
    fn apply(&'t mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        let new_vertex_type_index = add_public_vertex_type::<T>(self.vertex_store_mut_ref())?;
        self.vertex_store_state_restorer_mut_ref()
            .register_new_private_vertex_type_to_revert(&new_vertex_type_index)?;
        Ok(VertexTypeIndex::from(new_vertex_type_index))
    }
}
