use crate::error::{GraphComputingError, UserError, UserErrorType};

use crate::graph::graph::Graph;
use crate::graph::vertex::{Vertex, VertexIndex};

use super::update_vertex::UpdateVertex;

pub trait AddVertex {
    fn add_new_vertex(&mut self, vertex: Vertex) -> Result<VertexIndex, GraphComputingError>;
    fn add_or_replace_vertex(&mut self, vertex: Vertex)
        -> Result<VertexIndex, GraphComputingError>;
    fn add_or_update_vertex(
        &mut self,
        vertex: Vertex,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

impl AddVertex for Graph {
    // TODO: use try_insert() once it is stable
    fn add_new_vertex(
        &mut self,
        vertex_to_add: Vertex,
    ) -> Result<VertexIndex, GraphComputingError> {
        let key_ref_of_vertex_to_add = vertex_to_add.key_ref();
        if !self
            .vertex_key_to_vertex_index_map_ref()
            .contains_key(key_ref_of_vertex_to_add)
        {
            self.add_or_replace_vertex(vertex_to_add)
        } else {
            Err(UserError::new(
                UserErrorType::VertexAlreadyExists,
                format!(
                    "A vertex with key '{}' already exists",
                    key_ref_of_vertex_to_add
                ),
                None,
            )
            .into())
        }
    }

    /// Replacement deletes connected edges
    fn add_or_replace_vertex(
        &mut self,
        new_vertex: Vertex,
    ) -> Result<VertexIndex, GraphComputingError> {
        let key_ref_of_new_vertex = new_vertex.key_ref();

        let vertex_index: VertexIndex =
            self.vertex_store_mut_ref().push(new_vertex.clone())?.into();

        self.vertex_key_to_vertex_index_map_mut_ref()
            .insert(key_ref_of_new_vertex.to_owned(), vertex_index.clone());

        self.expand_adjacency_matrices_to_match_vertex_capacity()?;
        Ok(vertex_index)
    }

    fn add_or_update_vertex(
        &mut self,
        vertex: Vertex,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        self.update_or_add_vertex(vertex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_vertex_value::ReadVertexValue;
    use crate::operations::select_edge_type::EdgeTypeSelectorTrait;
    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn add_vertex() {
        let mut graph = Graph::new(5, 5).unwrap();
        let vertex_key = String::from("A key");
        let vertex_property = String::from("A property");

        let another_key = String::from("Another key");
        let another_vertex_property = String::from("Another property");

        let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
        graph.add_new_vertex(vertex_to_add.clone()).unwrap();

        assert_eq!(
            *graph.vertex_value(&vertex_key).unwrap(),
            vertex_to_add.value()
        );

        let another_vertex_to_add =
            Vertex::new(another_key.clone(), another_vertex_property.clone().into());
        graph.add_new_vertex(another_vertex_to_add.clone()).unwrap();

        assert_eq!(
            *graph.vertex_value(&another_key).unwrap(),
            another_vertex_to_add.clone().value()
        );

        match graph.add_new_vertex(another_vertex_to_add.clone()) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn add_or_replace_vertex() {
        let mut graph = standard_graph_for_testing();

        let vertex = Vertex::new(String::from("1").into(), String::from("replaced").into());
        graph.add_or_replace_vertex(vertex.clone()).unwrap();

        let edge_selection = graph.select_edge_type(String::from("is_a").into()).unwrap();
        let from_vertices = edge_selection.get_from_vertices().unwrap();
        assert!(!from_vertices.contains(&vertex));
    }
}
