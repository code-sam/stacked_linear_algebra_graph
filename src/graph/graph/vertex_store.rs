use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::value_type::NativeDataType as GraphNativeDataType;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::{VertexKeyRef, Vertex, VertexTrait};

use super::indexer::{Indexer, IndexerTrait};

#[derive(Clone, Debug)]
pub(crate) struct VertexStore<T: ValueType> {
    // TODO: should vertices be kept as a SparseVector or diagonal SparseMatrices? What's more efficient?
    // Using diagonal matrices may bring advantages for combined processing with edge data.
    // The underlying GraphBLAS implementation must however be optimized for diagional matrices,
    // especially in terms of access speed. TODO: bench access speed to diagonal matrices.
    vertices: SparseMatrix<T>,
    indexer: Indexer
}

// pub(crate) trait VertexStoreTrait<T: ValueType> {
//     fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;

//     fn vertices_ref(&self) -> &SparseMatrix<T>;
//     fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<T>;
// }

impl<T: ValueType> VertexStore<T> {
    pub(crate) fn with_initial_capacity(
        context: &Arc<Context>,
        inital_vertex_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let size = Size::new(
            inital_vertex_capacity.clone(),
            inital_vertex_capacity.clone(),
        );
        Ok(Self {
            vertices: SparseMatrix::new(context, &size)?,
            indexer: Indexer::with_initial_capacity(context, inital_vertex_capacity)?
        })
    }
}

pub(crate) trait SetCapacity {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
}

pub(crate) trait SparseVertexMatrix<T: ValueType> {
    fn vertices_ref(&self) -> &SparseMatrix<T>;
    fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<T>;
}

pub(crate) trait SetVertexData<T: ValueType> {
    fn add_new_vertex(&mut self, vertex: Vertex<T>) -> Result<Index, GraphComputingError>;
    fn add_or_replace_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<Index, GraphComputingError>;
    fn add_or_update_vertex(
        &mut self,
        vertex: Vertex<T>,
    ) -> Result<Option<Index>, GraphComputingError>;

    fn update_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError>;
}

pub(crate) trait Indexing {
    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError>;
    fn is_valid_key(&self, key: &VertexKeyRef) -> bool;
//     fn indexer_ref(&self) -> &Indexer;
//     fn indexer_mut_ref(&mut self) -> &mut Indexer;
}

impl<T: ValueType> SetCapacity for VertexStore<T> {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
        let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
        self.vertices.resize(&target_size)?;
        Ok(())
    }
}

impl<T: ValueType> SparseVertexMatrix<T> for VertexStore<T> {
    fn vertices_ref(&self) -> &SparseMatrix<T> {
        &self.vertices
    }
    fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<T> {
        &mut self.vertices
    }
}

// impl<T: MatrixDataType> SetVertexData<T> for VertexStore<T> {
//     fn set_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError> {
//         self.vertices.set_element((index, index, value).into())?;
//         Ok(())
//     }
// }

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl SetVertexData<$value_type> for VertexStore<$value_type> {
            fn add_new_vertex(&mut self, vertex: Vertex<$value_type>) -> Result<Index, GraphComputingError> {
                let index = self.indexer.add_new_key(vertex.key_ref())?;
                self.vertices.set_element((index, index, vertex.value_ref().clone()).into())?;
                Ok(index)
            }

            fn add_or_replace_vertex(
                &mut self,
                vertex: Vertex<$value_type>,
            ) -> Result<Index, GraphComputingError> {
                let index = self.indexer.add_or_replace_key(vertex.key_ref())?;
                self.vertices.set_element((index, index, vertex.value_ref().clone()).into())?;
                Ok(index)
            }

            fn add_or_update_vertex(
                &mut self,
                vertex: Vertex<$value_type>,
            ) -> Result<Option<Index>, GraphComputingError> {
                match self.indexer.index_for_key(vertex.key_ref()) {
                    Some(index_ref) => {
                        let index = index_ref.clone();
                        self.vertices.set_element((index, index, vertex.value_ref().clone()).into())?;
                        Ok(Some(index))
                    },
                    None => {
                        // REVIEW: can this arm be made faster with the knowledge that the vertex is new?
                        Ok(Some(self.add_new_vertex(vertex)?))
                    }
                }
            }

            fn update_vertex_value(
                &mut self,
                index: Index,
                value: $value_type,
            ) -> Result<(), GraphComputingError> {
                self.indexer.try_index_validity(&index)?;
                self.vertices.set_element((index, index, value).into())?;
                Ok(())
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_set_vertex_data);

impl<T: ValueType> Indexing for VertexStore<T> {
    fn is_valid_index(&self, index: &Index) -> Result<bool, GraphComputingError> {
        self.indexer.is_valid_index(index)
    }

    fn is_valid_key(&self, key: &VertexKeyRef) -> bool {
        self.indexer.is_valid_key(key)
    }
//     fn indexer_ref(&self) -> &Indexer {
//         &self.indexer
//     }

//     fn indexer_mut_ref(&mut self) -> &mut Indexer {
//         self
//     }
}

// macro_rules! set_sparse_matrix_capacity {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size)?;
//     };
// }

// impl SetCapacity for VertexStore {
//     fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_graph_indentifier_for_all_matrix_data_types!(set_sparse_matrix_capacity, vertices);
//         Ok(())
//     }
// }

// macro_rules! implement_set_capacity {
//     ($dummy:literal, $($y:ident),+) => {
//         impl SetCapacity for VertexStore {
//             fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 implement_set_capacity!($($y),+);
//     };
//     ($type_id:ident, $($y:ident),*) => {
//         paste::paste! {
//             self.[<vertices $type_id>].resize(&target_size)?;
//         }
//         implement_set_capacity!($($y),*);
//     };
//     ($type_id:ident) => {
//                 paste::paste! {
//                     self.[<vertices $type_id>].resize(&target_size)?;
//                 }
//                 Ok(())
//             }
//         }
//     }
// }

// macro_rules! implement_macro_for_all_graph_data_typed_parameter {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(
//             0,
//             _bool,
//             _i8,
//             _i16,
//             _i32,
//             _i64,
//             _u8,
//             _u16,
//             _u32,
//             _u64,
//             _isize,
//             _usize,
//             _char,
//             _unit
//         );
//     };
// }

// implement_macro_for_all_graph_data_typed_parameter!(implement_set_capacity);

// pub(crate) trait SetCapacityTyped<G: GraphNativeDataType, M: MatrixDataType> {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! implement_set_capacity_typed {
//     ($vertices_typed:ident, $graph_data_type:ty, $matrix_data_type:ty) => {
//         impl SetCapacityTyped<$graph_data_type, $matrix_data_type> for VertexStore {
//             fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 Ok(self.$vertices_typed.resize(&target_size)?)
//             }
//         }
//     };
// }

// impl SetCapacityTyped<bool> for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         Ok(self.vertices_bool.resize(&target_size)?)
//     }
// }

// implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types!(implement_set_capacity_typed, vertices);

// pub(crate) trait VertexData {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! resize_vertices {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size);
//     };
// }

// impl VertexData for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_indentifier_for_all_native_data_types!(resize_vertices, vertices);
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::{
        collections::sparse_vector::SparseVector,
        context::{Context as GraphblasContext, Mode as GraphblasMode},
        index::ElementIndex as GraphblasElementIndex,
    };
    
    #[test]
    fn add_new_vertex() {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut store = VertexStore::<u8>::with_initial_capacity(&graphblas_context, &10).unwrap();
        
        let vertex_1 = Vertex::new(String::from("key"), 1u8);

        let index_1 = store.add_new_vertex(vertex_1.clone()).unwrap();
        assert!(store.is_valid_index(&index_1).unwrap());
        assert!(store.is_valid_key(vertex_1.key_ref()));
    }
}
