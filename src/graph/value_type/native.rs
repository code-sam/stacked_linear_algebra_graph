pub trait NativeDataType {}

// TODO: think about ways to support data types that are non-native to graphblas_sparse_linear_algebra.
// char and unit may also be support through a kind of edge_type specific vertex values.

impl NativeDataType for bool {}
impl NativeDataType for i8 {}
impl NativeDataType for i16 {}
impl NativeDataType for i32 {}
impl NativeDataType for i64 {}
impl NativeDataType for u8 {}
impl NativeDataType for u16 {}
impl NativeDataType for u32 {}
impl NativeDataType for u64 {}
impl NativeDataType for f32 {}
impl NativeDataType for f64 {}
impl NativeDataType for isize {} // TODO: think about the value and cost of supporting usize and isize
impl NativeDataType for usize {}
impl NativeDataType for char {} // u32 can represent a char
impl NativeDataType for () {}
