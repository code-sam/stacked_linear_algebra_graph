use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::operations::edge_type::add_edge_type::AddEdgeType as AddEdgeTypeToEdgeStore;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::operators::operators::add::AddEdgeType;

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToEdgeStore::<T>::apply(self.edge_store_mut_ref())
    }

    // fn apply_with_pre_allocated_index(&mut self, added_edge_type_index: &mut EdgeTypeIndex) -> Result<(), GraphComputingError> {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {}
