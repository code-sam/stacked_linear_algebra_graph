use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::add_edge_type::AddPrivateEdgeType as AddPrivateEdgeTypeToEdgeStore;
use crate::graph::edge_store::operations::add_edge_type::AddPublicEdgeType;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};

pub trait AddEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

pub(crate) trait AddPrivateEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddPublicEdgeType::<T>::apply(self.edge_store_mut_ref())
    }
}

impl<T: ValueType + GetValueTypeIdentifier> AddPrivateEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddPrivateEdgeTypeToEdgeStore::<T>::apply(self.edge_store_mut_ref())
    }
}

#[cfg(test)]
mod tests {}
