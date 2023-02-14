use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::error::{SystemError, SystemErrorType};

use crate::graph::graph::graph::Graph;
use crate::graph::index::ElementIndex;

use crate::graph::value_type::ValueType;

pub type VertexKey = String;
pub type VertexKeyRef = str;

// Use a struct instead of a type to discourage using and/or generating indices that are not coming from the pblic API.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexIndex {
    index: ElementIndex,
}

impl VertexIndex {
    pub(crate) fn new(index: ElementIndex) -> Self {
        VertexIndex { index }
    }
    pub(crate) fn index(self) -> ElementIndex {
        self.index
    }
    pub(crate) fn index_ref(&self) -> &ElementIndex {
        &self.index
    }
}

// TODO: Implementation leaks VertexIndex instantiation out of pub(crate) scope
// impl From<ElementIndex> for VertexIndex {
//     fn from(index: ElementIndex) -> Self {
//         VertexIndex::new(index)
//     }
// }
// impl From<VertexIndex> for ElementIndex {
//     fn from(index: VertexIndex) -> Self {
//         index.index()
//     }
// }

pub trait VertexTrait<T: ValueType> {
    fn new(key: VertexKey, value: T) -> Self;
    // pub fn key(&self) -> &VertexKey;
    fn key_ref(&self) -> &VertexKeyRef;
    fn value_ref(&self) -> &T;
    // pub fn value(self) -> T;
    fn update_value(&mut self, new_value: T);
    fn update_key(&mut self, new_key: VertexKey);
}

// TODO: implementation implies Vertices from different graphs can be equal.
// TODO: The implementation defines a Vertex coordindate defined by a key.
// Whereas the coordinate can be a key, or an index. Is this struct a
// consistent definition of a Vertex?
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vertex<T: ValueType> {
    key: VertexKey,
    value: T,
}

impl<T: ValueType> From<Vertex<T>> for VertexKey {
    fn from(vertex: Vertex<T>) -> Self {
        vertex.key_ref().to_owned()
    }
}

// / ```
// / # use cairn_knowledge_graph::graph::vertex::Vertex;
// / let vertex: Vertex = (String::from("Vertex key"), 1u8).into();
// / assert_eq!(vertex.key_ref().to_owned(), String::from("Vertex key"));
// / assert_eq!(vertex.value(), 1u8);
// / ```
impl<T: ValueType> From<(VertexKey, T)> for Vertex<T> {
    fn from(as_tuple: (VertexKey, T)) -> Self {
        Vertex::new(as_tuple.0, as_tuple.1)
    }
}

impl<T: ValueType> VertexTrait<T> for Vertex<T> {
    fn new(key: VertexKey, value: T) -> Self {
        Self { key, value }
    }
    // pub fn key(&self) -> &VertexKey {
    //     &self.key
    // }
    fn key_ref(&self) -> &VertexKeyRef {
        &self.key
    }
    fn value_ref(&self) -> &T {
        &self.value
    }
    // pub fn value(self) -> T {
    //     self.value
    // }

    fn update_value(&mut self, new_value: T) {
        self.value = new_value;
    }
    fn update_key(&mut self, new_key: VertexKey) {
        self.key = new_key;
    }

    // TO REVIEW: converting Vertex to an enum would make the vertex immutable, but introduce runtime cost
    // It should not be possible to access/reach a deleted vertex
    // pub(crate) fn mark_as_removed(&mut self) {
    //     self.key = String::from("_deleted");
    //     self.value = VertexValue::None;
    // }
}

pub trait VertexKeyAndIndexConversion {
    fn vertex_index_to_vertex_key_ref(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<&VertexKeyRef, GraphComputingError>;

    fn vertex_key_ref_to_vertex_index_ref(
        &self,
        key: &VertexKeyRef,
    ) -> Result<&VertexIndex, GraphComputingError>;
}

// impl VertexKeyAndIndexConversion for Graph {
//     fn vertex_index_to_vertex_key_ref(
//         &self,
//         vertex_index: VertexIndex,
//     ) -> Result<&VertexKeyRef, GraphComputingError> {
//         match self.vertex_store_ref().get_ref(vertex_index) {
//             Ok(vertex) => return Ok(vertex.key_ref()),
//             Err(_) => {
//                 // TODO:match actual error type
//                 return Err(LogicError::new(
//                     LogicErrorType::VertexMustExist,
//                     format!("There is no vertex at index [{}]", vertex_index.index()),
//                     None,
//                 )
//                 .into());
//             }
//         }
//     }

//     fn vertex_key_ref_to_vertex_index_ref(
//         &self,
//         key: &VertexKeyRef,
//     ) -> Result<&VertexIndex, GraphComputingError> {
//         match self.vertex_key_to_vertex_index_map_ref().get(key) {
//             None => Err(SystemError::new(
//                 SystemErrorType::KeyNotFound,
//                 format!("Could not map vertex key '{}' to a vertex index", key),
//                 None,
//             )
//             .into()),
//             Some(vertex_index) => Ok(vertex_index),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // use crate::operations::add_vertex::AddVertex;

    // #[test]
    // fn test_convert_vertex_index_to_vertex_key_ref() {
    //     let mut graph = Graph::new(10, 20).unwrap();

    //     let vertex_key_1 = String::from("Vertex_1");
    //     let vertex_value_1 = String::from("Property_1");
    //     let vertex_1 = Vertex::new(vertex_key_1.clone(), vertex_value_1.into());
    //     graph.add_or_replace_vertex(vertex_1).unwrap();

    //     let vertex_key_2 = String::from("Vertex_2");
    //     let vertex_value_2 = String::from("Property_2");
    //     let vertex_2 = Vertex::new(vertex_key_2.clone(), vertex_value_2.into());
    //     graph.add_or_replace_vertex(vertex_2).unwrap();

    //     let index_vertex_1 = graph
    //         .vertex_key_to_vertex_index_map_ref()
    //         .get(vertex_key_1.as_str())
    //         .unwrap();
    //     assert_eq!(
    //         graph
    //             .vertex_index_to_vertex_key_ref(index_vertex_1.clone())
    //             .unwrap(),
    //         vertex_key_1
    //     );

    //     let index_vertex_2 = graph
    //         .vertex_key_to_vertex_index_map_ref()
    //         .get(vertex_key_2.as_str())
    //         .unwrap();
    //     assert_eq!(
    //         graph
    //             .vertex_index_to_vertex_key_ref(index_vertex_2.clone())
    //             .unwrap(),
    //         vertex_key_2
    //     );
    // }
}
