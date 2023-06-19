use crate::graph::edge_store::edge_store::EdgeStoreTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        edge::{EdgeTypeIndex, EdgeTypeKeyRef},
        edge_store::EdgeStore,
        indexer::IndexerTrait,
    },
};

pub(crate) trait DeleteType {
    fn delete_edge_type_with_key(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn delete_edge_type_with_index(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteType for EdgeStore {
    fn delete_edge_type_with_key(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        let index = *self.edge_type_indexer_ref().try_index_for_key(key)?;
        self.edge_type_indexer_mut_ref().free_index_unchecked(index)
    }

    fn delete_edge_type_with_index(
        &mut self,
        index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_mut_ref()
            .free_index_unchecked(*index)
    }
}
