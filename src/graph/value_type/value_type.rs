use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GraphblasSparseMatrixTrait;
use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;
use graphblas_sparse_linear_algebra::{
    collections::{sparse_matrix::SparseMatrix, sparse_vector::SparseVector},
    operators::mask::VectorMask,
};

use crate::graph::edge_store::weighted_adjacency_matrix::{
    SparseWeightedAdjacencyMatrix, SparseWeightedAdjacencyMatrixForValueType,
};
use crate::graph::vertex_store::SparseVertexMatrix;
use crate::graph::{
    edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix, vertex_store::VertexMatrixStore,
};

use super::implement_macro_for_all_native_value_types;

pub trait ValueType: GraphblasValueType {
    fn value_type_index() -> ValueTypeIndex;
}

impl ValueType for bool {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::Boolean
    }
}
impl ValueType for i8 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::Integer8Bit
    }
}
impl ValueType for i16 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::Integer16Bit
    }
}
impl ValueType for i32 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::Integer32Bit
    }
}
impl ValueType for i64 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::Integer64Bit
    }
}
impl ValueType for u8 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::UnsignedInteger8Bit
    }
}
impl ValueType for u16 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::UnsignedInteger16Bit
    }
}
impl ValueType for u32 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::UnsignedInteger32Bit
    }
}
impl ValueType for u64 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::UnsignedInteger64Bit
    }
}
impl ValueType for f32 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::FloatingPoint32Bit
    }
}
impl ValueType for f64 {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::FloatingPoint64Bit
    }
}
impl ValueType for isize {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::PointerSizedInteger
    }
}
impl ValueType for usize {
    fn value_type_index() -> ValueTypeIndex {
        ValueTypeIndex::PointerSizedUnsizedInteger
    }
}

pub trait ValueTypeIndexer<T: ValueType> {
    fn value_type_index() -> u8;
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum ValueTypeIndex {
    Boolean = 0u8,
    Integer8Bit = 1u8,
    Integer16Bit = 2u8,
    Integer32Bit = 3u8,
    Integer64Bit = 4u8,
    UnsignedInteger8Bit = 5u8,
    UnsignedInteger16Bit = 6u8,
    UnsignedInteger32Bit = 7u8,
    UnsignedInteger64Bit = 8u8,
    FloatingPoint32Bit = 9u8,
    FloatingPoint64Bit = 10u8,
    PointerSizedInteger = 11u8,
    PointerSizedUnsizedInteger = 12u8,
}

impl ValueTypeIndex {
    fn from_index(index: u8) -> Self {
        // https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
        todo!()
    }
}

impl ValueTypeIndexer<bool> for bool {
    fn value_type_index() -> u8 {
        ValueTypeIndex::Boolean as u8
    }
}

pub trait SparseVertexMatrixForValueType<T: ValueType>
where
    SparseMatrix<T>: GraphblasSparseMatrixTrait,
{
    fn sparse_matrix_ref(vertex_matrix: &VertexMatrixStore) -> &SparseMatrix<T>;
    fn sparse_matrix_mut_ref(vertex_matrix: &mut VertexMatrixStore) -> &mut SparseMatrix<T>;
}

macro_rules! implement_sparse_vertex_matrix_for_value_type {
    ($value_type: ty) => {
        impl SparseVertexMatrixForValueType<$value_type> for $value_type {
            fn sparse_matrix_ref(vertex_matrix: &VertexMatrixStore) -> &SparseMatrix<$value_type> {
                SparseVertexMatrix::<$value_type>::sparse_matrix_ref(vertex_matrix)
            }

            fn sparse_matrix_mut_ref(
                vertex_matrix: &mut VertexMatrixStore,
            ) -> &mut SparseMatrix<$value_type> {
                SparseVertexMatrix::<$value_type>::sparse_matrix_mut_ref(vertex_matrix)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_sparse_vertex_matrix_for_value_type);

// pub trait SparseAdjacencyMatrixForValueType<T: ValueType>
// where
//     SparseMatrix<T>: GraphblasSparseMatrixTrait,
// {
//     fn sparse_matrix_ref(adjacency_matrix: &WeightedAdjacencyMatrix) -> &SparseMatrix<T>;
//     // fn sparse_matrix_ref(adjacency_matrix: &WeightedAdjacencyMatrix) -> &impl (GraphblasSparseMatrixTrait + MatrixMask + ContextTrait);
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
