use std::fmt::Display;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetVectorElementValueTyped, SetVectorElementTyped,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait, VertexIndex};

use crate::graph::indexer::AssignedIndexTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKey;
use crate::graph::vertex_store::AddVertex as AddVertexToStore;

pub trait AddVertex<T: ValueType> {
    fn add_new_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_new_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<VertexIndex, GraphComputingError>;

    // /// Replacement deletes connected edges
    // fn add_or_replace_vertex(
    //     &mut self,
    //     vertex: VertexDefinedByKey<T>,
    // ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,

        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

impl<T> AddVertex<T> for Graph
where
    T: ValueType
        + GetVectorElementValueTyped<T>
        + SetVectorElementTyped<T>
        + Default
        + Copy
        + Display,
    SparseMatrix<T>: Display,
{
    fn add_new_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<VertexIndex, GraphComputingError> {
        let new_index = self
            .vertex_store_mut_ref()
            .add_new_key_defined_vertex(vertex)?;
        match new_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref()
                    .resize_adjacency_matrices(new_vertex_capacity)?;
            }
            None => (),
        }
        Ok(*new_index.index_ref())
    }

    fn add_new_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<VertexIndex, GraphComputingError> {
        let new_index = self
            .vertex_store_mut_ref()
            .add_new_vertex_with_type_index_and_vertex_key(vertex)?;
        match new_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref()
                    .resize_adjacency_matrices(new_vertex_capacity)?;
            }
            None => (),
        }
        Ok(*new_index.index_ref())
    }

    // fn add_or_replace_vertex(
    //     &mut self,
    //     vertex: VertexDefinedByKey<$value_type>,
    // ) -> Result<VertexIndex, GraphComputingError> {
    //     let new_index = self
    //         .vertex_store_mut_ref()
    //         .add_or_replace_key_defined_vertex(vertex)?;
    //     match new_index.new_index_capacity() {
    //         Some(new_capacity) => {
    //             self.update_vertex_capacity(&new_capacity)?;
    //         }
    //         None => (),
    //     }
    //     Ok(*new_index.index_ref())
    // }

    fn add_or_update_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        match self
            .vertex_store_mut_ref()
            .add_or_update_key_defined_vertex(vertex)?
        {
            Some(new_index) => {
                match new_index.new_index_capacity() {
                    Some(new_vertex_capacity) => {
                        self.edge_store_mut_ref()
                            .resize_adjacency_matrices(new_vertex_capacity)?;
                    }
                    None => (),
                }
                Ok(Some(*new_index.index_ref()))
            }
            None => Ok(None),
        }
    }

    fn add_or_update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,

        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        match self
            .vertex_store_mut_ref()
            .add_or_update_vertex_with_type_index_and_vertex_key(vertex)?
        {
            Some(new_index) => {
                match new_index.new_index_capacity() {
                    Some(new_vertex_capacity) => {
                        self.edge_store_mut_ref()
                            .resize_adjacency_matrices(new_vertex_capacity)?;
                    }
                    None => (),
                }
                Ok(Some(*new_index.index_ref()))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::vertex::vertex::GetVertexValue;
    use crate::operators::add::AddVertexType;
    use crate::operators::read::ReadVertexValue;

    // #[test]
    // fn add_or_replace_vertex() {
    //     let mut graph = Graph::with_initial_capacity(&1, &5, &5).unwrap();
    //     let vertex_type_key = String::from("A type");
    //     let vertex_key = String::from("A key");
    //     let vertex_property = 1u8;

    //     let vertex_to_add = VertexDefinedByKey::new(
    //         vertex_type_key.as_str(),
    //         vertex_key.as_str(),
    //         &vertex_property,
    //     );
    //     let vertex_type_index = graph.add_new_vertex_type(vertex_type_key.as_str()).unwrap();
    //     let index1 = graph
    //         .add_or_update_vertex(vertex_to_add.clone())
    //         .unwrap()
    //         .unwrap();

    //     let vertex_property_2 = 2u8;
    //     let vertex_to_add_2 = VertexDefinedByKey::new(
    //         vertex_type_key.as_str(),
    //         vertex_key.as_str(),
    //         &vertex_property_2,
    //     );

    //     let index2 = graph
    //         .add_or_update_vertex(vertex_to_add_2)
    //         .unwrap()
    //         .unwrap();

    //     assert_ne!(index1, index2);
    //     let value_1: u8 = graph
    //         .try_vertex_value_by_key(vertex_type_key.as_str(), vertex_key.as_str())
    //         .unwrap();
    //     assert_eq!(value_1, vertex_property_2);
    //     let value_2: u8 = graph
    //         .try_vertex_value_by_index(&vertex_type_index, &index2)
    //         .unwrap();
    //     assert_eq!(value_2, vertex_property_2);
    //     assert!(!graph.is_valid_vertex_index(&index1).unwrap());
    // }

    #[test]
    fn add_or_update_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &5, &5).unwrap();
        let vertex_type_key = String::from("VertexType");
        let vertex_key = String::from("A key");
        let vertex_property = 1u8;

        let vertex_to_add = VertexDefinedByKey::new(
            vertex_type_key.as_str(),
            vertex_key.as_str(),
            &vertex_property,
        );
        let vertex_type_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_type_key.as_str()).unwrap();
        let index1 = graph
            .add_or_update_key_defined_vertex(vertex_to_add.clone())
            .unwrap()
            .unwrap();

        let vertex_property_2 = 2u8;
        let vertex_to_add_2 = VertexDefinedByKey::new(
            vertex_type_key.as_str(),
            vertex_key.as_str(),
            &vertex_property_2,
        );

        let index2 = graph
            .add_or_update_key_defined_vertex(vertex_to_add_2)
            .unwrap();

        assert_eq!(None, index2);
        let value_2: u8 = graph
            .try_vertex_value_by_key(vertex_type_key.as_str(), &vertex_key)
            .unwrap();
        assert_eq!(value_2, vertex_property_2);
        let value_2: u8 = graph
            .try_vertex_value_by_index(&vertex_type_index, &index1)
            .unwrap();
        assert_eq!(value_2, vertex_property_2);
    }

    #[test]
    fn add_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &5, &5).unwrap();
        let vertex_type_key = String::from("A type");
        let vertex_key = String::from("A key");
        let vertex_property = 1u8;

        let another_key = String::from("Another key");
        let another_vertex_property = 2u8;

        let vertex_to_add = VertexDefinedByKey::new(
            vertex_type_key.as_str(),
            vertex_key.as_str(),
            &vertex_property,
        );
        let vertex_type_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_type_key.as_str()).unwrap();
        graph
            .add_new_key_defined_vertex(vertex_to_add.clone())
            .unwrap();

        let value: u8 = graph
            .try_vertex_value_by_key(vertex_type_key.as_str(), vertex_key.as_str())
            .unwrap();
        assert_eq!(value, vertex_to_add.value_ref().clone());

        let another_vertex_to_add = VertexDefinedByKey::new(
            vertex_type_key.as_str(),
            another_key.as_str(),
            &another_vertex_property,
        );
        let index = graph
            .add_new_key_defined_vertex(another_vertex_to_add.clone())
            .unwrap();

        let value: u8 = graph
            .try_vertex_value_by_index(&vertex_type_index, &index)
            .unwrap();
        assert_eq!(value, another_vertex_to_add.clone().value_ref().clone());

        match graph.add_new_key_defined_vertex(another_vertex_to_add.clone()) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn add_new_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &1, &1).unwrap();

        for i in 0..3 {
            AddVertexType::<u8>::add_new_vertex_type(
                &mut graph,
                format!("vertex_type_{}", i).as_str(),
            )
            .unwrap();
        }

        for i in 0..50 {
            graph
                .add_new_key_defined_vertex(VertexDefinedByKey::new(
                    "vertex_type_2",
                    format!("vertex_{}", i).as_str(),
                    &i,
                ))
                .unwrap();
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
