use crate::{graph::{edge::{EdgeTypeKeyRef, EdgeTypeIndex}, edge_store::EdgeStore, indexer::{IndexerTrait, NewIndexTrait}}, error::GraphComputingError};
use crate::graph::edge_store::edge_store::EdgeStoreTrait;

pub(crate) trait DeleteType {
    fn add_new_edge_type_with_key(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn add_new_edge_type_with_index(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteType for EdgeStore {
    fn add_new_edge_type(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        let new_type_index = self.edge_type_indexer_mut_ref().dele(key)?;
        Ok(())
    }
}
