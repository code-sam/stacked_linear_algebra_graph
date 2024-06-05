use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::delete_edge_type::DropEdgeType as DropEdgeTypeFromEdgeStore;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::GetEdgeTypeIndex;

pub trait DropEdgeType {
    /// Deletes the edge type, and all its edges
    fn drop_edge_type(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DropPrivateEdgeType {
    /// Deletes the edge type, and all its edges
    fn drop_private_edge_type(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
