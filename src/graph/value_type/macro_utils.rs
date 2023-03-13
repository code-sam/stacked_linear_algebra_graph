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
pub(crate) use implement_macro_with_typed_indentifier_for_all_value_types;

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
pub(crate) use implement_1_type_macro_with_2_typed_indentifiers_for_all_value_types;

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
