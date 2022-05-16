use crate::error::GraphComputingError;

use crate::graph::edge::{DirectedEdge, EdgeTypeRef};
use crate::graph::graph::Graph;

pub trait ReadEdge {
    fn is_edge(&self, edge: &DirectedEdge) -> Result<bool, GraphComputingError>;

    // TODO: review placement of this function
    fn is_edge_type(&self, edge_type: &EdgeTypeRef) -> Result<bool, GraphComputingError>;
}

impl ReadEdge for Graph {
    fn is_edge(&self, edge: &DirectedEdge) -> Result<bool, GraphComputingError> {
        let edge_coordinate;
        match self.get_edge_coordinate(edge) {
            Ok(coordinate) => edge_coordinate = coordinate,
            Err(_) => return Ok(false), // TODO: match error type to decide if an error should be passed on instead
        }
        match self.get_edge_adjacency_matrix_ref(edge.edge_type_ref()) {
            Ok(edge_adjacency_matrix) => edge_adjacency_matrix.is_edge(&edge_coordinate),
            Err(_) => Ok(false), // TODO: match error type to decide if an error should be passed on instead
        }
    }

    // fn is_edge_by_coordinate(&self, edge_coordinate: &EdgeCoordinate) -> Result<bool, GraphComputingError> {
    //     match self.get_edge_adjacency_matrix(edge) {
    //         Ok(edge_adjacency_matrix) => edge_adjacency_matrix.is_edge(&edge_coordinate),
    //         Err(error) => Ok(false), // TODO: match error type to decide if an error should be passed on instead
    //     }
    // }

    fn is_edge_type(&self, edge_type: &EdgeTypeRef) -> Result<bool, GraphComputingError> {
        Ok(self
            .edge_type_to_edge_type_index_map_ref()
            .contains_key(edge_type))
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
            .is_edge_type(String::from("this_edge_type_does_not_exist").as_str())
            .unwrap());
        assert!(graph.is_edge_type(String::from("is_a").as_str()).unwrap());
    }
}
