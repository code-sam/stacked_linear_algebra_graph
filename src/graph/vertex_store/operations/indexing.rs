use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{operations::CheckIndex, VertexIndex, VertexTypeIndex},
        vertex_store::{GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore},
    },
};

pub trait CheckVertexTypeIndex {
    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl CheckVertexTypeIndex for VertexStore {
    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .is_valid_index(vertex_type_index)
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)
    }

    fn is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .is_valid_public_index(vertex_type_index)
    }

    fn try_is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index)
    }

    fn is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .is_valid_private_index(vertex_type_index)
    }

    fn try_is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index)
    }
}

pub trait CheckVertexIndex {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl CheckVertexIndex for VertexStore {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref().is_valid_index(vertex_index)
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref().try_index_validity(vertex_index)
    }

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_public_index(vertex_index)
    }

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_is_valid_public_index(vertex_index)
    }

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_private_index(vertex_index)
    }

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_is_valid_private_index(vertex_index)
    }
}
