use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::indexing::operations::{CheckIndex, FreeIndex};
use crate::graph::indexing::EdgeTypeIndex;
use crate::{error::GraphComputingError, graph::edge_store::EdgeStore};

pub(crate) trait DropEdgeType {
    fn drop_valid_public_edge_type(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
    fn drop_valid_private_edge_type(
        &mut self,
        ede_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn drop_edge_type_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DropEdgeType for EdgeStore {
    fn drop_valid_public_edge_type(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .try_is_valid_public_index(edge_type_index)?;
        self.drop_edge_type_unchecked(edge_type_index)
    }

    fn drop_valid_private_edge_type(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .try_is_valid_private_index(edge_type_index)?;
        self.drop_edge_type_unchecked(edge_type_index)
    }

    fn drop_edge_type_unchecked(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index_unchecked(*index)
    }
}
