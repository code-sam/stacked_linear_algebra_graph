use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::operations::edge_type::add_edge_type::AddEdgeType as AddEdgeTypeToEdgeStore;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph_operators::operator_traits::new::NewEdgeType;

impl<T: ValueType + GetValueTypeIdentifier> NewEdgeType<T> for Graph {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToEdgeStore::<T>::apply(self.edge_store_mut_ref())
    }
}

#[cfg(test)]
mod tests {}
