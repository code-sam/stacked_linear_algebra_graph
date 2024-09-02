use std::fmt::Debug;

use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;

use super::implement_macro_for_all_native_value_types;

pub trait ValueType: GraphblasValueType + Debug {}
// pub trait ValueType: GraphblasValueType + Debug {}

macro_rules! implement_value_type {
    ($value_type:ty) => {
        impl ValueType for $value_type {}
    };
}
implement_macro_for_all_native_value_types!(implement_value_type);

pub trait IntoValueType<T: ValueType> {
    fn into_value_type(self) -> T;
}

macro_rules! implement_into_value_type {
    ($from_value_type:ty, $into_value_type:ty) => {
        impl IntoValueType<$into_value_type> for $from_value_type {
            fn into_value_type(self) -> $into_value_type {
                self as $into_value_type
            }
        }
    };
}

macro_rules! implement_integer_into_bool_type {
    ($from_value_type:ty, $into_value_type:ty) => {
        impl IntoValueType<$into_value_type> for $from_value_type {
            fn into_value_type(self) -> $into_value_type {
                self != 0
            }
        }
    };
}

// macro_rules! implement_into_value_type_generic {
//     ($dollar:tt, $from_value_type:ty) => {
//         implement_into_value_type!($from_value_type, $dollar into_value_type);
//     };
// }

macro_rules! implement_into_value_type_for_all_from_value_types_except_to_bool {
    ($from_value_type:ty) => {
        // implement_into_value_type!($from_value_type, bool);
        implement_into_value_type!($from_value_type, i8);
        implement_into_value_type!($from_value_type, i16);
        implement_into_value_type!($from_value_type, i32);
        implement_into_value_type!($from_value_type, i64);
        implement_into_value_type!($from_value_type, u8);
        implement_into_value_type!($from_value_type, u16);
        implement_into_value_type!($from_value_type, u32);
        implement_into_value_type!($from_value_type, u64);
        // implement_into_value_type!($from_value_type, f32);
        // implement_into_value_type!($from_value_type, f64);
        implement_into_value_type!($from_value_type, isize);
        implement_into_value_type!($from_value_type, usize);
        // implement_macro_for_all_native_value_types!(
        //     implement_into_value_type_generic
        // );
    };
}

implement_macro_for_all_native_value_types!(
    implement_into_value_type_for_all_from_value_types_except_to_bool
);

macro_rules! implement_into_value_type_for_standard_cases_not_involving_bool {
    ($from_value_type:ty) => {
        implement_into_value_type!($from_value_type, f32);
        implement_into_value_type!($from_value_type, f64);
    };
}

// implement_into_value_type_for_standard_cases_not_involving_bool!(bool);
implement_into_value_type_for_standard_cases_not_involving_bool!(i8);
implement_into_value_type_for_standard_cases_not_involving_bool!(i16);
implement_into_value_type_for_standard_cases_not_involving_bool!(i32);
implement_into_value_type_for_standard_cases_not_involving_bool!(i64);
implement_into_value_type_for_standard_cases_not_involving_bool!(u8);
implement_into_value_type_for_standard_cases_not_involving_bool!(u16);
implement_into_value_type_for_standard_cases_not_involving_bool!(u32);
implement_into_value_type_for_standard_cases_not_involving_bool!(u64);
implement_into_value_type_for_standard_cases_not_involving_bool!(f32);
implement_into_value_type_for_standard_cases_not_involving_bool!(f64);
implement_into_value_type_for_standard_cases_not_involving_bool!(isize);
implement_into_value_type_for_standard_cases_not_involving_bool!(usize);

macro_rules! implement_into_value_type_for_integer_to_bool {
    ($from_value_type:ty) => {
        implement_integer_into_bool_type!($from_value_type, bool);
    };
}

// implement_into_value_type_for_integer_to_bool!(bool);
implement_into_value_type_for_integer_to_bool!(i8);
implement_into_value_type_for_integer_to_bool!(i16);
implement_into_value_type_for_integer_to_bool!(i32);
implement_into_value_type_for_integer_to_bool!(i64);
implement_into_value_type_for_integer_to_bool!(u8);
implement_into_value_type_for_integer_to_bool!(u16);
implement_into_value_type_for_integer_to_bool!(u32);
implement_into_value_type_for_integer_to_bool!(u64);
// implement_into_value_type_for_integer_to_bool!(f32);
// implement_into_value_type_for_integer_to_bool!(f64);
implement_into_value_type_for_integer_to_bool!(isize);
implement_into_value_type_for_integer_to_bool!(usize);

impl IntoValueType<bool> for bool {
    fn into_value_type(self) -> bool {
        self
    }
}
impl IntoValueType<bool> for f32 {
    fn into_value_type(self) -> bool {
        self != 0.0
    }
}
impl IntoValueType<bool> for f64 {
    fn into_value_type(self) -> bool {
        self != 0.0
    }
}
impl IntoValueType<f32> for bool {
    fn into_value_type(self) -> f32 {
        (self as u8) as f32
    }
}
impl IntoValueType<f64> for bool {
    fn into_value_type(self) -> f64 {
        (self as u8) as f64
    }
}

// macro_rules! implement_for_all_combinations {
//     ($($from_value_type:ty),*; $($into_value_type:ty),*) => {
//         $(
//             $(
//                 implement_into_value_type!($from_value_type, $into_value_type);
//             )*
//         )*
//     };
// }

// implement_for_all_combinations!(
//     bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize;
//     bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize
// );
