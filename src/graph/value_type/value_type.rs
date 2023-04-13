pub(crate) use graphblas_sparse_linear_algebra::value_type::ValueType;

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

#[derive(Clone, Debug)]
pub enum NativeValueType {
    Boolean,
    Integer8Bit,
    Integer16Bit,
}
