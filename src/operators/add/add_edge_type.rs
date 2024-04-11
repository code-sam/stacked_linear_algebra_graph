use crate::error::GraphComputingError;

use crate::graph::edge::EdgeTypeIndex;
use crate::graph::edge_store::operations::add_edge_type::AddEdgeType as AddEdgeTypeToStore;

use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};

pub trait AddEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToStore::<T>::add_new_edge_type(self.edge_store_mut_ref())
    }
}

#[cfg(test)]
mod tests {}
