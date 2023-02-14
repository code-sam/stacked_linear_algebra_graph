// use crate::graph::graph::indexed_vertex_and_adjacency_matrix_store::native_store_data_type::V as StoreNativeDataType;
use graphblas_sparse_linear_algebra::value_type::ValueType as StoreNativeDataType;

use crate::{graph::value_type::NativeDataType as GraphNativeDataType, error::{GraphComputingError, LogicError, LogicErrorType}};

pub(crate) trait ConvertScalarToStoreImplementationType<GraphType: GraphNativeDataType, StoreType: StoreNativeDataType> {
    fn to_implementation_type(self) -> StoreType;
}

macro_rules! implement_convert_scalar_to_store_type {
    ($graph_type:ty, $store_type:ty) => {
        impl ConvertScalarToStoreImplementationType<$graph_type, $store_type> for $graph_type {
            fn to_implementation_type(self) -> $store_type {
                self
            }
        }
    };
}

implement_convert_scalar_to_store_type!(bool, bool);
implement_convert_scalar_to_store_type!(i8, i8);
implement_convert_scalar_to_store_type!(i16, i16);
implement_convert_scalar_to_store_type!(i32, i32);
implement_convert_scalar_to_store_type!(i64, i64);
implement_convert_scalar_to_store_type!(u8, u8);
implement_convert_scalar_to_store_type!(u16, u16);
implement_convert_scalar_to_store_type!(u32, u32);
implement_convert_scalar_to_store_type!(u64, u64);
implement_convert_scalar_to_store_type!(f32, f32);
implement_convert_scalar_to_store_type!(f64, f64);

impl ConvertScalarToStoreImplementationType<char, u32> for char {
    fn to_implementation_type(self) -> u32 {
        self as u32
    }
}

impl ConvertScalarToStoreImplementationType<(), bool> for () {
    fn to_implementation_type(self) -> bool {
        false
    }
}

pub(crate) trait ConvertScalarToGraphType<StoreType: StoreNativeDataType, GraphType: GraphNativeDataType> {
    fn to_graph_type(self) -> Result<GraphType, GraphComputingError>;
}

macro_rules! implement_convert_scalar_to_graph_type {
    ($store_type:ty, $graph_type:ty) => {
        impl ConvertScalarToGraphType<$store_type, $graph_type> for $graph_type {
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

impl ConvertScalarToGraphType<u32, char> for u32 {
    fn to_graph_type(self) -> Result<char, GraphComputingError> {
       match char::from_u32(self) {
        Some(character) => Ok(character),
        None => Err(LogicError::new(LogicErrorType::InvalidCharacter, format!("Graph operations resulted in an invalid character value: {}", self), None).into())
       }
    }
}

impl ConvertScalarToGraphType<bool, ()> for bool {
    fn to_graph_type(self) -> Result<(), GraphComputingError> {
        Ok(())
    }
}
