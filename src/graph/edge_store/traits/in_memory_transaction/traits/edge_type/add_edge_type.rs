use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::in_memory_transaction::{
    GetEdgeStore, GetEdgeStoreStateRestorer, InMemoryEdgeStoreTransaction,
    RegisterNewEdgeTypeToRevert,
};
use crate::graph::edge_store::traits::traits::edge_type::add_edge_type::AddEdgeType;
use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::indexing::traits::GenerateIndex;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};

impl<'s, T: ValueType + GetValueTypeIdentifier> AddEdgeType<T>
    for InMemoryEdgeStoreTransaction<'s>
{
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_type_index = self
            .edge_store_mut_ref()
            .edge_type_indexer_mut_ref()
            .new_index()?;
        self.edge_store_state_restorer_mut_ref()
            .register_new_edge_type_to_revert(&new_type_index);
        let edge_type_index = self
            .edge_store_mut_ref()
            .add_edge_type_at_assigned_index::<T>(new_type_index)?;
        Ok(edge_type_index)
    }
}
