use crate::error::GraphComputingError;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait AddPublicEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

pub(crate) trait AddPrivateEdgeType<T: ValueType> {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}
