use crate::error::GraphComputingError;

use crate::graph::edge::{
    DirectedEdgeDefinedByIndices, DirectedEdgeDefinedByKeys, EdgeToEdgeCoordinate,
};
use crate::graph::graph::Graph;

pub trait DeleteEdge {
    fn delete_edge_defined_by_keys(
        &mut self,
        edge_to_delete: &DirectedEdgeDefinedByKeys,
    ) -> Result<(), GraphComputingError>;
    fn delete_edge_defined_by_indices(
        &mut self,
        edge_to_delete: &DirectedEdgeDefinedByIndices,
    ) -> Result<(), GraphComputingError>;
    // fn delete_selected_edges(&mut self, edge_selection_to_delete: &EdgeSelection) -> Result<(), GraphComputingError>;
}

impl DeleteEdge for Graph {
    fn delete_edge_defined_by_keys(
        &mut self,
        edge_to_delete: &DirectedEdgeDefinedByKeys,
    ) -> Result<(), GraphComputingError> {
        let edge_coordinate_to_delete = self.key_defined_edge_to_edge_coordinate(edge_to_delete)?;
        let adjacency_matrix_of_edge_to_delete =
            self.get_edge_adjacency_matrix_mut_ref(edge_to_delete.edge_type_ref())?;
        adjacency_matrix_of_edge_to_delete.delete_edge(&edge_coordinate_to_delete)?;
        Ok(())
    }

    fn delete_edge_defined_by_indices(
        &mut self,
        edge_to_delete: &DirectedEdgeDefinedByIndices,
    ) -> Result<(), GraphComputingError> {
        let edge_coordinate_to_delete =
            self.index_defined_edge_to_edge_coordinate(edge_to_delete)?;
        let adjacency_matrix_of_edge_to_delete = self
            .adjacency_matrices_mut_ref()
            .get_mut_ref(edge_to_delete.edge_type().clone())?;
        adjacency_matrix_of_edge_to_delete.delete_edge(&edge_coordinate_to_delete)?;
        Ok(())
    }
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

        graph
            .delete_edge_defined_by_keys(&edge_vertex1_vertex2)
            .unwrap();
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        graph
            .delete_edge_defined_by_keys(&edge_vertex2_vertex1)
            .unwrap();
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap())
    }

    #[test]
    fn delete_non_existing_edge() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_1"),
            vertex_2.clone().into(),
        );

        let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();

        let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }

        // Deleting non-existing edge, connecting existing edges
        let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
        match result {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
