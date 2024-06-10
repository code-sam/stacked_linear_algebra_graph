use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::edge_store::GetEdgeTypeIndicer;
use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};
use crate::graph::indexing::operations::SetIndexCapacity;
use crate::graph::indexing::ElementCount;
use crate::graph::vertex_store::operations::resize_vertex_vectors::ResizeVertexVectors;
use crate::graph::vertex_store::GetVertexElementIndexer;

pub(crate) trait SetVertexCapacity {
    fn set_vertex_capacity(
        &mut self,
        vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;

    fn set_vertex_capacity_of_vertex_vectors_and_adjacency_matrices(
        &mut self,
        vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl SetVertexCapacity for Graph {
    fn set_vertex_capacity(
        &mut self,
        vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.set_vertex_capacity_of_vertex_vectors_and_adjacency_matrices(vertex_capacity)?;
        self.vertex_store_mut_ref()
            .element_indexer_mut_ref()
            .set_index_capacity(vertex_capacity)?;
        self.edge_store_mut_ref()
            .edge_type_indexer_mut_ref()
            .set_index_capacity(vertex_capacity)?;
        Ok(())
    }

    /// Does not set capacity of element_indexer and edge_type_indexer.
    fn set_vertex_capacity_of_vertex_vectors_and_adjacency_matrices(
        &mut self,
        vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .resize_vertex_vectors(vertex_capacity)?;
        self.edge_store_mut_ref()
            .resize_adjacency_matrices(vertex_capacity)?;
        Ok(())
    }
}
