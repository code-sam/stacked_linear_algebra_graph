use std::fmt::Display;
use std::mem;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::clone_graphblas_vector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::new_graphblas_vector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetGraphblasSparseVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::context::GetContext;
use graphblas_sparse_linear_algebra::graphblas_bindings::GrB_Vector;
use graphblas_sparse_linear_algebra::graphblas_bindings::GrB_Vector_free;
use graphblas_sparse_linear_algebra::operators::apply::ApplyUnaryOperator;
use graphblas_sparse_linear_algebra::operators::apply::UnaryOperatorApplier;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::mask::VectorMask;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::unary_operator::Identity;
use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::ElementCount;
use crate::graph::value_type::implement_1_type_macro_with_enum_type_indentifier_for_all_value_types;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::GetValueTypeIdentifier;
use crate::graph::value_type::GetValueTypeIdentifierRef;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::ValueTypeIdentifier;

use super::GetVectorLength;

unsafe impl Send for VertexVector {}
unsafe impl Sync for VertexVector {}

#[derive(Debug)]
pub(crate) struct VertexVector {
    graphblas_context: Arc<GraphBLASContext>,
    value_type: ValueTypeIdentifier,
    sparse_vector: GrB_Vector,
}

pub(crate) trait CreateVertexVector<T> {
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<VertexVector, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> CreateVertexVector<T> for VertexVector {
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<VertexVector, GraphComputingError> {
        Ok(VertexVector {
            graphblas_context: graphblas_context.clone(),
            sparse_vector: unsafe {
                new_graphblas_vector(
                    &graphblas_context,
                    initial_vertex_capacity,
                    T::to_graphblas_type(),
                )?
            },
            value_type: T::value_type_identifier(),
        })
    }
}

impl Drop for VertexVector {
    fn drop(&mut self) -> () {
        let _ = self
            .graphblas_context
            .call_without_detailed_error_information(|| unsafe {
                GrB_Vector_free(&mut self.sparse_vector)
            });
    }
}

impl Clone for VertexVector {
    fn clone(&self) -> Self {
        VertexVector {
            graphblas_context: self.graphblas_context.to_owned(),
            value_type: self.value_type.to_owned(),
            sparse_vector: unsafe {
                clone_graphblas_vector(self.context_ref(), self.graphblas_vector_ref()).unwrap()
            },
        }
    }
}

impl Display for VertexVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VertexVector:");
        writeln!(f, "graphblas_context: {:?}", self.graphblas_context);
        writeln!(f, "value_type: {:?}", self.value_type);
        writeln!(
            f,
            "sparse_vector: \n{}",
            <VertexVector as ToSparseVector<f64>>::to_sparse_vector(self).unwrap()
        );
        return writeln!(f, "");
    }
}

impl GetGraphblasContext for VertexVector {
    fn graphblas_context(&self) -> Arc<GraphBLASContext> {
        self.graphblas_context.clone()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }
}

impl GetContext for VertexVector {
    fn context(&self) -> Arc<GraphBLASContext> {
        self.graphblas_context.clone()
    }

    fn context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }
}

impl GetGraphblasSparseVector for VertexVector {
    unsafe fn graphblas_vector(&self) -> GrB_Vector {
        self.sparse_vector
    }

    unsafe fn graphblas_vector_ref(&self) -> &GrB_Vector {
        &self.sparse_vector
    }

    unsafe fn graphblas_vector_mut_ref(&mut self) -> &mut GrB_Vector {
        &mut self.sparse_vector
    }
}

impl VectorMask for VertexVector {
    unsafe fn graphblas_vector(&self) -> GrB_Vector {
        self.sparse_vector
    }
}

impl GetValueTypeIdentifierRef for VertexVector {
    fn value_type_identifier_ref(&self) -> &ValueTypeIdentifier {
        &self.value_type
    }
}

// TODO: this approach should work once Type Alias Impl Trait (TAIT) is stable
// https://github.com/rust-lang/rust/issues/63063
// fn apply_to_adjacency_matrices_of_all_value_types<T: ValueType, F: Fn(&SparseVector<T>) -> Result<(), GraphComputingError>>(&self, f: F) -> Result<(), GraphComputingError> {
//     f(&self.sparse_vector_bool)?;
//     Ok(())
// }

pub trait ToSparseVector<T: ValueType> {
    fn to_sparse_vector(&self) -> Result<SparseVector<T>, GraphComputingError>;
}

pub trait IntoSparseVector<T: ValueType> {
    fn into_sparse_vector(self) -> Result<SparseVector<T>, GraphComputingError>;
}

impl<T: ValueType + ToSparseVectorForValueType<T>> ToSparseVector<T> for VertexVector {
    fn to_sparse_vector(&self) -> Result<SparseVector<T>, GraphComputingError> {
        T::to_sparse_vector(self)
    }
}

impl<T: ValueType + IntoSparseVectorForValueType<T>> IntoSparseVector<T> for VertexVector {
    fn into_sparse_vector(self) -> Result<SparseVector<T>, GraphComputingError> {
        T::into_sparse_vector(self)
    }
}

pub(crate) trait ToSparseVectorForValueType<T: ValueType> {
    fn to_sparse_vector(
        vector: &(impl GetContext
              + GetGraphblasSparseVector
              + GetVectorLength
              + GetValueTypeIdentifierRef),
    ) -> Result<SparseVector<T>, GraphComputingError>;
}

pub(crate) trait IntoSparseVectorForValueType<T: ValueType> {
    fn into_sparse_vector(vector: VertexVector) -> Result<SparseVector<T>, GraphComputingError>;
}

pub(crate) trait IntoSparseVectorAndClearValuesForValueType<T: ValueType> {
    fn into_sparse_vector_and_clear_values(
        vector: &mut VertexVector,
    ) -> Result<SparseVector<T>, GraphComputingError>;
}

macro_rules! implement_to_sparse_vector_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl ToSparseVectorForValueType<$value_type> for $value_type {
            fn to_sparse_vector(
                vector: &(impl GetContext
                      + GetGraphblasSparseVector
                      + GetVectorLength
                      + GetValueTypeIdentifierRef),
            ) -> Result<SparseVector<$value_type>, GraphComputingError> {
                match vector.value_type_identifier_ref() {
                    &ValueTypeIdentifier::$value_type_identifier => unsafe {
                        Ok(SparseVector::<$value_type>::from_graphblas_vector(
                            vector.context(),
                            clone_graphblas_vector(
                                vector.context_ref(),
                                vector.graphblas_vector_ref(),
                            )?,
                        )?)
                    },
                    _ => {
                        let mut product_vector =
                            SparseVector::<$value_type>::new(vector.context(), vector.length()?)?;

                        UnaryOperatorApplier::new().apply_to_vector(
                            &Identity::<$value_type>::new(),
                            vector,
                            &Assignment::<$value_type>::new(),
                            &mut product_vector,
                            &SelectEntireVector::new(vector.context()),
                            &OperatorOptions::new_default(),
                        )?;

                        return Ok(product_vector);
                    }
                }
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_to_sparse_vector_for_value_type
);

macro_rules! implement_into_sparse_vector_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl IntoSparseVectorForValueType<$value_type> for $value_type {
            fn into_sparse_vector(
                mut vertex_vector: VertexVector,
            ) -> Result<SparseVector<$value_type>, GraphComputingError> {
                <$value_type>::into_sparse_vector_and_clear_values(&mut vertex_vector)
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_into_sparse_vector_for_value_type
);

macro_rules! implement_into_sparse_vector_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl IntoSparseVectorAndClearValuesForValueType<$value_type> for $value_type {
            fn into_sparse_vector_and_clear_values(
                vertex_vector: &mut VertexVector,
            ) -> Result<SparseVector<$value_type>, GraphComputingError> {
                match vertex_vector.value_type_identifier_ref() {
                    &ValueTypeIdentifier::$value_type_identifier => unsafe {
                        let mut graphblas_vector = new_graphblas_vector(
                            &vertex_vector.graphblas_context,
                            vertex_vector.length()?,
                            <$value_type>::to_graphblas_type(),
                        )?;

                        mem::swap(&mut graphblas_vector, &mut vertex_vector.sparse_vector);

                        Ok(SparseVector::<$value_type>::from_graphblas_vector(
                            vertex_vector.context(),
                            graphblas_vector,
                        )?)
                    },
                    _ => {
                        let mut product_vector = SparseVector::<$value_type>::new(
                            vertex_vector.context(),
                            vertex_vector.length()?,
                        )?;

                        UnaryOperatorApplier::new().apply_to_vector(
                            &Identity::<$value_type>::new(),
                            vertex_vector,
                            &Assignment::<$value_type>::new(),
                            &mut product_vector,
                            &SelectEntireVector::new(vertex_vector.context()),
                            &OperatorOptions::new_default(),
                        )?;

                        return Ok(product_vector);
                    }
                }
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_into_sparse_vector_for_value_type
);

pub(crate) trait CreateSparseVectorForValueType<T: ValueType> {
    fn new_sparse_vector(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<SparseVector<T>, GraphComputingError>;
}

macro_rules! implement_create_sparse_vector_for_value_type {
    ($value_type:ty) => {
        impl CreateSparseVectorForValueType<$value_type> for $value_type {
            fn new_sparse_vector(
                graphblas_context: Arc<GraphBLASContext>,
                initial_vertex_capacity: ElementCount,
            ) -> Result<SparseVector<$value_type>, GraphComputingError> {
                Ok(SparseVector::<$value_type>::new(
                    graphblas_context,
                    initial_vertex_capacity,
                )?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_create_sparse_vector_for_value_type);
