use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;

pub(crate) trait GetVectorLength {
    fn length(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
}
