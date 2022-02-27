use crate::error::GraphComputingError;

use crate::graph::edge::EdgeTypeRef;
use crate::graph::graph::Graph;

pub trait DropEdgeType {
    fn drop_edge_type(&mut self, edge_type: &EdgeTypeRef) -> Result<(), GraphComputingError>;
}

impl DropEdgeType for Graph {
    fn drop_edge_type(&mut self, edge_type: &EdgeTypeRef) -> Result<(), GraphComputingError> {
        let edge_type_index;
        match self
            .edge_type_to_edge_type_index_map_mut_ref()
            .remove(edge_type)
        {
            Some(index) => edge_type_index = index,
            None => return Ok(()),
        }
        self.adjacency_matrices_mut_ref().free(edge_type_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::DirectedEdge;
    use crate::graph::vertex::Vertex;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_edge::ReadEdge;

    #[test]
    fn drop_edge_type() {
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
        graph.add_edge(edge_vertex2_vertex1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();

        assert!(graph.is_edge(&edge_vertex1_vertex2).unwrap());
        assert!(graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(graph.is_edge(&edge_vertex1_vertex2_type2).unwrap());

        graph
            .drop_edge_type(edge_vertex1_vertex2.edge_type_ref())
            .unwrap();
        assert!(!graph.is_edge(&edge_vertex1_vertex2).unwrap());
        assert!(!graph.is_edge(&edge_vertex2_vertex1).unwrap());
        assert!(graph.is_edge(&edge_vertex1_vertex2_type2).unwrap());
    }
}
