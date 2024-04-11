use crate::error::GraphComputingError;
use crate::graph::edge::{EdgeTypeIndex, GetDirectedEdgeCoordinateIndex};
use crate::graph::edge_store::operations::indexing::Indexing as EdgeStoreIndexing;
use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph, VertexIndex, VertexTypeIndex};
use crate::graph::indexer::CheckIndex;
use crate::graph::vertex_store::VertexStoreTrait;

pub trait Indexing {
    fn is_valid_vertex_index(&self, vertex_key: &VertexIndex) -> Result<bool, GraphComputingError>;
    fn is_valid_vertex_type_index(
        &self,
        vertex_key: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_edge_type_index(
        &self,
        vertex_key: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_edge(
        &self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn is_valid_edge_coordinate(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_edge_validity(
        &self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_coordinate_validity(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError>;
}

// TODO: where applicable, move implementations down to store level
impl Indexing for Graph {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_index(vertex_index)
    }

    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_index(vertex_type_index)
    }

    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref()
            .is_valid_edge_type_index(edge_type_index)
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_index_validity(vertex_index)
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_edge_type_index_validity(edge_type_index)
    }

    fn is_valid_edge(
        &self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(self.is_valid_edge_type_index(edge_type)?
            && self.is_valid_vertex_index(tail)?
            && self.is_valid_vertex_index(head)?)
    }

    fn is_valid_edge_coordinate(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(self.is_valid_edge_type_index(edge.edge_type_ref())?
            && self.is_valid_vertex_index(edge.tail_ref())?
            && self.is_valid_vertex_index(edge.head_ref())?)
    }

    fn try_edge_validity(
        &self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(edge_type)?;
        self.try_vertex_index_validity(tail)?;
        self.try_vertex_index_validity(head)?;
        Ok(())
    }

    fn try_edge_coordinate_validity(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(edge.edge_type_ref())?;
        self.try_vertex_index_validity(edge.tail_ref())?;
        self.try_vertex_index_validity(edge.head_ref())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
