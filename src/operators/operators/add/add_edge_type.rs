use crate::error::GraphComputingError;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::ValueType;

pub trait AddEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
    // fn apply_with_pre_allocated_index(&mut self, added_edge_type_index: &mut EdgeTypeIndex) -> Result<(), GraphComputingError>;
}

pub(crate) trait AddPrivateEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
    // fn apply_with_pre_allocated_index(&mut self, added_edge_type_index: &mut EdgeTypeIndex) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
