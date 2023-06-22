macro_rules! implement_macro_for_all_native_value_types {
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
    };
}
pub(crate) use implement_macro_for_all_native_value_types;

macro_rules! implement_macro_with_typed_indentifier_for_all_value_types {
    ($macro_identifier:ident, $untyped_ident:ident) => {
        paste::paste! {
            $macro_identifier!([<$untyped_ident _bool>]);
            $macro_identifier!([<$untyped_ident _i8>]);
            $macro_identifier!([<$untyped_ident _i16>]);
            $macro_identifier!([<$untyped_ident _i32>]);
            $macro_identifier!([<$untyped_ident _i64>]);
            $macro_identifier!([<$untyped_ident _u8>]);
            $macro_identifier!([<$untyped_ident _u16>]);
            $macro_identifier!([<$untyped_ident _u32>]);
            $macro_identifier!([<$untyped_ident _u64>]);
            $macro_identifier!([<$untyped_ident _f32>]);
            $macro_identifier!([<$untyped_ident _f64>]);
            $macro_identifier!([<$untyped_ident _isize>]);
            $macro_identifier!([<$untyped_ident _usize>]);
            // $macro_identifier!([<$untyped_ident _char>]);
            // $macro_identifier!([<$untyped_ident _unit>]);
        }
    };
}

macro_rules! implement_1_type_macro_with_typed_indentifier_for_all_value_types {
    ($macro_identifier:ident, $untyped_ident:ident) => {
        paste::paste! {
            $macro_identifier!([<$untyped_ident _bool>], bool);
            $macro_identifier!([<$untyped_ident _i8>], i8);
            $macro_identifier!([<$untyped_ident _i16>], i16);
            $macro_identifier!([<$untyped_ident _i32>], i32);
            $macro_identifier!([<$untyped_ident _i64>], i64);
            $macro_identifier!([<$untyped_ident _u8>], u8);
            $macro_identifier!([<$untyped_ident _u16>], u16);
            $macro_identifier!([<$untyped_ident _u32>], u32);
            $macro_identifier!([<$untyped_ident _u64>], u64);
            $macro_identifier!([<$untyped_ident _f32>], f32);
            $macro_identifier!([<$untyped_ident _f64>], f64);
            $macro_identifier!([<$untyped_ident _isize>], isize);
            $macro_identifier!([<$untyped_ident _usize>], usize);
            // $macro_identifier!([<$untyped_ident _char>], char);
            // $macro_identifier!([<$untyped_ident _unit>], ());
        }
    };
}
pub(crate) use implement_1_type_macro_with_typed_indentifier_for_all_value_types;

macro_rules! implement_1_type_macro_with_2_typed_indentifiers_for_all_value_types {
    ($macro_identifier:ident, $untyped_ident_1:ident, $untyped_ident_2:ident) => {
        paste::paste! {
            $macro_identifier!([<$untyped_ident_1 _bool>], [<$untyped_ident_2 _bool>], bool);
            $macro_identifier!([<$untyped_ident_1 _i8>], [<$untyped_ident_2 _i8>], i8);
            $macro_identifier!([<$untyped_ident_1 _i16>], [<$untyped_ident_2 _i16>], i16);
            $macro_identifier!([<$untyped_ident_1 _i32>], [<$untyped_ident_2 _i32>], i32);
            $macro_identifier!([<$untyped_ident_1 _i64>], [<$untyped_ident_2 _i64>], i64);
            $macro_identifier!([<$untyped_ident_1 _u8>], [<$untyped_ident_2 _u8>], u8);
            $macro_identifier!([<$untyped_ident_1 _u16>], [<$untyped_ident_2 _u16>], u16);
            $macro_identifier!([<$untyped_ident_1 _u32>], [<$untyped_ident_2 _u32>], u32);
            $macro_identifier!([<$untyped_ident_1 _u64>], [<$untyped_ident_2 _u64>], u64);
            $macro_identifier!([<$untyped_ident_1 _f32>], [<$untyped_ident_2 _f32>], f32);
            $macro_identifier!([<$untyped_ident_1 _f64>], [<$untyped_ident_2 _f64>], f64);
            $macro_identifier!([<$untyped_ident_1 _isize>], [<$untyped_ident_2 _isize>], isize);
            $macro_identifier!([<$untyped_ident_1 _usize>], [<$untyped_ident_2 _usize>], usize);
            // $macro_identifier!([<$untyped_ident _char>], char);
            // $macro_identifier!([<$untyped_ident _unit>], ());
        }
    };
}

macro_rules! implement_3_type_macro_for_all_native_value_types {
    ($macro_identifier:ident) => {
        implement_3_type_macro_stage_1!($macro_identifier, bool);
        implement_3_type_macro_stage_1!($macro_identifier, i8);
        implement_3_type_macro_stage_1!($macro_identifier, i16);
        implement_3_type_macro_stage_1!($macro_identifier, i32);
        implement_3_type_macro_stage_1!($macro_identifier, i64);
        implement_3_type_macro_stage_1!($macro_identifier, u8);
        implement_3_type_macro_stage_1!($macro_identifier, u16);
        implement_3_type_macro_stage_1!($macro_identifier, u32);
        implement_3_type_macro_stage_1!($macro_identifier, u64);
        implement_3_type_macro_stage_1!($macro_identifier, f32);
        implement_3_type_macro_stage_1!($macro_identifier, f64);
        implement_3_type_macro_stage_1!($macro_identifier, isize);
        implement_3_type_macro_stage_1!($macro_identifier, usize);
    };
}

macro_rules! implement_3_type_macro_stage_1 {
    ($macro_identifier:ident, $value_type:ty) => {
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, bool);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, i8);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, i16);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, i32);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, i64);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, u8);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, u16);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, u32);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, u64);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, f32);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, f64);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, isize);
        implement_3_type_macro_stage_2!($macro_identifier, $value_type, usize);
    };
}

macro_rules! implement_3_type_macro_stage_2 {
    ($macro_identifier:ident, $value_type_1:ty, $value_type_2:ty) => {
        $macro_identifier!($value_type_1, $value_type_2, bool);
        $macro_identifier!($value_type_1, $value_type_2, i8);
        $macro_identifier!($value_type_1, $value_type_2, i16);
        $macro_identifier!($value_type_1, $value_type_2, i32);
        $macro_identifier!($value_type_1, $value_type_2, i64);
        $macro_identifier!($value_type_1, $value_type_2, u8);
        $macro_identifier!($value_type_1, $value_type_2, u16);
        $macro_identifier!($value_type_1, $value_type_2, u32);
        $macro_identifier!($value_type_1, $value_type_2, u64);
        $macro_identifier!($value_type_1, $value_type_2, f32);
        $macro_identifier!($value_type_1, $value_type_2, f64);
        $macro_identifier!($value_type_1, $value_type_2, isize);
        $macro_identifier!($value_type_1, $value_type_2, usize);
    };
}

macro_rules! implement_4_type_macro_for_all_native_value_types {
    ($macro_identifier:ident) => {
        implement_4_type_macro_stage_1!($macro_identifier, bool);
        implement_4_type_macro_stage_1!($macro_identifier, i8);
        implement_4_type_macro_stage_1!($macro_identifier, i16);
        implement_4_type_macro_stage_1!($macro_identifier, i32);
        implement_4_type_macro_stage_1!($macro_identifier, i64);
        implement_4_type_macro_stage_1!($macro_identifier, u8);
        implement_4_type_macro_stage_1!($macro_identifier, u16);
        implement_4_type_macro_stage_1!($macro_identifier, u32);
        implement_4_type_macro_stage_1!($macro_identifier, u64);
        implement_4_type_macro_stage_1!($macro_identifier, f32);
        implement_4_type_macro_stage_1!($macro_identifier, f64);
        implement_4_type_macro_stage_1!($macro_identifier, isize);
        implement_4_type_macro_stage_1!($macro_identifier, usize);
    };
}

macro_rules! implement_4_type_macro_stage_1 {
    ($macro_identifier:ident, $value_type:ty) => {
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, bool);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, i8);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, i16);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, i32);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, i64);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, u8);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, u16);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, u32);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, u64);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, f32);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, f64);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, isize);
        implement_4_type_macro_stage_2!($macro_identifier, $value_type, usize);
    };
}

macro_rules! implement_4_type_macro_stage_2 {
    ($macro_identifier:ident, $value_type_1:ty, $value_type_2:ty) => {
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, bool);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, i8);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, i16);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, i32);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, i64);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, u8);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, u16);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, u32);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, u64);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, f32);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, f64);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, isize);
        implement_4_type_macro_stage_3!($macro_identifier, $value_type_1, $value_type_2, usize);
    };
}

macro_rules! implement_4_type_macro_stage_3 {
    ($macro_identifier:ident, $value_type_1:ty, $value_type_2:ty, $value_type_3:ty) => {
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, bool);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, i8);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, i16);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, i32);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, i64);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, u8);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, u16);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, u32);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, u64);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, f32);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, f64);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, isize);
        $macro_identifier!($value_type_1, $value_type_2, $value_type_3, usize);
    };
}

// macro_rules! implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types {
//     ($macro_identifier:ident, $untyped_ident:ident) => {
//         paste::paste! {
//             $macro_identifier!([<$untyped_ident _bool>], bool, bool);
//             $macro_identifier!([<$untyped_ident _i8>], i8, i8);
//             $macro_identifier!([<$untyped_ident _i16>], i16, i16);
//             $macro_identifier!([<$untyped_ident _i32>], i32, i32);
//             $macro_identifier!([<$untyped_ident _i64>], i64, i64);
//             $macro_identifier!([<$untyped_ident _u8>], u8, u8);
//             $macro_identifier!([<$untyped_ident _u16>], u16, u16);
//             $macro_identifier!([<$untyped_ident _u32>], u32, u32);
//             $macro_identifier!([<$untyped_ident _u64>], u64, u64);
//             $macro_identifier!([<$untyped_ident _f32>], f32, f32);
//             $macro_identifier!([<$untyped_ident _f64>], f64, f64);
//             $macro_identifier!([<$untyped_ident _isize>], isize, isize);
//             $macro_identifier!([<$untyped_ident _usize>], usize, usize);
//             $macro_identifier!([<$untyped_ident _char>], char, u32);
//             $macro_identifier!([<$untyped_ident _unit>], (), bool);
//         }
//     };
// }
// pub(crate) use implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types;

// macro_rules! implement_macro_for_all_native_and_matrix_data_types {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(bool, bool);
//         $macro_identifier!(i8, i8);
//         $macro_identifier!(i16, i16);
//         $macro_identifier!(i32, i32);
//         $macro_identifier!(i64, i64);
//         $macro_identifier!(u8, u8);
//         $macro_identifier!(u16, u16);
//         $macro_identifier!(u32, u32);
//         $macro_identifier!(u64, u64);
//         $macro_identifier!(f32, f32);
//         $macro_identifier!(f64, f64);
//         $macro_identifier!(isize, isize);
//         $macro_identifier!(usize, usize);
//         $macro_identifier!(char, u32);
//         $macro_identifier!((), bool);
//     };
// }
// pub(crate) use implement_macro_for_all_native_and_matrix_data_types;

// macro_rules! implement_macro_for_all_native_data_types {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(bool);
//         $macro_identifier!(i8);
//         $macro_identifier!(i16);
//         $macro_identifier!(i32);
//         $macro_identifier!(i64);
//         $macro_identifier!(u8);
//         $macro_identifier!(u16);
//         $macro_identifier!(u32);
//         $macro_identifier!(u64);
//         $macro_identifier!(f32);
//         $macro_identifier!(f64);
//         $macro_identifier!(isize);
//         $macro_identifier!(usize);
//         $macro_identifier!(char);
//         $macro_identifier!(());
//     };
// }
// pub(crate) use implement_macro_for_all_native_data_types;

// macro_rules! implement_macro_with_typed_variable_for_all_native_data_types {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(bool, bool);
//         $macro_identifier!(i8, i8);
//         $macro_identifier!(i16, i16);
//         $macro_identifier!(i32, i32);
//         $macro_identifier!(i64, i64);
//         $macro_identifier!(u8, u8);
//         $macro_identifier!(u16, u16);
//         $macro_identifier!(u32, u32);
//         $macro_identifier!(u64, u64);
//         $macro_identifier!(f32, f32);
//         $macro_identifier!(f64, f64);
//         $macro_identifier!(isize, isize);
//         $macro_identifier!(usize, usize);
//         $macro_identifier!(char, u32);
//         $macro_identifier!((), bool);
//     };
// }
// pub(crate) use implement_macro_with_typed_variable_for_all_native_data_types;
