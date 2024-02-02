use std::fmt::Debug;

use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;

pub trait ValueType: GraphblasValueType + Debug {}

impl ValueType for bool {}
impl ValueType for i8 {}
impl ValueType for i16 {}
impl ValueType for i32 {}
impl ValueType for i64 {}
impl ValueType for u8 {}
impl ValueType for u16 {}
impl ValueType for u32 {}
impl ValueType for u64 {}
impl ValueType for f32 {}
impl ValueType for f64 {}
impl ValueType for isize {}
impl ValueType for usize {}
