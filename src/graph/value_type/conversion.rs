// use crate::graph::graph::indexed_vertex_and_adjacency_matrix_store::native_store_data_type::V as StoreNativeDataType;
use super::ValueType;

use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::value_type::NativeDataType as GraphNativeDataType,
};

pub(crate) trait ConvertScalarToMatrixType<GraphType: GraphNativeDataType, MatrixType: ValueType> {
    fn to_matrix_type(self) -> MatrixType;
}

macro_rules! implement_convert_scalar_to_matrix_type {
    ($graph_type:ty, $matrix_type:ty) => {
        impl ConvertScalarToMatrixType<$graph_type, $matrix_type> for $graph_type {
            fn to_matrix_type(self) -> $matrix_type {
                self
            }
        }
    };
}

implement_convert_scalar_to_matrix_type!(bool, bool);
implement_convert_scalar_to_matrix_type!(i8, i8);
implement_convert_scalar_to_matrix_type!(i16, i16);
implement_convert_scalar_to_matrix_type!(i32, i32);
implement_convert_scalar_to_matrix_type!(i64, i64);
implement_convert_scalar_to_matrix_type!(u8, u8);
implement_convert_scalar_to_matrix_type!(u16, u16);
implement_convert_scalar_to_matrix_type!(u32, u32);
implement_convert_scalar_to_matrix_type!(u64, u64);
implement_convert_scalar_to_matrix_type!(f32, f32);
implement_convert_scalar_to_matrix_type!(f64, f64);
implement_convert_scalar_to_matrix_type!(isize, isize);
implement_convert_scalar_to_matrix_type!(usize, usize);

impl ConvertScalarToMatrixType<char, u32> for char {
    fn to_matrix_type(self) -> u32 {
        self as u32
    }
}

impl ConvertScalarToMatrixType<(), bool> for () {
    fn to_matrix_type(self) -> bool {
        false
    }
}

pub(crate) trait ConvertScalarToGraphType<MatrixType: ValueType, GraphType: GraphNativeDataType> {
    fn to_graph_type(self) -> Result<GraphType, GraphComputingError>;
}

macro_rules! implement_convert_scalar_to_graph_type {
    ($matrix_type:ty, $graph_type:ty) => {
        impl ConvertScalarToGraphType<$matrix_type, $graph_type> for $graph_type {
            fn to_graph_type(self) -> Result<$graph_type, GraphComputingError> {
                Ok(self)
            }
        }
    };
}

implement_convert_scalar_to_graph_type!(bool, bool);
implement_convert_scalar_to_graph_type!(i8, i8);
implement_convert_scalar_to_graph_type!(i16, i16);
implement_convert_scalar_to_graph_type!(i32, i32);
implement_convert_scalar_to_graph_type!(i64, i64);
implement_convert_scalar_to_graph_type!(u8, u8);
implement_convert_scalar_to_graph_type!(u16, u16);
implement_convert_scalar_to_graph_type!(u32, u32);
implement_convert_scalar_to_graph_type!(u64, u64);
implement_convert_scalar_to_graph_type!(f32, f32);
implement_convert_scalar_to_graph_type!(f64, f64);
implement_convert_scalar_to_graph_type!(isize, isize);
implement_convert_scalar_to_graph_type!(usize, usize);

impl ConvertScalarToGraphType<u32, char> for u32 {
    fn to_graph_type(self) -> Result<char, GraphComputingError> {
        match char::from_u32(self) {
            Some(character) => Ok(character),
            None => Err(LogicError::new(
                LogicErrorType::InvalidCharacter,
                format!(
                    "Graph operations resulted in an invalid character value: {}",
                    self
                ),
                None,
            )
            .into()),
        }
    }
}

impl ConvertScalarToGraphType<bool, ()> for bool {
    fn to_graph_type(self) -> Result<(), GraphComputingError> {
        Ok(())
    }
}
