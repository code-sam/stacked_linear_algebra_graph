use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::indexer::FreeIndex;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, edge_store::EdgeStore},
};

pub(crate) trait DropEdgeType {
    fn drop_edge_type(&mut self, index: &EdgeTypeIndex) -> Result<(), GraphComputingError>;

    fn drop_edge_type_unchecked(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DropEdgeType for EdgeStore {
    fn drop_edge_type(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index(*edge_type_index)
    }

    fn drop_edge_type_unchecked(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index_unchecked(*index)
    }
}
