use crate::error::GraphComputingError;

use crate::graph::indexing::GetEdgeTypeIndex;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;

pub trait DropEdgeTypeVersioned {
    /// Deletes the edge type, and all its edges
    fn drop_edge_type(
        &mut self,
        edge_type: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
