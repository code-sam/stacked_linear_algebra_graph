use crate::graph::value_type::ValueType;

use super::graph::{VertexIndex, VertexTypeIndex};
use super::value_type::implement_macro_for_all_native_value_types;

pub type VertexKey = String;
pub type VertexKeyRef = str;

pub type VertexTypeKey = String;
pub type VertexTypeKeyRef = str;

pub trait VertexDefinedByKeyTrait<T: ValueType> {
    fn type_key_ref(&self) -> &VertexTypeKeyRef;
    fn key_ref(&self) -> &VertexKeyRef;
    fn value_ref(&self) -> &T;
}

pub trait VertexDefinedByIndexTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn index_ref(&self) -> &VertexIndex;
    fn value_ref(&self) -> &T;
}

pub trait VertexDefinedByTypeIndexAndVertexKeyTrait<T: ValueType> {
    fn type_index_ref(&self) -> &VertexTypeIndex;
    fn key_ref(&self) -> &VertexTypeKeyRef;
    fn value_ref(&self) -> &T;
}

pub trait VertexDefinedByTypeKeyAndVertexIndexTrait<T: ValueType> {
    fn type_key_ref(&self) -> &VertexTypeKeyRef;
    fn index_ref(&self) -> &VertexIndex;
    fn value_ref(&self) -> &T;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByKey<T: ValueType> {
    key: VertexKey,
    vertex_type: VertexTypeKey,
    value: T,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByIndex<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeIndex,
    value: T,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VertexDefinedByTypeIndexAndVertexKey<T: ValueType> {
    key: VertexKey,
    vertex_type: VertexTypeIndex,
    value: T,
}

pub struct VertexDefinedByTypeKeyAndVertexIndex<T: ValueType> {
    index: VertexIndex,
    vertex_type: VertexTypeKey,
    value: T,
}

macro_rules! implement_vertex_defined_by_key_trait {
    ($value_type:ty) => {
        impl VertexDefinedByKeyTrait<$value_type> for VertexDefinedByKey<$value_type> {
            fn type_key_ref(&self) -> &VertexTypeKeyRef {
                self.vertex_type.as_str()
            }
            fn key_ref(&self) -> &VertexKeyRef {
                self.key.as_str()
            }
            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_vertex_defined_by_key_trait);

macro_rules! implement_vertex_defined_by_index_trait {
    ($value_type:ty) => {
        impl VertexDefinedByIndexTrait<$value_type> for VertexDefinedByIndex<$value_type> {
            fn type_index_ref(&self) -> &VertexTypeIndex {
                &self.vertex_type
            }

            fn index_ref(&self) -> &VertexIndex {
                &self.index
            }

            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_vertex_defined_by_index_trait);

macro_rules! implement_vertex_defined_by_type_index_and_vertex_key_trait {
    ($value_type:ty) => {
        impl VertexDefinedByTypeIndexAndVertexKeyTrait<$value_type>
            for VertexDefinedByTypeIndexAndVertexKey<$value_type>
        {
            fn type_index_ref(&self) -> &VertexTypeIndex {
                &self.vertex_type
            }

            fn key_ref(&self) -> &VertexKeyRef {
                &self.key
            }

            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_vertex_defined_by_type_index_and_vertex_key_trait
);

macro_rules! implement_vertex_defined_by_type_key_and_vertex_index_trait {
    ($value_type:ty) => {
        impl VertexDefinedByTypeKeyAndVertexIndexTrait<$value_type>
            for VertexDefinedByTypeKeyAndVertexIndex<$value_type>
        {
            fn type_key_ref(&self) -> &VertexTypeKeyRef {
                &self.vertex_type
            }

            fn index_ref(&self) -> &VertexIndex {
                &self.index
            }

            fn value_ref(&self) -> &$value_type {
                &self.value
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_vertex_defined_by_type_key_and_vertex_index_trait
);

impl<T: ValueType + Clone> VertexDefinedByKey<T> {
    pub fn new(vertex_type: &VertexTypeKeyRef, key: &VertexKeyRef, value: &T) -> Self {
        Self {
            key: key.to_owned(),
            vertex_type: vertex_type.to_owned(),
            value: value.clone(),
        }
    }
}

impl<T: ValueType + Clone> VertexDefinedByIndex<T> {
    pub fn new(vertex_type: &VertexTypeIndex, index: &VertexIndex, value: &T) -> Self {
        Self {
            index: index.clone(),
            vertex_type: vertex_type.clone(),
            value: value.clone(),
        }
    }
}

impl<T: ValueType + Clone> VertexDefinedByTypeIndexAndVertexKey<T> {
    pub fn new(vertex_type: &VertexTypeIndex, key: &VertexKeyRef, value: &T) -> Self {
        Self {
            key: key.to_string(),
            vertex_type: vertex_type.clone(),
            value: value.clone(),
        }
    }
}

impl<T: ValueType + Clone> VertexDefinedByTypeKeyAndVertexIndex<T> {
    pub fn new(vertex_type: &VertexTypeKeyRef, index: &VertexIndex, value: &T) -> Self {
        Self {
            index: index.clone(),
            vertex_type: vertex_type.to_string(),
            value: value.clone(),
        }
    }
}

// pub trait VertexKeyAndIndexConversion {
//     fn vertex_index_to_vertex_key_ref(
//         &self,
//         vertex_index: VertexIndex,
//     ) -> Result<&VertexKeyRef, GraphComputingError>;

//     fn vertex_key_ref_to_vertex_index_ref(
//         &self,
//         key: &VertexKeyRef,
//     ) -> Result<&VertexIndex, GraphComputingError>;
// }

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
