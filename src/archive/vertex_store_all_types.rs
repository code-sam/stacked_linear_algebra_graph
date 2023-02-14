use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{SparseMatrix, SetMatrixElement, MatrixElement, Size, SparseMatrixTrait};
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::data_type::{ConvertScalarToMatrixType, implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types, implement_macro_with_typed_graph_indentifier_for_all_matrix_data_types};
use crate::graph::data_type::NativeDataType as GraphNativeDataType;
use crate::graph::data_type::MatrixDataType;
use crate::graph::data_type::implement_macro_for_all_native_and_matrix_data_types;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;

#[derive(Clone, Debug)]
pub(crate) struct VertexMatrix<T: MatrixDataType> {
    matrix: SparseMatrix<T>
}

impl<T: MatrixDataType> VertexMatrix<T> {
    pub(crate) fn new(context: &Arc<Context>, size: &Size) -> Result<Self, GraphComputingError> {
        Ok(Self {
            matrix: SparseMatrix::<T>::new(context, size)?
        })
    }

    pub(crate) fn sparse_matrix_ref(&self) -> &SparseMatrix<T> {
        &self.matrix
    }
    pub(crate) fn sparse_matrix_mut_ref(&mut self) -> &mut SparseMatrix<T> {
        &mut self.matrix
    }
}

#[derive(Clone, Debug)]
pub(crate) enum VertexMatrixEnum {
    Bool(SparseMatrix<bool>),
    I8(SparseMatrix<i8>)
}

#[derive(Clone, Debug)]
pub(crate) struct VertexStore<T: MatrixDataType> {
    // TODO: should vertices be kept as a SparseVector or diagonal SparseMatrices? What's more efficient?
    // Using diagonal matrices may bring advantages for combined processing with edge data.
    // The underlying GraphBLAS implementation must however be optimized for diagional matrices,
    // especially in terms of access speed. TODO: bench access speed to diagonal matrices.
    vertices_bool: SparseMatrix<T>,
    vertices_i8: SparseMatrix<T>,
    vertices_i16: SparseMatrix<T>,
    vertices_i32: SparseMatrix<T>,
    vertices_i64: SparseMatrix<T>,
    vertices_u8: SparseMatrix<T>,
    vertices_u16: SparseMatrix<T>,
    vertices_u32: SparseMatrix<T>,
    vertices_u64: SparseMatrix<T>,
    vertices_f32: SparseMatrix<T>,
    vertices_f64: SparseMatrix<T>,
    vertices_isize: SparseMatrix<T>,
    vertices_usize: SparseMatrix<T>,
    vertices_char: SparseMatrix<T>, // REVIEW, note that some operations on the u32 may result in invalid chars. Though that's the same for other data types as well
    vertices_unit: SparseMatrix<T>, // REVIEW

    // vertices_enum: [VertexMatrixEnum; 2],
    // vertices_tuple: (SparseMatrix<bool>, SparseMatrix<i8>)
}

impl<T: MatrixDataType> VertexStore<T> {
    pub(crate) fn with_initial_capacity(context: &Arc<Context>, inital_vertex_capacity: &ElementCount) -> Result<Self, GraphComputingError> {
        let size = Size::new(inital_vertex_capacity.clone(), inital_vertex_capacity.clone());
        Ok(Self {
            vertices_bool: SparseMatrix::new(context, &size)?,
            vertices_i8: SparseMatrix::new(context, &size)?,
            vertices_i16: SparseMatrix::new(context, &size)?,
            vertices_i32: SparseMatrix::new(context, &size)?,
            vertices_i64: SparseMatrix::new(context, &size)?,
            vertices_u8: SparseMatrix::new(context, &size)?,
            vertices_u16: SparseMatrix::new(context, &size)?,
            vertices_u32: SparseMatrix::new(context, &size)?,
            vertices_u64: SparseMatrix::new(context, &size)?,
            vertices_f32: SparseMatrix::new(context, &size)?,
            vertices_f64: SparseMatrix::new(context, &size)?,
            vertices_isize: SparseMatrix::new(context, &size)?,
            vertices_usize: SparseMatrix::new(context, &size)?,
            vertices_char: SparseMatrix::new(context, &size)?,
            vertices_unit: SparseMatrix::new(context, &size)?,

            // vertices_enum: [
            //     VertexMatrixEnum::Bool(SparseMatrix::new(context, &size)?),
            //     VertexMatrixEnum::I8(SparseMatrix::new(context, &size)?),
            // ],
            // vertices_tuple: (
            //     SparseMatrix::new(context, &size)?,
            //     SparseMatrix::new(context, &size)?,
            // ),
        })
    }

    // pub(crate) fn all_vertex_matrices(&self) -> 

    // Experiment for communicating the typing problem
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4bd89dbfc3e937eadb7b4f76efb51906

    // pub(crate) fn map_vertex_matrices_mut<T: MatrixDataType>(&mut self, function_to_apply: fn(&mut SparseMatrix<T>) -> Result<(), GraphComputingError>) -> Result<(), GraphComputingError> {
    //     function_to_apply(self.vertices_bool.sparse_matrix_mut_ref())?;
    //     Ok(())
    // }
}

pub(crate) trait SetCapacity {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
}

impl<T: MatrixDataType> SetCapacity for VertexStore<T> {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
        let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
        self.vertices_bool.resize(&target_size)?;
        self.vertices_i8.resize(&target_size)?;
        self.vertices_i16.resize(&target_size)?;
        self.vertices_i32.resize(&target_size)?;
        self.vertices_i64.resize(&target_size)?;
        self.vertices_u8.resize(&target_size)?;
        self.vertices_u16.resize(&target_size)?;
        self.vertices_u32.resize(&target_size)?;
        self.vertices_u64.resize(&target_size)?;
        self.vertices_isize.resize(&target_size)?;
        self.vertices_usize.resize(&target_size)?;
        self.vertices_char.resize(&target_size)?;
        self.vertices_unit.resize(&target_size)?;

        // for matrix in &mut self.vertices_enum {
        //     match matrix {
        //         VertexMatrixEnum::Bool(matrix) => {
        //             matrix.resize(&target_size)?;
        //         }
        //         _ => {
        //             todo!()
        //         }
        //     }
        // }

        Ok(())
    }
}

pub(crate) trait SparseVertexMatrix<G: GraphNativeDataType, M: MatrixDataType> {
    fn vertices_ref(&self) -> &SparseMatrix<M>;
    fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<M>;
}

macro_rules! implement_get_vertices {
    ($field_name:ident, $graph_data_type:ty, $store_data_type:ty) => {
        impl<T: MatrixDataType> SparseVertexMatrix<$graph_data_type, $store_data_type> for VertexStore<T> {
            fn vertices_ref(&self) -> &SparseMatrix<$store_data_type> {
                &self.$field_name
            }
            fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<$store_data_type> {
                &mut self.$field_name
            }
        }
    };
}

implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types!(implement_get_vertices, vertices);

pub(crate) trait SetVertexData<G: GraphNativeDataType + ConvertScalarToMatrixType<G, M>, M: MatrixDataType> {
    fn set_vertex_value(&mut self, index: Index, value: G) -> Result<(), GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($graph_type:ty, $matrix_type:ty) => {
        impl<T: MatrixDataType> SetVertexData<$graph_type, $matrix_type> for VertexStore<T> {
            fn set_vertex_value(&mut self, index: Index, value: $graph_type) -> Result<(), GraphComputingError> {
                let implementation_value = value.to_matrix_type();
                SparseVertexMatrix::<$graph_type, $matrix_type>::vertices_mut_ref(self).set_element((index, index, implementation_value).into())?;
                Ok(())
            }
        }
    };
}

implement_macro_for_all_native_and_matrix_data_types!(implement_set_vertex_data);

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
