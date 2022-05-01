use crate::error::GraphComputingError;

use crate::graph::adjacency_matrix::AdjacencyMatrix;
use crate::graph::edge::EdgeType;
use crate::graph::graph::EdgeTypeIndex;
use crate::graph::graph::Graph;

pub trait AddEdgeType {
    fn add_edge_type(
        &mut self,
        edge_type_key: EdgeType,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<'g> AddEdgeType for Graph {
    fn add_edge_type(&mut self, edge_type: EdgeType) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_adjacency_matrix = AdjacencyMatrix::new(
            &self.graphblas_context_ref(),
            edge_type.clone(),
            self.vertex_store_ref().get_capacity()?,
        )?;
        let edge_type_index: EdgeTypeIndex = self
            .adjacency_matrices_mut_ref()
            .push(new_adjacency_matrix)?
            .into();
        self.edge_type_to_edge_type_index_map_mut_ref()
            .insert(edge_type, edge_type_index.clone());
        Ok(edge_type_index)
    }
}
