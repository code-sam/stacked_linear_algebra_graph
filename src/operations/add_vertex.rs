use crate::error::{GraphComputingError, UserError, UserErrorType};

use crate::graph::graph::{Graph, GraphTrait, VertexIndex};
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, NativeDataType};
use crate::graph::vertex::{Vertex, VertexTrait};
use crate::graph::vertex_store::operations::AddVertex as AddVertexToStore;
use crate::graph::vertex_store::operations::Indexing;

// use super::update_vertex::UpdateVertex;

// use super::update_vertex::UpdateVertex;

pub trait AddVertex<T: ValueType> {
    fn add_new_vertex(&mut self, vertex: Vertex<T>) -> Result<VertexIndex, GraphComputingError>;

    /// Replacement deletes connected edges
    fn add_or_replace_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

// impl<T: ValueType> AddVertex<T> for Graph<T> {
// fn add_new_vertex(
//     &mut self,
//     vertex_to_add: Vertex<T>,
// ) -> Result<VertexIndex, GraphComputingError> {
//     let key_ref_of_vertex_to_add = vertex_to_add.key_ref();
//     if !self.is_vertex_key(key_ref_of_vertex_to_add)
//     {
//         // GraphData::add_or_replace_vertex(&self, vertex_to_add)
//         self.add_or_replace_vertex(vertex_to_add)
//     } else {
//         Err(UserError::new(
//             UserErrorType::VertexAlreadyExists,
//             format!(
//                 "A vertex with key '{}' already exists",
//                 key_ref_of_vertex_to_add
//             ),
//             None,
//         )
//         .into())
//     }
// }

/// Replacement deletes connected edges
// fn add_or_replace_vertex(
//     &mut self,
//     new_vertex: Vertex<T>,
// ) -> Result<VertexIndex, GraphComputingError> {
//     self.add_or_replace_vertex(new_vertex)
// }

// fn add_or_update_vertex(
//     &mut self,
//     vertex: Vertex<T>,
// ) -> Result<Option<VertexIndex>, GraphComputingError> {
//     match self.vertex_index(vertex.key_ref()) {
//         Some(&vertex_index) => {
//             self.update_vertex_value_by_index(&vertex_index, vertex.value_ref().clone())?;
//             Ok(None)
//         }
//         // TODO: does add_or_replace_vertex() perform redundant checks and/or work?
//         None => Ok(Some(self.add_or_replace_vertex(vertex)?)),
//     }
// }
// }

macro_rules! implement_add_vertex {
    ($value_type:ty) => {
        impl AddVertex<$value_type> for Graph<$value_type> {
            fn add_new_vertex(
                &mut self,
                vertex: Vertex<$value_type>,
            ) -> Result<VertexIndex, GraphComputingError> {
                self.vertex_store_mut_ref().add_new_vertex(vertex)
            }

            fn add_or_replace_vertex(
                &mut self,
                vertex: Vertex<$value_type>,
            ) -> Result<VertexIndex, GraphComputingError> {
                self.vertex_store_mut_ref().add_or_replace_vertex(vertex)
            }

            fn add_or_update_vertex(
                &mut self,
                vertex: Vertex<$value_type>,
            ) -> Result<Option<VertexIndex>, GraphComputingError> {
                self.vertex_store_mut_ref().add_or_update_vertex(vertex)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_add_vertex);

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::{
        collections::sparse_vector::SparseVector,
        context::{Context as GraphblasContext, Mode as GraphblasMode},
        index::ElementIndex as GraphblasElementIndex,
    };

    use crate::operations::{add_vertex::AddVertex, Indexing, ReadVertexValue};
    // use crate::operations::read_vertex_value::ReadVertexValue;
    // use crate::operations::select_edge_type::EdgeTypeSelectorTrait;
    // use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn add_or_replace_vertex() {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut graph = Graph::<u8>::with_initial_capacity(graphblas_context, 5, 5).unwrap();
        let vertex_key = String::from("A key");
        let vertex_property = 1u8;

        let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
        let index1 = graph.add_or_replace_vertex(vertex_to_add.clone()).unwrap();

        let vertex_property_2 = 2u8;
        let vertex_to_add_2 = Vertex::new(vertex_key.clone(), vertex_property_2.clone().into());

        let index2 = graph.add_or_replace_vertex(vertex_to_add_2).unwrap();

        assert_ne!(index1, index2);
        assert_eq!(
            graph.vertex_value_by_key(&vertex_key).unwrap(),
            vertex_property_2
        );
        assert_eq!(
            graph.vertex_value_by_index(index2).unwrap(),
            vertex_property_2
        );
        assert!(!graph.is_valid_vertex_index(&index1).unwrap());
    }

    #[test]
    fn add_or_update_vertex() {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut graph = Graph::<u8>::with_initial_capacity(graphblas_context, 5, 5).unwrap();
        let vertex_key = String::from("A key");
        let vertex_property = 1u8;

        let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
        let index1 = graph.add_or_update_vertex(vertex_to_add.clone()).unwrap();

        let vertex_property_2 = 2u8;
        let vertex_to_add_2 = Vertex::new(vertex_key.clone(), vertex_property_2.clone().into());

        let index2 = graph.add_or_update_vertex(vertex_to_add_2).unwrap();

        assert_eq!(None, index2);
        assert_eq!(
            graph.vertex_value_by_key(&vertex_key).unwrap(),
            vertex_property_2
        );
        assert_eq!(
            graph.vertex_value_by_index(index1.unwrap()).unwrap(),
            vertex_property_2
        );
    }

    #[test]
    fn add_vertex() {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut graph = Graph::<u8>::with_initial_capacity(graphblas_context, 5, 5).unwrap();
        let vertex_key = String::from("A key");
        let vertex_property = 1u8;

        let another_key = String::from("Another key");
        let another_vertex_property = 2u8;

        let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
        graph.add_new_vertex(vertex_to_add.clone()).unwrap();

        assert_eq!(
            graph.vertex_value_by_key(&vertex_key).unwrap(),
            vertex_to_add.value_ref().clone()
        );

        let another_vertex_to_add =
            Vertex::new(another_key.clone(), another_vertex_property.clone().into());
        let index = graph.add_new_vertex(another_vertex_to_add.clone()).unwrap();

        assert_eq!(
            graph.vertex_value_by_index(index).unwrap(),
            another_vertex_to_add.clone().value_ref().clone()
        );

        match graph.add_new_vertex(another_vertex_to_add.clone()) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }

    // #[test]
    // fn add_or_replace_vertex() {
    //     let mut graph = standard_graph_for_testing();

    //     let vertex = Vertex::new(String::from("1").into(), String::from("replaced").into());
    //     graph.add_or_replace_vertex(vertex.clone()).unwrap();

    //     let edge_selection = graph.select_edge_type(String::from("is_a").into()).unwrap();
    //     let from_vertices = edge_selection.get_from_vertices().unwrap();
    //     assert!(!from_vertices.contains(&vertex));
    // }
}
