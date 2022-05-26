use crate::error::{GraphComputingError, LogicError, LogicErrorType, SystemError, SystemErrorType};

use crate::graph::edge::adjacency_matrix::EdgeCoordinate;
use crate::graph::edge::{
    DirectedEdgeDefinedByIndices, DirectedEdgeDefinedByKeys, EdgeToEdgeCoordinate, EdgeTypeIndex,
};
use crate::graph::graph::Graph;

use super::add_edge_type::AddEdgeType;

pub trait AddEdge {
    fn add_edge_using_keys(
        &mut self,
        edge: DirectedEdgeDefinedByKeys,
    ) -> Result<(), GraphComputingError>;

    /// If the EdgeType already exists, then the edge is added to it.
    /// Existing edges for the EdgesType remain unaffected.
    fn add_edge_and_edge_type_using_keys(
        &mut self,
        edge: DirectedEdgeDefinedByKeys,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;

    fn add_edge_using_indices(
        &mut self,
        edge: DirectedEdgeDefinedByIndices,
    ) -> Result<(), GraphComputingError>;
}

impl AddEdge for Graph {
    fn add_edge_using_keys(
        &mut self,
        edge: DirectedEdgeDefinedByKeys,
    ) -> Result<(), GraphComputingError> {
        let edge_type_index: EdgeTypeIndex;
        match self
            .edge_type_to_edge_type_index_map_ref()
            .get(edge.edge_type_ref())
        {
            None => {
                return Err(LogicError::new(
                    LogicErrorType::EdgeTypeMustExist,
                    format!("EdgeType \"{}\" does not exist", edge.edge_type_ref()),
                    None,
                )
                .into())
            }
            Some(index) => {
                edge_type_index = index.clone(); // REVIEW: cloning seems inefficient but required but set_edge_in_adjacency_matrix() takes a mutable borrow of self, which the index otherwise references
            }
        }
        let edge_coordinate = self.key_defined_edge_to_edge_coordinate(&edge)?;
        self.set_edge_in_adjacency_matrix(&edge_coordinate, edge_type_index.clone())?; // TODO: by index, and by key
        Ok(())
    }

    /// If the EdgeType already exists, then the edge is added to it.
    /// Existing edges for the EdgesType remain unaffected.
    fn add_edge_and_edge_type_using_keys(
        &mut self,
        edge: DirectedEdgeDefinedByKeys,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        let edge_type_index: EdgeTypeIndex;
        match self
            .edge_type_to_edge_type_index_map_ref()
            .get(edge.edge_type_ref())
        {
            None => {
                edge_type_index = self.add_new_edge_type(edge.edge_type_ref().to_owned())?;
            }
            Some(index) => {
                edge_type_index = index.clone(); // REVIEW: cloning seems inefficient but required but set_edge_in_adjacency_matrix() takes a mutable borrow of self, which the index otherwise references
            }
        }
        let edge_coordinate = self.key_defined_edge_to_edge_coordinate(&edge)?;
        self.set_edge_in_adjacency_matrix(&edge_coordinate, edge_type_index.clone())?;
        Ok(edge_type_index)
    }

    fn add_edge_using_indices(
        &mut self,
        edge: DirectedEdgeDefinedByIndices,
    ) -> Result<(), GraphComputingError> {
        let edge_coordinate = self.index_defined_edge_to_edge_coordinate(&edge)?;
        self.set_edge_in_adjacency_matrix(&edge_coordinate, edge.edge_type().clone())?;
        Ok(())
    }
}

impl Graph {
    fn set_edge_in_adjacency_matrix(
        &mut self,
        edge_coordinate: &EdgeCoordinate,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        match self
            .adjacency_matrices_mut_ref()
            .get_mut_ref(edge_type_index)
        {
            Ok(edge_type_adjacency_matrix) => {
                edge_type_adjacency_matrix.add_edge(&edge_coordinate)?
            }
            Err(_) => {
                // TODO: check actual error type
                return Err(SystemError::new(
                    SystemErrorType::IndexOutOfBounds,
                    format!(
                        "Unable to access adjacency matrix at index: {}",
                        edge_type_index.index_ref()
                    ),
                    None,
                )
                .into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::graph::Graph;
    use crate::graph::vertex::Vertex;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_edge::ReadEdge;

    #[test]
    fn add_edge() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_1"),
            vertex_2.clone().into(),
        );
        let edge_vertex2_vertex1 = DirectedEdgeDefinedByKeys::new(
            vertex_2.clone().into(),
            String::from("edge_type_1"),
            vertex_1.clone().into(),
        );
        let edge_vertex1_vertex2_type2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_2"),
            vertex_2.clone().into(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        assert_eq!(
            graph
                .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
                .unwrap(),
            true
        );
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap());

        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap());

        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
            .unwrap();
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap());
    }

    #[test]
    fn add_edge_errors() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_1"),
            vertex_2.clone().into(),
        );

        match graph.add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone()) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        match graph.add_edge_and_edge_type_using_keys(edge_vertex1_vertex2) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }
}
