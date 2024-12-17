use crate::graph::edge_store::operations::operations::edge_type::delete_edge_type::DropEdgeType;
use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::indexing::operations::{CheckIndex, FreeIndex};
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::edge_store::EdgeStore};

impl DropEdgeType for EdgeStore {
    fn drop_valid_public_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .try_is_valid_public_index(edge_type_index.index())?;
        self.edge_type_indexer_mut_ref()
            .free_public_index_unchecked(edge_type_index.index())
    }

    fn drop_valid_private_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .try_is_valid_private_index(edge_type_index.index())?;
        self.edge_type_indexer_mut_ref()
            .free_private_index_unchecked(edge_type_index.index())
    }
}
