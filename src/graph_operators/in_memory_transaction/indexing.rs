use crate::error::GraphComputingError;
use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge_store::traits::traits::edge_type::indexing::Indexing as EdgeStoreIndexing;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::traits::vertex_type::CheckVertexTypeIndex;
use crate::graph_operators::operator_traits::indexing::CheckIndex;
use crate::transaction::in_memory::InMemoryGraphTransaction;

impl<'g> CheckIndex for InMemoryGraphTransaction<'g> {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_transaction
            .is_valid_vertex_index(vertex_index)
    }

    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_transaction
            .is_valid_vertex_type_index(vertex_type_index)
    }

    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_transaction
            .is_valid_edge_type_index(edge_type_index)
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction
            .try_vertex_index_validity(vertex_index)
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction
            .try_vertex_type_index_validity(vertex_type_index)
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_transaction
            .try_edge_type_index_validity(edge_type_index)
    }

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&impl GetEdgeTypeIndex>,
    ) -> Result<(), GraphComputingError> {
        match edge_type_index {
            Some(edge_type_index) => self.try_edge_type_index_validity(edge_type_index),
            None => Ok(()),
        }
    }

    fn try_optional_vertex_type_index_validity(
        &self,
        vertex_type_index: Option<&impl GetVertexTypeIndex>,
    ) -> Result<(), GraphComputingError> {
        match vertex_type_index {
            Some(vertex_type_index) => self.try_vertex_type_index_validity(vertex_type_index),
            None => Ok(()),
        }
    }

    fn is_valid_edge(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(self.is_valid_edge_type_index(edge_type)?
            && self.is_valid_vertex_index(tail)?
            && self.is_valid_vertex_index(head)?)
    }

    fn is_valid_edge_coordinate(
        &self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<bool, GraphComputingError> {
        self.is_valid_edge(edge.edge_type_ref(), edge.tail_ref(), edge.head_ref())
    }

    fn try_edge_validity(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
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
        self.try_edge_validity(edge.edge_type_ref(), edge.tail_ref(), edge.head_ref())
    }
}

#[cfg(test)]
mod tests {}
