use crate::error::GraphComputingError;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};

pub trait Indexing {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&EdgeTypeIndex>,
    ) -> Result<(), GraphComputingError>;
}
