use crate::error::GraphComputingError;
use crate::graph::indexing::GetEdgeTypeIndex;

pub trait Indexing {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}
