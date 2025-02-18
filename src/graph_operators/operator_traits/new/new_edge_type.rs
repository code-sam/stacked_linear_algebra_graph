use crate::error::GraphComputingError;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;


pub trait NewEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
