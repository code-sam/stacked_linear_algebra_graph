use crate::error::GraphComputingError;

use crate::graph::edge::DirectedEdge;
use crate::graph::graph::Graph;

pub trait ReadEdge {
    fn is_edge(&self, edge: &DirectedEdge) -> Result<bool, GraphComputingError>;
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
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::graph::vertex::VertexValue;

    // TODO
    // #[test]
    // fn new_graph() {
    //     let graph = Graph::new(10, 20);
    // }
}
