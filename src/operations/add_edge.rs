use crate::error::GraphComputingError;

use crate::graph::edge::DirectedEdge;
use crate::graph::graph::EdgeTypeIndex;
use crate::graph::graph::Graph;

use super::add_edge_type::AddEdgeType;

// TODO: make the interface more pecise, and similar to add_vertex.
pub trait AddEdge {
    fn add_edge(
        &mut self,
        edge: DirectedEdge,
    ) -> Result<Option<EdgeTypeIndex>, GraphComputingError>;
}

impl AddEdge for Graph {
    fn add_edge(
        &mut self,
        edge: DirectedEdge,
    ) -> Result<Option<EdgeTypeIndex>, GraphComputingError> {
        let edge_type_index: EdgeTypeIndex;
        let index_to_return: Option<EdgeTypeIndex>;
        match self
            .edge_type_to_edge_type_index_map_ref()
            .get(edge.edge_type_ref())
        {
            None => {
                edge_type_index = self.add_new_edge_type(edge.edge_type_ref().to_owned())?; // TODO: review the use of add_NEW_edge_type
                index_to_return = Some(edge_type_index.clone());
            }
            Some(index) => {
                edge_type_index = index.clone(); // REVIEW: cloning seems inefficient but required but set_edge_in_adjacency_matrix() takes a mutable borrow of self, which the index otherwise references
                index_to_return = None;
            }
        }
        self.set_edge_in_adjacency_matrix(&edge, edge_type_index)?;
        Ok(index_to_return)
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
    }

    #[test]
    fn add_edge_errors() {
        let mut graph = Graph::new(5, 5).unwrap();

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            String::from("edge_type_1"),
        );

        match graph.add_edge(edge_vertex1_vertex2.clone()) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        match graph.add_edge(edge_vertex1_vertex2) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }
}
