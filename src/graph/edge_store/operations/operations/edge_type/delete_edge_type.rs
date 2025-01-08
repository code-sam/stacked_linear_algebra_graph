use crate::error::GraphComputingError;
use crate::graph::indexing::GetEdgeTypeIndex;

pub(crate) trait DropEdgeType {
    fn drop_valid_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}
