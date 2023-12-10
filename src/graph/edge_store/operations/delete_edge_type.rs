use crate::graph::edge_store::edge_store::EdgeStoreTrait;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, edge_store::EdgeStore, indexer::IndexerTrait},
};

pub(crate) trait DeleteType {
    fn delete_edge_type_with_index(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteType for EdgeStore {
    fn delete_edge_type_with_index(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index_unchecked(*index)
    }
}
