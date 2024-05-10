use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{EdgeStore, GetEdgeTypeIndicer},
        indexing::{operations::CheckIndex, EdgeTypeIndex, GetEdgeTypeIndex},
    },
};

pub trait Indexing {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl Indexing for EdgeStore {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_index(edge_type_index.index_ref())
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index_ref())
    }

    fn is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_public_index(edge_type_index.index_ref())
    }

    fn try_is_valid_public_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_is_valid_public_index(edge_type_index.index_ref())
    }

    fn is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_private_index(edge_type_index.index_ref())
    }

    fn try_is_valid_private_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_is_valid_private_index(edge_type_index.index_ref())
    }
}
