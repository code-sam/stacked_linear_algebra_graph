use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
};
use crate::graph::vertex_store::operations::GetVertexVector;
use crate::graph::vertex_store::ToSparseVector;

pub(crate) trait RegisterVertexVectorToRestore<'s> {
    fn register_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'s> RegisterVertexVectorToRestore<'s> for AtomicInMemoryVertexStoreTransaction<'s> {
    fn register_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_vertex_vector_to_restore(vertex_type_index)
    }
}
