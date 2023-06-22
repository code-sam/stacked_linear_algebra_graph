use crate::graph::value_type::ValueType;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait},
        value_type::implement_macro_for_all_native_value_types,
        vertex::{VertexDefinedByIndex, VertexDefinedByKey, VertexDefinedByTypeIndexAndVertexKey},
        vertex_store::vertex_operations::UpdateVertex,
    },
};

// REVIEW update vs set
pub trait UpdateVertexValue<T: ValueType> {
    fn update_vertex_value_by_key(
        &mut self,
        vertex: &VertexDefinedByKey<T>,
    ) -> Result<(), GraphComputingError>;
    fn update_vertex_value_by_index(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;
    fn update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: &VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<(), GraphComputingError>;
    // fn update_vertex_defined_by_type_key_and_vertex_index(
    //     &mut self,
    //     vertex: &VertexDefinedByTypeKeyAndVertexIndex<T>
    // ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_update_vertex_value {
    ($value_type:ty) => {
        impl UpdateVertexValue<$value_type> for Graph {
            fn update_vertex_value_by_key(
                &mut self,
                vertex: &VertexDefinedByKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.vertex_store_mut_ref()
                    .update_key_defined_vertex(vertex)
            }

            fn update_vertex_value_by_index(
                &mut self,
                vertex: &VertexDefinedByIndex<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.vertex_store_mut_ref()
                    .update_index_defined_vertex(vertex)
            }

            fn update_vertex_defined_by_type_index_and_vertex_key(
                &mut self,
                vertex: &VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.vertex_store_mut_ref()
                    .update_vertex_defined_by_type_index_and_vertex_key(vertex)
            }

            // fn update_vertex_defined_by_type_key_and_vertex_index(
            //     &mut self,
            //     vertex: &VertexDefinedByTypeKeyAndVertexIndex<$value_type>
            // ) -> Result<(), GraphComputingError> {
            //     self.vertex_store_mut_ref().update_vertex_defined_by_type_key_and_vertex_index(vertex)
            // }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_update_vertex_value);

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
