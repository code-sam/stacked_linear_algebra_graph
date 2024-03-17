use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        edge_store::{EdgeStore, GetEdgeTypeIndicer},
        indexer::IndexerTrait,
    },
};

pub trait Indexing {
    fn is_valid_edge_type_index(
        &self,
        vertex_key: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

// TODO: where applicable, move implementations down to store level
impl Indexing for EdgeStore {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref().is_valid_index(edge_type_index)
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index)
    }
}
