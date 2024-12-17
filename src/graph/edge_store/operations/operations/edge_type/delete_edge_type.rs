use crate::error::GraphComputingError;
use crate::graph::indexing::GetEdgeTypeIndex;

pub(crate) trait DropEdgeType {
    fn drop_valid_public_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
    fn drop_valid_private_edge_type(
        &mut self,
        ede_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}
