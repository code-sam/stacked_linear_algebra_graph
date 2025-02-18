use std::fmt::Display;
use std::mem;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    new_graphblas_matrix, GetGraphblasSparseMatrix,
};

use graphblas_sparse_linear_algebra::collections::sparse_matrix::clone_graphblas_matrix;
use graphblas_sparse_linear_algebra::context::GetContext;
use graphblas_sparse_linear_algebra::graphblas_bindings::{GrB_Matrix, GrB_Matrix_free};
use graphblas_sparse_linear_algebra::operators::apply::{ApplyUnaryOperator, UnaryOperatorApplier};
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::mask::{MatrixMask, SelectEntireMatrix};
use graphblas_sparse_linear_algebra::operators::options::OptionsForOperatorWithMatrixArgument;
use graphblas_sparse_linear_algebra::operators::unary_operator::Identity;
use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;

use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::ElementCount;
use crate::graph::value_type::{
    implement_1_type_macro_with_enum_type_indentifier_for_all_value_types,
    implement_macro_for_all_native_value_types, GetValueTypeIdentifier, GetValueTypeIdentifierRef,
    ValueType, ValueTypeIdentifier,
};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{Size, SparseMatrix};
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

use super::operations::GetMatrixSize;

static DEFAULT_OPERATOR_OPTIONS: Lazy<OptionsForOperatorWithAdjacencyMatrixArgument> =
    Lazy::new(|| OptionsForOperatorWithAdjacencyMatrixArgument::new_default());

static UNARY_OPERATOR_APPLIER: Lazy<UnaryOperatorApplier> =
    Lazy::new(|| UnaryOperatorApplier::new());

unsafe impl Send for WeightedAdjacencyMatrix {}
unsafe impl Sync for WeightedAdjacencyMatrix {}

#[derive(Debug)]
pub(crate) struct WeightedAdjacencyMatrix {
    graphblas_context: Arc<GraphBLASContext>,
    value_type: ValueTypeIdentifier,
    sparse_matrix: GrB_Matrix,
}

pub(crate) trait CreateWeightedAdjacencyMatrix<T> {
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<WeightedAdjacencyMatrix, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> CreateWeightedAdjacencyMatrix<T>
    for WeightedAdjacencyMatrix
{
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<WeightedAdjacencyMatrix, GraphComputingError> {
        Ok(WeightedAdjacencyMatrix {
            graphblas_context: graphblas_context.clone(),
            sparse_matrix: unsafe {
                new_graphblas_matrix(
                    &graphblas_context,
                    Size::new(initial_vertex_capacity, initial_vertex_capacity),
                    T::to_graphblas_type(),
                )?
            },
            value_type: T::value_type_identifier(),
        })
    }
}

impl Drop for WeightedAdjacencyMatrix {
    fn drop(&mut self) -> () {
        let _ = self
            .graphblas_context
            .call_without_detailed_error_information(|| unsafe {
                GrB_Matrix_free(&mut self.sparse_matrix)
            });
    }
}

impl Clone for WeightedAdjacencyMatrix {
    fn clone(&self) -> Self {
        WeightedAdjacencyMatrix {
            graphblas_context: self.graphblas_context.to_owned(),
            value_type: self.value_type.to_owned(),
            sparse_matrix: unsafe {
                clone_graphblas_matrix(self.context_ref(), self.graphblas_matrix_ref()).unwrap()
            },
        }
    }
}

impl Display for WeightedAdjacencyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "WeightedAdjacencyMatrix:");
        writeln!(f, "graphblas_context: {:?}", self.graphblas_context);
        writeln!(f, "value_type: {:?}", self.value_type);
        writeln!(
            f,
            "sparse_matrix: \n{}",
            <WeightedAdjacencyMatrix as ToSparseMatrix<f64>>::to_sparse_matrix(self).unwrap()
        );
        return writeln!(f, "");
    }
}

impl GetGraphblasContext for WeightedAdjacencyMatrix {
    fn graphblas_context(&self) -> Arc<GraphBLASContext> {
        self.graphblas_context.clone()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }
}

impl GetContext for WeightedAdjacencyMatrix {
    fn context(&self) -> Arc<GraphBLASContext> {
        self.graphblas_context.clone()
    }

    fn context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }
}

impl GetGraphblasSparseMatrix for WeightedAdjacencyMatrix {
    unsafe fn graphblas_matrix(&self) -> GrB_Matrix {
        self.sparse_matrix
    }

    unsafe fn graphblas_matrix_ref(&self) -> &GrB_Matrix {
        &self.sparse_matrix
    }

    unsafe fn graphblas_matrix_mut_ref(&mut self) -> &mut GrB_Matrix {
        &mut self.sparse_matrix
    }
}

impl MatrixMask for WeightedAdjacencyMatrix {
    unsafe fn graphblas_matrix(&self) -> GrB_Matrix {
        self.sparse_matrix
    }
}

impl GetValueTypeIdentifierRef for WeightedAdjacencyMatrix {
    fn value_type_identifier_ref(&self) -> &ValueTypeIdentifier {
        &self.value_type
    }
}

// TODO: this approach should work once Type Alias Impl Trait (TAIT) is stable
// https://github.com/rust-lang/rust/issues/63063
// fn apply_to_adjacency_matrices_of_all_value_types<T: ValueType, F: Fn(&SparseMatrix<T>) -> Result<(), GraphComputingError>>(&self, f: F) -> Result<(), GraphComputingError> {
//     f(&self.sparse_matrix_bool)?;
//     Ok(())
// }

// pub trait IntoSparseMatrix<T: ValueType> {
//     fn sparse_matrix(&self) -> Result<SparseMatrix<T>, GraphComputingError>;
// }

// impl<T: ValueType + IntoSparseMatrixForValueType<T>> IntoSparseMatrix<T>
//     for WeightedAdjacencyMatrix
// {
//     fn sparse_matrix(&self) -> Result<SparseMatrix<T>, GraphComputingError> {
//         T::sparse_matrix(self)
//     }
// }

// pub trait IntoSparseMatrixForValueType<T: ValueType> {
//     fn sparse_matrix(
//         matrix: &(impl GetContext
//               + GetGraphblasSparseMatrix
//               + GetMatrixSize
//               + GetValueTypeIdentifierRef),
//     ) -> Result<SparseMatrix<T>, GraphComputingError>;
// }

// macro_rules! implement_into_sparse_matrix_for_value_type {
//     ($value_type_identifier:ident, $value_type:ty) => {
//         impl IntoSparseMatrixForValueType<$value_type> for $value_type {
//             fn sparse_matrix(
//                 matrix: &(impl GetContext
//                       + GetGraphblasSparseMatrix
//                       + GetMatrixSize
//                       + GetValueTypeIdentifierRef),
//             ) -> Result<SparseMatrix<$value_type>, GraphComputingError> {
//                 match matrix.value_type_identifier_ref() {
//                     &ValueTypeIdentifier::$value_type_identifier => unsafe {
//                         Ok(SparseMatrix::<$value_type>::from_graphblas_matrix(
//                             matrix.context(),
//                             clone_graphblas_matrix(
//                                 matrix.context_ref(),
//                                 matrix.graphblas_matrix_ref(),
//                             )?,
//                         )?)
//                     },
//                     _ => {
//                         let mut product_matrix =
//                             SparseMatrix::<$value_type>::new(matrix.context(), matrix.size()?)?;

//                         UNARY_OPERATOR_APPLIER.apply_to_matrix(
//                             &Identity::<$value_type>::new(),
//                             matrix,
//                             &Assignment::<$value_type>::new(),
//                             &mut product_matrix,
//                             &SelectEntireMatrix::new(matrix.context()),
//                             &*DEFAULT_OPERATOR_OPTIONS,
//                         )?;

//                         return Ok(product_matrix);
//                     }
//                 }
//             }
//         }
//     };
// }
// implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
//     implement_into_sparse_matrix_for_value_type
// );

pub trait ToSparseMatrix<T: ValueType> {
    fn to_sparse_matrix(&self) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub trait IntoSparseMatrix<T: ValueType> {
    fn into_sparse_matrix(self) -> Result<SparseMatrix<T>, GraphComputingError>;
}

impl<T: ValueType + ToSparseMatrixForValueType<T>> ToSparseMatrix<T> for WeightedAdjacencyMatrix {
    fn to_sparse_matrix(&self) -> Result<SparseMatrix<T>, GraphComputingError> {
        T::to_sparse_matrix(self)
    }
}

impl<T: ValueType + IntoSparseMatrixForValueType<T>> IntoSparseMatrix<T>
    for WeightedAdjacencyMatrix
{
    fn into_sparse_matrix(self) -> Result<SparseMatrix<T>, GraphComputingError> {
        T::into_sparse_matrix(self)
    }
}

pub(crate) trait ToSparseMatrixForValueType<T: ValueType> {
    fn to_sparse_matrix(
        matrix: &(impl GetContext
              + GetGraphblasSparseMatrix
              + GetMatrixSize
              + GetValueTypeIdentifierRef),
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait IntoSparseMatrixForValueType<T: ValueType> {
    fn into_sparse_matrix(
        matrix: WeightedAdjacencyMatrix,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait IntoSparseMatrixAndClearValuesForValueType<T: ValueType> {
    fn into_sparse_matrix_and_clear_values(
        matrix: &mut WeightedAdjacencyMatrix,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

macro_rules! implement_to_sparse_matrix_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl ToSparseMatrixForValueType<$value_type> for $value_type {
            fn to_sparse_matrix(
                matrix: &(impl GetContext
                      + GetGraphblasSparseMatrix
                      + GetMatrixSize
                      + GetValueTypeIdentifierRef),
            ) -> Result<SparseMatrix<$value_type>, GraphComputingError> {
                match matrix.value_type_identifier_ref() {
                    &ValueTypeIdentifier::$value_type_identifier => unsafe {
                        Ok(SparseMatrix::<$value_type>::from_graphblas_matrix(
                            matrix.context(),
                            clone_graphblas_matrix(
                                matrix.context_ref(),
                                matrix.graphblas_matrix_ref(),
                            )?,
                        )?)
                    },
                    _ => {
                        let mut product_matrix =
                            SparseMatrix::<$value_type>::new(matrix.context(), matrix.size()?)?;

                        UnaryOperatorApplier::new().apply_to_matrix(
                            &Identity::<$value_type>::new(),
                            matrix,
                            &Assignment::<$value_type>::new(),
                            &mut product_matrix,
                            &SelectEntireMatrix::new(matrix.context()),
                            &OptionsForOperatorWithMatrixArgument::new_default(),
                        )?;

                        return Ok(product_matrix);
                    }
                }
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_to_sparse_matrix_for_value_type
);

macro_rules! implement_into_sparse_matrix_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl IntoSparseMatrixForValueType<$value_type> for $value_type {
            fn into_sparse_matrix(
                mut adjacency_matrix: WeightedAdjacencyMatrix,
            ) -> Result<SparseMatrix<$value_type>, GraphComputingError> {
                <$value_type>::into_sparse_matrix_and_clear_values(&mut adjacency_matrix)
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_into_sparse_matrix_for_value_type
);

macro_rules! implement_into_sparse_matrix_for_value_type {
    ($value_type_identifier:ident, $value_type:ty) => {
        impl IntoSparseMatrixAndClearValuesForValueType<$value_type> for $value_type {
            fn into_sparse_matrix_and_clear_values(
                adjacency_matrix: &mut WeightedAdjacencyMatrix,
            ) -> Result<SparseMatrix<$value_type>, GraphComputingError> {
                match adjacency_matrix.value_type_identifier_ref() {
                    &ValueTypeIdentifier::$value_type_identifier => unsafe {
                        let mut graphblas_matrix = new_graphblas_matrix(
                            &adjacency_matrix.graphblas_context,
                            adjacency_matrix.size()?,
                            <$value_type>::to_graphblas_type(),
                        )?;

                        mem::swap(&mut graphblas_matrix, &mut adjacency_matrix.sparse_matrix);

                        Ok(SparseMatrix::<$value_type>::from_graphblas_matrix(
                            adjacency_matrix.context(),
                            graphblas_matrix,
                        )?)
                    },
                    _ => {
                        let mut product_matrix = SparseMatrix::<$value_type>::new(
                            adjacency_matrix.context(),
                            adjacency_matrix.size()?,
                        )?;

                        UnaryOperatorApplier::new().apply_to_matrix(
                            &Identity::<$value_type>::new(),
                            adjacency_matrix,
                            &Assignment::<$value_type>::new(),
                            &mut product_matrix,
                            &SelectEntireMatrix::new(adjacency_matrix.context()),
                            &OptionsForOperatorWithMatrixArgument::new_default(),
                        )?;

                        return Ok(product_matrix);
                    }
                }
            }
        }
    };
}
implement_1_type_macro_with_enum_type_indentifier_for_all_value_types!(
    implement_into_sparse_matrix_for_value_type
);

pub(crate) trait CreateSparseMatrixForValueType<T: ValueType> {
    fn new_sparse_matrix(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

macro_rules! implement_create_sparse_matrix_for_value_type {
    ($value_type:ty) => {
        impl CreateSparseMatrixForValueType<$value_type> for $value_type {
            fn new_sparse_matrix(
                graphblas_context: Arc<GraphBLASContext>,
                initial_vertex_capacity: ElementCount,
            ) -> Result<SparseMatrix<$value_type>, GraphComputingError> {
                let size = (initial_vertex_capacity, initial_vertex_capacity).into();
                Ok(SparseMatrix::<$value_type>::new(graphblas_context, size)?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_create_sparse_matrix_for_value_type);

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::sparse_matrix_size;

    use super::*;

    #[test]
    fn new_adjacency_matrix() {
        let weighted_adjacency_matrix =
            <WeightedAdjacencyMatrix as CreateWeightedAdjacencyMatrix<f32>>::new(
                GraphBLASContext::init_default().unwrap(),
                10,
            )
            .unwrap();
        assert_eq!(
            sparse_matrix_size(&weighted_adjacency_matrix).unwrap(),
            Size::new(10, 10)
        );
    }
}
