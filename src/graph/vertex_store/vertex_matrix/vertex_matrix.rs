use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetMatrixElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetMatrixElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElement;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixRow;
use graphblas_sparse_linear_algebra::operators::extract::MatrixRowExtractor;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use once_cell::sync::Lazy;

use crate::error::LogicError;
use crate::error::LogicErrorType;
use crate::graph::graph::VertexIndex;
use crate::graph::graph::VertexTypeIndex;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::SparseVertexMatrixForValueType;
// use crate::graph::value_type::ValueTypeIndex;
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};
use crate::graph::vertex::vertex::VertexTypeKey;
use crate::graph::vertex::vertex::VertexTypeKeyRef;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::{error::GraphComputingError, graph::index::ElementCount};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static EXTRACT_MATRIX_ROW_OPERATOR: Lazy<MatrixRowExtractor> =
    Lazy::new(|| MatrixRowExtractor::new());

#[derive(Clone, Debug)]
pub struct VertexMatrixStore {
    graphblas_context: Arc<GraphBLASContext>,
    vertex_matrix_bool: SparseMatrix<bool>,
    vertex_matrix_i8: SparseMatrix<i8>,
    vertex_matrix_i16: SparseMatrix<i16>,
    vertex_matrix_i32: SparseMatrix<i32>,
    vertex_matrix_i64: SparseMatrix<i64>,
    vertex_matrix_u8: SparseMatrix<u8>,
    vertex_matrix_u16: SparseMatrix<u16>,
    vertex_matrix_u32: SparseMatrix<u32>,
    vertex_matrix_u64: SparseMatrix<u64>,
    vertex_matrix_f32: SparseMatrix<f32>,
    vertex_matrix_f64: SparseMatrix<f64>,
    vertex_matrix_isize: SparseMatrix<isize>,
    vertex_matrix_usize: SparseMatrix<usize>,
}

impl VertexMatrixStore {
    pub(crate) fn new(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_vertex_capacity: &ElementCount,
        initial_vertex_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let initial_size = Size::new(
            initial_vertex_type_capacity.to_owned(),
            initial_vertex_capacity.to_owned(),
        );
        Ok(Self {
            graphblas_context: graphblas_context.clone(),
            vertex_matrix_bool: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_i8: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_i16: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_i32: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_i64: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_u8: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_u16: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_u32: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_u64: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_f32: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_f64: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_isize: SparseMatrix::new(graphblas_context, &initial_size)?,
            vertex_matrix_usize: SparseMatrix::new(graphblas_context, &initial_size)?,
        })
    }
}

pub(crate) trait SparseVertexVector<T: ValueType> {
    fn sparse_vector_ref(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<&SparseVector<T>, GraphComputingError>;

    // REVIEW: mutating the cloned vector doesn't apply to the source vertex matrix.
    // fn sparse_vector_mut_ref(&mut self, vertex_type_index: &VertexTypeIndex) -> &mut SparseVector<T>;
}

macro_rules! implement_vertex_vector_trait {
    ($typed_sparse_matrix:ident, $value_type: ty) => {
        impl SparseVertexVector<$value_type> for VertexMatrixStore {
            fn sparse_vector_ref(
                &self,
                vertex_type_index: &VertexTypeIndex,
            ) -> Result<&SparseVector<$value_type>, GraphComputingError> {
                let mut vertex_vector = SparseVector::<$value_type>::new(
                    &self.graphblas_context_ref(),
                    &self.vertex_capacity()?,
                )?;

                // TODO: cache the accumulator for better performance
                let accumulator = Assignment::<$value_type>::new();

                EXTRACT_MATRIX_ROW_OPERATOR.apply(
                    &self.$typed_sparse_matrix,
                    vertex_type_index,
                    &ElementIndexSelector::All,
                    &accumulator,
                    &mut vertex_vector,
                    &SelectEntireVector::new(&self.graphblas_context_ref()), // TODO: cache this for better performance
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;

                Ok(&vertex_vector)
            }
        }
    };
}

implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_vertex_vector_trait,
    vertex_matrix
);

pub(crate) trait SparseVertexMatrix<T: ValueType> {
    fn sparse_matrix_ref(&self) -> &SparseMatrix<T>;
    fn sparse_matrix_mut_ref(&mut self) -> &mut SparseMatrix<T>;
}

macro_rules! implement_vertex_matrix_trait {
    ($typed_sparse_matrix:ident, $value_type: ty) => {
        impl SparseVertexMatrix<$value_type> for VertexMatrixStore {
            fn sparse_matrix_ref(&self) -> &SparseMatrix<$value_type> {
                &self.$typed_sparse_matrix
            }
            fn sparse_matrix_mut_ref(&mut self) -> &mut SparseMatrix<$value_type> {
                &mut self.$typed_sparse_matrix
            }
        }
    };
}

implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_vertex_matrix_trait,
    sparse_matrix
);

pub(crate) trait VertexMatrixTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext>;

    fn size(&self) -> Result<Size, GraphComputingError>;
    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_type_capacity(&self) -> Result<ElementCount, GraphComputingError>;

    // TODO: this probably needs a lifetime, or a clone
    // pub fn size_ref(&self) -> Result<&Size, GraphComputingError>;

    fn resize(&mut self, new_size: Size) -> Result<(), GraphComputingError>;
    fn set_vertex_capacity(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
    fn set_vertex_type_capacity(
        &mut self,
        new_vertex_type_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl VertexMatrixTrait for VertexMatrixStore {
    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }

    fn size(&self) -> Result<Size, GraphComputingError> {
        Ok(self.vertex_matrix_bool.size()?)
    }

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.vertex_matrix_bool.column_width()?)
    }

    fn vertex_type_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.vertex_matrix_bool.row_height()?)
    }

    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn resize(&mut self, new_size: Size) -> Result<(), GraphComputingError> {
        self.vertex_matrix_bool.resize(&new_size)?;
        self.vertex_matrix_i8.resize(&new_size)?;
        self.vertex_matrix_i16.resize(&new_size)?;
        self.vertex_matrix_i32.resize(&new_size)?;
        self.vertex_matrix_i64.resize(&new_size)?;
        self.vertex_matrix_u8.resize(&new_size)?;
        self.vertex_matrix_u16.resize(&new_size)?;
        self.vertex_matrix_u32.resize(&new_size)?;
        self.vertex_matrix_u64.resize(&new_size)?;
        self.vertex_matrix_f32.resize(&new_size)?;
        self.vertex_matrix_f64.resize(&new_size)?;
        self.vertex_matrix_isize.resize(&new_size)?;
        self.vertex_matrix_usize.resize(&new_size)?;
        Ok(())
    }

    fn set_vertex_capacity(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        let new_size = Size::new(self.vertex_type_capacity()?, new_vertex_capacity);
        self.vertex_matrix_bool.resize(&new_size)?;
        self.vertex_matrix_i8.resize(&new_size)?;
        self.vertex_matrix_i16.resize(&new_size)?;
        self.vertex_matrix_i32.resize(&new_size)?;
        self.vertex_matrix_i64.resize(&new_size)?;
        self.vertex_matrix_u8.resize(&new_size)?;
        self.vertex_matrix_u16.resize(&new_size)?;
        self.vertex_matrix_u32.resize(&new_size)?;
        self.vertex_matrix_u64.resize(&new_size)?;
        self.vertex_matrix_f32.resize(&new_size)?;
        self.vertex_matrix_f64.resize(&new_size)?;
        self.vertex_matrix_isize.resize(&new_size)?;
        self.vertex_matrix_usize.resize(&new_size)?;
        Ok(())
    }

    fn set_vertex_type_capacity(
        &mut self,
        new_vertex_type_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        let new_size = Size::new(new_vertex_type_capacity, self.vertex_capacity()?);
        self.vertex_matrix_bool.resize(&new_size)?;
        self.vertex_matrix_i8.resize(&new_size)?;
        self.vertex_matrix_i16.resize(&new_size)?;
        self.vertex_matrix_i32.resize(&new_size)?;
        self.vertex_matrix_i64.resize(&new_size)?;
        self.vertex_matrix_u8.resize(&new_size)?;
        self.vertex_matrix_u16.resize(&new_size)?;
        self.vertex_matrix_u32.resize(&new_size)?;
        self.vertex_matrix_u64.resize(&new_size)?;
        self.vertex_matrix_f32.resize(&new_size)?;
        self.vertex_matrix_f64.resize(&new_size)?;
        self.vertex_matrix_isize.resize(&new_size)?;
        self.vertex_matrix_usize.resize(&new_size)?;
        Ok(())
    }
}

pub(crate) trait SetVertexMatrixValue<T: ValueType> {
    fn set_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexMatrixForValueType<T> + SetMatrixElementTyped<T>>
    SetVertexMatrixValue<T> for VertexMatrixStore
{
    fn set_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError> {
        let element = MatrixElement::new(
            Coordinate::new(*vertex_type_index, *vertex_index),
            vertex_value,
        );
        Ok(T::sparse_matrix_mut_ref(self).set_element(element)?)
    }
}

pub(crate) trait ReadVertexValueInVertexMatrix<T: ValueType> {
    fn get_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn get_vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<
        T: ValueType + SparseVertexMatrixForValueType<T> + GetMatrixElementValueTyped<T> + Default,
    > ReadVertexValueInVertexMatrix<T> for VertexMatrixStore
{
    fn get_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        Ok(T::sparse_matrix_ref(self)
            .get_element_value(&Coordinate::new(*vertex_type_index, *vertex_index))?)
    }

    fn try_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        match self.get_vertex_value(vertex_type_index, vertex_index)? {
            Some(value) => Ok(value),
            None => Err(LogicError::new(
                LogicErrorType::VertexMustExist,
                format!(
                    "No vertex for vertex index: {}, and vertex type index: {}",
                    vertex_index, vertex_type_index
                ),
                None,
            )
            .into()),
        }
    }

    fn get_vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        Ok(T::get_element_value_or_default(
            T::sparse_matrix_ref(self),
            &Coordinate::new(*vertex_type_index, *vertex_index),
        )?)
    }
}

pub(crate) trait DeleteVertexValueInVertexMatrix<T: ValueType + SparseVertexMatrixForValueType<T>> {
    fn delete_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexMatrixForValueType<T>> DeleteVertexValueInVertexMatrix<T>
    for VertexMatrixStore
{
    fn delete_vertex_value(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(T::sparse_matrix_mut_ref(self)
            .drop_element(Coordinate::new(*vertex_type_index, *vertex_index))?)
    }
}

pub(crate) trait IsElementInVertexMatrix<T: ValueType> {
    fn is_vertex_element(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexMatrixForValueType<T>> IsElementInVertexMatrix<T>
    for VertexMatrixStore
{
    fn is_vertex_element(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(T::sparse_matrix_ref(self)
            .is_element(Coordinate::new(*vertex_type_index, *vertex_index))?)
    }

    fn try_is_vertex_element(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(T::sparse_matrix_ref(self)
            .try_is_element(Coordinate::new(*vertex_type_index, *vertex_index))?)
    }
}
