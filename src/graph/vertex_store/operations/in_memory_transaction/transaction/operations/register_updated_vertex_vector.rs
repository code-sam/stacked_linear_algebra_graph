use crate::error::GraphComputingError;
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::AtomicInMemoryVertexStoreTransaction;

pub(crate) trait RegisterUpdatedVertexVector<'t> {
    fn register_updated_vertex_vector(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterUpdatedVertexVector<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_updated_vertex_vector(
        &'t mut self,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_vertex_vector_to_restore(vertex_type_index)?;
        Ok(())
    }
}
