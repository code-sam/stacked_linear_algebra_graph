use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::add_edge_type::AddPrivateEdgeType as AddPrivateEdgeTypeToEdgeStore;
use crate::graph::edge_store::operations::add_edge_type::AddPublicEdgeType;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::operators::operators::add::AddPrivateEdgeType;
use crate::operators::operators::add::AddEdgeType;

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddPublicEdgeType::<T>::apply(self.edge_store_mut_ref())
    }

    // fn apply_with_pre_allocated_index(&mut self, added_edge_type_index: &mut EdgeTypeIndex) -> Result<(), GraphComputingError> {
    //     todo!()
    // }
}

impl<T: ValueType + GetValueTypeIdentifier> AddPrivateEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddPrivateEdgeTypeToEdgeStore::<T>::apply(self.edge_store_mut_ref())
    }
    
    // fn apply_with_pre_allocated_index(&mut self, added_edge_type_index: &mut EdgeTypeIndex) -> Result<(), GraphComputingError> {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {}
