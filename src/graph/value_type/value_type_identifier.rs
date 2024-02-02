use graphblas_sparse_linear_algebra::{
    operators::binary_operator::{Any, Assignment},
    value_type::ValueType,
};

use super::implement_1_type_macro_with_enum_type_indentifier_for_all_value_types;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum ValueTypeIdentifier {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    ISize,
    USize,
}

pub trait GetValueTypeIdentifierRef {
    fn value_type_identifier_ref(&self) -> &ValueTypeIdentifier;
}

pub trait GetValueTypeIdentifier {
    fn value_type_identifier() -> ValueTypeIdentifier;
}

macro_rules! implement_get_value_type_identifier {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl GetValueTypeIdentifier for $value_type {
            fn value_type_identifier() -> ValueTypeIdentifier {
                ValueTypeIdentifier::$value_type_identifier
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_get_value_type_identifier
);
