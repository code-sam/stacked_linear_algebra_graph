use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::map::MapMutableAdjacencyMatrices,
            weighted_adjacency_matrix::operations::DeleteVertexConnections,
        },
        graph::{GetEdgeStore, GetVertexStore},
        indexing::{GetVertexIndexIndex, GetVertexTypeIndex},
        vertex_store::{
            DeleteVertexForAllTypes, DeleteVertexValue as DeleteVertexValueFromVertexStore,
        },
    },
};

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::Graph;

pub trait DropVertexIndex {
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DropPrivateVertexIndex {
    fn drop_private_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError>;
}

pub trait DeleteVertexValue {
    fn delete_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_element_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeletePrivateVertexValue {
    fn delete_private_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_element_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
