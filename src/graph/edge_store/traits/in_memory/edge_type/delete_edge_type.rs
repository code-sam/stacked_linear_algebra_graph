use crate::graph::edge_store::traits::traits::edge_type::delete_edge_type::DropEdgeType;
use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::indexing::traits::{CheckIndex, FreeIndex};
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::edge_store::EdgeStore};

impl DropEdgeType for EdgeStore {
    fn drop_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .try_index_validity(edge_type_index.index())?;
        self.drop_edge_type_unchecked(edge_type_index)
    }

    fn drop_edge_type_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index_unchecked(edge_type_index.index())
    }
}
