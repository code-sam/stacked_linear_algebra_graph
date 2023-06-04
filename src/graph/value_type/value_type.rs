// pub(crate) use graphblas_sparse_linear_algebra::value_type::ValueType;
use graphblas_sparse_linear_algebra::{
    collections::{
        sparse_matrix::{GraphblasSparseMatrixTrait, SparseMatrix},
        sparse_vector::SparseVector,
    },
    context::ContextTrait,
    operators::mask::{MatrixMask, VectorMask},
    value_type::ValueType as GraphBLASValueType,
};

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixSparseMatrixTrait;
use crate::graph::{
    edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix,
    vertex_store::{SparseVertexVector, VertexVector},
};

use super::implement_macro_for_all_native_value_types;

// use crate::{graph::data_type::NativeDataType as GraphNativeDataType, error::{GraphComputingError, LogicError, LogicErrorType}};

// pub(crate) use ValueType as StoreNativeDataType;
// pub(crate) trait NativeDataType: ValueType {}

// impl NativeDataType for bool {}
// impl NativeDataType for i8 {}
// impl NativeDataType for i16 {}
// impl NativeDataType for i32 {}
// impl NativeDataType for i64 {}
// impl NativeDataType for u8 {}
// impl NativeDataType for u16 {}
// impl NativeDataType for u32 {}
// impl NativeDataType for u64 {}
// impl NativeDataType for isize {}
// impl NativeDataType for usize {}
// impl NativeDataType for f32 {}
// impl NativeDataType for f64 {}

// impl SparseVertexVectorForValueType<bool> for bool {
//     fn sparse_vector_ref(vertex_vector: &VertexVector) -> &SparseVector<bool> {
//         SparseVertexVector::<bool>::sparse_vector_ref(vertex_vector)
//     }
// }

pub trait ValueType: GraphBLASValueType {
    // fn value_type_enum() -> ValueTypeIndex;
    // fn vertex_vector_ref()
    // fn sparse_vector_ref(vertex_vector: &VertexVector) -> SparseVector<T>;
}

impl ValueType for bool {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::Boolean
    // }
}
impl ValueType for i8 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::Integer8Bit
    // }
}
impl ValueType for i16 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::Integer16Bit
    // }
}
impl ValueType for i32 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::Integer32Bit
    // }
}
impl ValueType for i64 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::Integer64Bit
    // }
}
impl ValueType for u8 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::UnsignedInteger8Bit
    // }
}
impl ValueType for u16 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::UnsignedInteger16Bit
    // }
}
impl ValueType for u32 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::UnsignedInteger32Bit
    // }
}
impl ValueType for u64 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::UnsignedInteger64Bit
    // }
}
impl ValueType for f32 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::FloatingPoint32Bit
    // }
}
impl ValueType for f64 {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::FloatingPoint64Bit
    // }
}
impl ValueType for isize {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::PointerSizedInteger
    // }
}
impl ValueType for usize {
    // fn value_type_enum() -> ValueTypeIndex {
    //     ValueTypeIndex::PointerSizedUnsizedInteger
    // }
}

// #[derive(Clone, Debug)]
// pub enum ValueTypeIndex {
//     Boolean = 0,
//     Integer8Bit = 1,
//     Integer16Bit = 2,
//     Integer32Bit = 3,
//     Integer64Bit = 4,
//     UnsignedInteger8Bit = 5,
//     UnsignedInteger16Bit = 6,
//     UnsignedInteger32Bit = 7,
//     UnsignedInteger64Bit = 8,
//     FloatingPoint32Bit = 9,
//     FloatingPoint64Bit = 10,
//     PointerSizedInteger = 11,
//     PointerSizedUnsizedInteger = 12,
// }

pub trait SparseVertexVectorForValueType<T: ValueType>
where
    SparseVector<T>: VectorMask,
{
    fn sparse_vector_ref(vertex_vector: &VertexVector) -> &SparseVector<T>;
    fn sparse_vector_mut_ref(vertex_vector: &mut VertexVector) -> &mut SparseVector<T>;
}

macro_rules! implement_sparse_vertex_vector_for_value_type {
    ($value_type: ty) => {
        impl SparseVertexVectorForValueType<$value_type> for $value_type {
            fn sparse_vector_ref(vertex_vector: &VertexVector) -> &SparseVector<$value_type> {
                SparseVertexVector::<$value_type>::sparse_vector_ref(vertex_vector)
            }

            fn sparse_vector_mut_ref(
                vertex_vector: &mut VertexVector,
            ) -> &mut SparseVector<$value_type> {
                SparseVertexVector::<$value_type>::sparse_vector_mut_ref(vertex_vector)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_sparse_vertex_vector_for_value_type);

pub trait SparseAdjacencyMatrixForValueType<T: ValueType>
where
    SparseMatrix<T>: MatrixMask,
{
    fn sparse_matrix_ref(adjacency_matrix: &WeightedAdjacencyMatrix) -> &SparseMatrix<T>;
    // fn sparse_matrix_ref(adjacency_matrix: &WeightedAdjacencyMatrix) -> &impl (GraphblasSparseMatrixTrait + MatrixMask + ContextTrait);
    fn sparse_matrix_mut_ref(
        adjacency_matrix: &mut WeightedAdjacencyMatrix,
    ) -> &mut SparseMatrix<T>;
}

macro_rules! implement_sparse_vertex_vector_for_value_type {
    ($value_type: ty) => {
        impl SparseAdjacencyMatrixForValueType<$value_type> for $value_type {
            fn sparse_matrix_ref(
                adjacency_matrix: &WeightedAdjacencyMatrix,
            ) -> &SparseMatrix<$value_type> {
                WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                    adjacency_matrix,
                )
            }

            fn sparse_matrix_mut_ref(
                adjacency_matrix: &mut WeightedAdjacencyMatrix,
            ) -> &mut SparseMatrix<$value_type> {
                WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_mut_ref(
                    adjacency_matrix,
                )
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_sparse_vertex_vector_for_value_type);

// pub trait SparseAdjacencyMatrixMaskForValueType<T: ValueType> {
//     fn sparse_matrix_ref(adjacency_matrix: &WeightedAdjacencyMatrix) -> &MatrixMask;
//     fn sparse_matrix_mut_ref(
//         adjacency_matrix: &mut WeightedAdjacencyMatrix,
//     ) -> &mut SparseMatrix<T>;
// }

// macro_rules! implement_sparse_vertex_vector_for_value_type {
//     ($value_type: ty) => {
//         impl SparseAdjacencyMatrixForValueType<$value_type> for $value_type {
//             fn sparse_matrix_ref(
//                 adjacency_matrix: &WeightedAdjacencyMatrix,
//             ) -> &SparseMatrix<$value_type> {
//                 WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
//                     adjacency_matrix,
//                 )
//             }

//             fn sparse_matrix_mut_ref(
//                 adjacency_matrix: &mut WeightedAdjacencyMatrix,
//             ) -> &mut SparseMatrix<$value_type> {
//                 WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_mut_ref(
//                     adjacency_matrix,
//                 )
//             }
//         }
//     };
// }
// implement_macro_for_all_native_value_types!(implement_sparse_vertex_vector_for_value_type);
