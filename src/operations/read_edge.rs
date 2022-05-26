use crate::error::GraphComputingError;

use crate::graph::edge::{
    DirectedEdgeDefinedByIndices, DirectedEdgeDefinedByKeys, EdgeToEdgeCoordinate, EdgeTypeIndex,
    EdgeTypeRef,
};
use crate::graph::graph::Graph;
use crate::operations::read_vertex_value::ReadVertexValue;

pub trait ReadEdge {
    fn is_key_defined_edge_in_graph(
        &self,
        edge: &DirectedEdgeDefinedByKeys,
    ) -> Result<bool, GraphComputingError>;
    fn is_index_defined_edge_in_graph(
        &self,
        edge: &DirectedEdgeDefinedByIndices,
    ) -> Result<bool, GraphComputingError>;

    // TODO: review placement of this function
    fn is_edge_type_in_graph(
        &self,
        edge_type: &EdgeTypeRef,
    ) -> Result<bool, GraphComputingError>;
    fn is_index_defined_edge_type_in_graph(
        &self,
        edge_type: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;
}

impl ReadEdge for Graph {
    fn is_key_defined_edge_in_graph(
        &self,
        edge: &DirectedEdgeDefinedByKeys,
    ) -> Result<bool, GraphComputingError> {
        if self.is_valid_vertex_key(edge.originates_from_vertex()) && self.is_valid_vertex_key(edge.points_to_vertex()) {
            let edge_coordinate = self.key_defined_edge_to_edge_coordinate(edge)?;
            match self.get_edge_adjacency_matrix_ref(edge.edge_type_ref()) {
                Ok(edge_adjacency_matrix) => edge_adjacency_matrix.is_edge(&edge_coordinate),
                Err(_) => Ok(false), // TODO: match error type to decide if an error should be passed on instead
            }
        } else {
            return Ok(false)
        }
    }

    fn is_index_defined_edge_in_graph(
        &self,
        edge: &DirectedEdgeDefinedByIndices,
    ) -> Result<bool, GraphComputingError> {
        let edge_coordinate = self.index_defined_edge_to_edge_coordinate(edge)?;
        match self
            .adjacency_matrices_ref()
            .get_ref(edge.edge_type().clone())
        {
            Ok(edge_adjacency_matrix) => edge_adjacency_matrix.is_edge(&edge_coordinate),
            Err(_) => Ok(false), // TODO: match error type to decide if an error should be passed on instead
        }
    }

    fn is_edge_type_in_graph(
        &self,
        edge_type: &EdgeTypeRef,
    ) -> Result<bool, GraphComputingError> {
        Ok(self
            .edge_type_to_edge_type_index_map_ref()
            .contains_key(edge_type))
    }

    fn is_index_defined_edge_type_in_graph(
        &self,
        edge_type: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(self.adjacency_matrices_ref().is_valid_index(edge_type)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn test_is_edge_type() {
        let graph = standard_graph_for_testing();

        assert!(!graph
            .is_edge_type_in_graph(
                String::from("this_edge_type_does_not_exist").as_str()
            )
            .unwrap());
        assert!(graph
            .is_edge_type_in_graph(String::from("is_a").as_str())
            .unwrap());
    }
}
