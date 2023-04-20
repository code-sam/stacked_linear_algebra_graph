use crate::error::{GraphComputingError, SystemError, SystemErrorType, UserError, UserErrorType};

use super::add_vertex::AddVertex;
use crate::graph::graph::graph::Graph;
use crate::graph::data_type::NativeDataType;
use crate::graph::vertex::{Vertex, VertexIndex};

pub trait UpdateVertex<T: NativeDataType> {
    fn update_or_add_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError>; // REVIEW update vs set
    fn update_vertex_value(&mut self, vertex_to_update: Vertex<T>) -> Result<(), GraphComputingError>;
    fn update_vertex_value_by_index(
        &mut self,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T: NativeDataType> UpdateVertex<T> for Graph {
    fn update_or_add_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        // TODO: use AddVertex trait implementation
        let vertex_index = self
            .vertex_key_to_vertex_index_map_mut_ref()
            .get(vertex.key_ref());
        match vertex_index {
            Some(&vertex_index) => {
                self.vertex_store_mut_ref()
                    .update(vertex_index, vertex)?;
                Ok(None)
            }
            None => Ok(Some(self.add_or_replace_vertex(vertex)?)),
        }
    }

    // TODO: is there a use-case for returning the VertexIndex?
    fn update_vertex_value(&mut self, vertex_to_update: Vertex<T>) -> Result<(), GraphComputingError> {
        let vertex_index = self
            .vertex_key_to_vertex_index_map_mut_ref()
            .get(vertex_to_update.key_ref());
        match vertex_index {
            Some(&vertex_index) => {
                self.vertex_store_mut_ref()
                    .update(vertex_index, vertex_to_update)?;
                Ok(())
            }
            None => Err(UserError::new(
                UserErrorType::VertexKeyNotFound,
                format!(
                    "No vertex with key {} found to update",
                    vertex_to_update.key_ref()
                ),
                None,
            )
            .into()),
        }
    }

    fn update_vertex_value_by_index(
        &mut self,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_to_update = self.vertex_store_mut_ref().get_mut_ref(vertex_index);
        
        match vertex_to_update {
            Ok(vertex) => vertex.update_value(vertex_value),
            Err(_) => {
                // TODO: technically, another system error could have occured
                return Err(SystemError::new(
                    SystemErrorType::IndexOutOfBounds,
                    format!("No vertex at selected index"),
                    None,
                )
                .into());
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::operations::read_vertex_value::ReadVertexValue;

    // #[test]
    // fn update_vertex() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_replace_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph.update_vertex(another_vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }

    // #[test]
    // fn add_or_update_vertex() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_update_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph
    //         .add_or_update_vertex(another_vertex_to_add.clone())
    //         .unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }

    // #[test]
    // fn update_vertex_value_by_index() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_update_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph
    //         .add_or_update_vertex(another_vertex_to_add.clone())
    //         .unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }
}
