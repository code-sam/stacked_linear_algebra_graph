use crate::error::GraphComputingError;

use crate::graph::edge::DirectedEdge;
use crate::graph::graph::Graph;

pub trait DeleteEdge {
    fn delete_edge(&mut self, edge_to_delete: &DirectedEdge) -> Result<(), GraphComputingError>;
    // fn delete_selected_edges(&mut self, edge_selection_to_delete: &EdgeSelection) -> Result<(), GraphComputingError>;
}

impl DeleteEdge for Graph {
    fn delete_edge(&mut self, edge_to_delete: &DirectedEdge) -> Result<(), GraphComputingError> {
        let edge_coordinate_to_delete = self.get_edge_coordinate(edge_to_delete)?;
        let adjacency_matrix_of_edge_to_delete =
            self.get_edge_adjacency_matrix_mut_ref(edge_to_delete.edge_type_ref())?;
        adjacency_matrix_of_edge_to_delete.delete_edge(&edge_coordinate_to_delete)?;
        Ok(())
    }

    // TODO: delete by edge_type index and coordinate
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::vertex::Vertex;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_edge::ReadEdge;

    #[test]
    fn delete_edge() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            String::from("edge_type_1"),
        );
        let edge_vertex2_vertex1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            String::from("edge_type_1"),
        );
        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            String::from("edge_type_2"),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2.clone()).unwrap();
        assert_eq!(graph.is_edge(&edge_vertex1_vertex2).unwrap(), true);
        assert!(!graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(!graph.is_edge(&edge_vertex1_vertex2_type2).unwrap());

        graph.add_edge(edge_vertex1_vertex2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1.clone()).unwrap();
        assert!(graph.is_edge(&edge_vertex1_vertex2).unwrap());
        assert!(graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(!graph.is_edge(&edge_vertex1_vertex2_type2).unwrap());

        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        assert!(graph.is_edge(&edge_vertex1_vertex2).unwrap());
        assert!(graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(graph.is_edge(&edge_vertex1_vertex2_type2).unwrap());

        graph.delete_edge(&edge_vertex1_vertex2).unwrap();
        assert!(!graph.is_edge(&edge_vertex1_vertex2).unwrap());
        assert!(graph.is_edge(&edge_vertex2_vertex1).unwrap());
        graph.delete_edge(&edge_vertex2_vertex1).unwrap();
        assert!(!graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(graph.is_edge(&edge_vertex1_vertex2_type2).unwrap())
    }

    #[test]
    fn delete_non_existing_edge() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            String::from("edge_type_1"),
        );

        let result = graph.delete_edge(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();

        let result = graph.delete_edge(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }

        // Deleting non-existing edge, connecting existing edges
        let result = graph.delete_edge(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
