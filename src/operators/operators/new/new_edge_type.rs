use crate::error::GraphComputingError;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::ValueType;

pub trait NewEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

pub(crate) trait NewPrivateEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
