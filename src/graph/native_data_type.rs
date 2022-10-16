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
impl NativeDataType for isize {}
impl NativeDataType for usize {}
// impl NativeDataType for char {}
// impl NativeDataType for () {}

macro_rules! implement_macro_for_all_native_data_types {
    ($macro_identifier:ident) => {
        $macro_identifier!(bool);
        $macro_identifier!(i8);
        $macro_identifier!(i16);
        $macro_identifier!(i32);
        $macro_identifier!(i64);
        $macro_identifier!(u8);
        $macro_identifier!(u16);
        $macro_identifier!(u32);
        $macro_identifier!(u64);
        $macro_identifier!(f32);
        $macro_identifier!(f64);
        $macro_identifier!(isize);
        $macro_identifier!(usize);
        // $macro_identifier!(char);
        // $macro_identifier!(());
    };
}
pub(crate) use implement_macro_for_all_native_data_types;

macro_rules! implement_macro_with_typed_variable_for_all_native_data_types {
    ($macro_identifier:ident) => {
        $macro_identifier!(bool, bool);
        $macro_identifier!(i8, i8);
        $macro_identifier!(i16, i16);
        $macro_identifier!(i32, i32);
        $macro_identifier!(i64, i64);
        $macro_identifier!(u8, u8);
        $macro_identifier!(u16, u16);
        $macro_identifier!(u32, u32);
        $macro_identifier!(u64, u64);
        $macro_identifier!(f32, f32);
        $macro_identifier!(f64, f64);
        $macro_identifier!(isize, isize);
        $macro_identifier!(usize, usize);
        // $macro_identifier!(char, char);
        // $macro_identifier!((), unit);
    };
}
pub(crate) use implement_macro_with_typed_variable_for_all_native_data_types;
