use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::operations::edge_type::add_edge_type::AddEdgeType as AddEdgeTypeToEdgeStore;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::new::NewEdgeType;

impl<'g, T: ValueType + GetValueTypeIdentifier> NewEdgeType<T> for InMemoryGraphTransaction<'g> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToEdgeStore::<T>::apply(&mut self.edge_store_transaction)
    }
}

#[cfg(test)]
mod tests {}
