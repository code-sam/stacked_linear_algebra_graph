use crate::error::GraphComputingError;

use crate::graph::edge::{EdgeTypeIndex, EdgeTypeKeyAndIndexConversion, EdgeTypeRef};
use crate::graph::graph::Graph;

pub trait DropEdgeType {
    /// Deletes the edge type, and all its edges
    fn drop_edge_type_with_key(
        &mut self,
        edge_type: &EdgeTypeRef,
    ) -> Result<(), GraphComputingError>;
    fn drop_edge_type_with_index(
        &mut self,
        edge_type: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DropEdgeType for Graph {
    fn drop_edge_type_with_key(
        &mut self,
        edge_type: &EdgeTypeRef,
    ) -> Result<(), GraphComputingError> {
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

    fn drop_edge_type_with_index(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let edge_type = self
            .edge_type_index_to_edge_type_ref(edge_type_index.clone())?
            .to_owned();
        self.edge_type_to_edge_type_index_map_mut_ref()
            .remove(edge_type.as_str());
        self.adjacency_matrices_mut_ref()
            .free(edge_type_index.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::DirectedEdgeDefinedByKeys;
    use crate::graph::vertex::Vertex;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_edge::ReadEdge;

    #[test]
    fn drop_edge_type() {
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
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
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
            .drop_edge_type_with_key(edge_vertex1_vertex2.edge_type_ref())
            .unwrap();
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
            .unwrap());
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap());
    }
}
