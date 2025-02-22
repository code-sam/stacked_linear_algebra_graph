use crate::error::GraphComputingError;
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::vertex_type::AddVertexType as AddVertexTypeToVertexStore;
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::new::NewVertexType;

impl<'g, T: ValueType + GetValueTypeIdentifier> NewVertexType<T> for InMemoryGraphTransaction<'g> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddVertexTypeToVertexStore::<T>::apply(&mut self.vertex_store_transaction)
    }
}
