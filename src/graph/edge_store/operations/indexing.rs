use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{EdgeStore, GetEdgeTypeIndicer},
        indexing::{operations::CheckIndex, EdgeTypeIndex},
    },
};

pub trait Indexing {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

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

    fn is_valid_public_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_public_index(edge_type_index)
    }

    fn try_is_valid_public_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_is_valid_public_index(edge_type_index)
    }

    fn is_valid_private_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_private_index(edge_type_index)
    }

    fn try_is_valid_private_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_is_valid_private_index(edge_type_index)
    }
}
