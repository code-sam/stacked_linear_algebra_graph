use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetMatrixElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetMatrixElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElement;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElement;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::VectorElement;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

use graphblas_sparse_linear_algebra::operators::extract::MatrixRowExtractor;

use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use once_cell::sync::Lazy;

use crate::error::LogicError;
use crate::error::LogicErrorType;
use crate::graph::graph::VertexIndex;

use crate::graph::value_type::SparseVertexVectorForValueType;
// use crate::graph::value_type::ValueTypeIndex;
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};

use crate::{error::GraphComputingError, graph::index::ElementCount};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static EXTRACT_MATRIX_ROW_OPERATOR: Lazy<MatrixRowExtractor> =
    Lazy::new(|| MatrixRowExtractor::new());

#[derive(Clone, Debug)]
pub struct VertexVector {
    graphblas_context: Arc<GraphBLASContext>,
    vertex_vector_bool: SparseVector<bool>,
    vertex_vector_i8: SparseVector<i8>,
    vertex_vector_i16: SparseVector<i16>,
    vertex_vector_i32: SparseVector<i32>,
    vertex_vector_i64: SparseVector<i64>,
    vertex_vector_u8: SparseVector<u8>,
    vertex_vector_u16: SparseVector<u16>,
    vertex_vector_u32: SparseVector<u32>,
    vertex_vector_u64: SparseVector<u64>,
    vertex_vector_f32: SparseVector<f32>,
    vertex_vector_f64: SparseVector<f64>,
    vertex_vector_isize: SparseVector<isize>,
    vertex_vector_usize: SparseVector<usize>,
}

impl VertexVector {
    pub(crate) fn new(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_vertex_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            graphblas_context: graphblas_context.clone(),
            vertex_vector_bool: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_i8: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_i16: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_i32: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_i64: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_u8: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_u16: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_u32: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_u64: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_f32: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_f64: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_isize: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
            vertex_vector_usize: SparseVector::new(graphblas_context, &initial_vertex_capacity)?,
        })
    }

    // pub(crate) fn map_mut_all_value_types<F>(
    //     &mut self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&mut SparseVector<T: ValueType>) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     self.vertex_vectors
    //         .as_mut_slice()
    //         .into_par_iter()
    //         .try_for_each(function_to_apply)?;
    //     Ok(())
    // }
}

// pub(crate) trait SparseVertexVector<T: ValueType> {
//     fn extract_sparse_vector(
//         &self,
//         vertex_type_index: &VertexTypeIndex,
//     ) -> Result<SparseVector<T>, GraphComputingError>;

//     // REVIEW: mutating the cloned vector doesn't apply to the source vertex matrix.
//     // fn sparse_vector_mut_ref(&mut self, vertex_type_index: &VertexTypeIndex) -> &mut SparseVector<T>;
// }

// TODO: implement type-generically
// macro_rules! implement_vertex_vector_trait {
//     ($typed_sparse_matrix:ident, $value_type: ty) => {
//         impl SparseVertexVector<$value_type> for VertexMatrix {
//             fn extract_sparse_vector(
//                 &self,
//                 vertex_type_index: &VertexTypeIndex,
//             ) -> Result<SparseVector<$value_type>, GraphComputingError> {
//                 let mut vertex_vector = SparseVector::<$value_type>::new(
//                     &self.graphblas_context_ref(),
//                     &self.vertex_capacity()?,
//                 )?;

//                 // TODO: cache the accumulator for better performance
//                 let accumulator = Assignment::<$value_type>::new();

//                 EXTRACT_MATRIX_ROW_OPERATOR.apply(
//                     &self.$typed_sparse_matrix,
//                     vertex_type_index,
//                     &ElementIndexSelector::All,
//                     &accumulator,
//                     &mut vertex_vector,
//                     &SelectEntireVector::new(&self.graphblas_context_ref()), // TODO: cache this for better performance
//                     &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
//                 )?;

//                 Ok(vertex_vector)
//             }
//         }
//     };
// }

// implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
//     implement_vertex_vector_trait,
//     vertex_matrix
// );

pub(crate) trait SparseVertexVector<T: ValueType> {
    fn sparse_vector_ref(&self) -> &SparseVector<T>;
    fn sparse_vector_mut_ref(&mut self) -> &mut SparseVector<T>;
}

macro_rules! implement_vertex_vector_trait {
    ($typed_sparse_vector:ident, $value_type: ty) => {
        impl SparseVertexVector<$value_type> for VertexVector {
            fn sparse_vector_ref(&self) -> &SparseVector<$value_type> {
                &self.$typed_sparse_vector
            }
            fn sparse_vector_mut_ref(&mut self) -> &mut SparseVector<$value_type> {
                &mut self.$typed_sparse_vector
            }
        }
    };
}

implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_vertex_vector_trait,
    vertex_vector
);

pub(crate) trait VertexVectorTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext>;

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;

    fn set_vertex_capacity(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl VertexVectorTrait for VertexVector {
    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.vertex_vector_bool.length()?)
    }

    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn set_vertex_capacity(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.vertex_vector_bool.resize(new_vertex_capacity)?;
        self.vertex_vector_i8.resize(new_vertex_capacity)?;
        self.vertex_vector_i16.resize(new_vertex_capacity)?;
        self.vertex_vector_i32.resize(new_vertex_capacity)?;
        self.vertex_vector_i64.resize(new_vertex_capacity)?;
        self.vertex_vector_u8.resize(new_vertex_capacity)?;
        self.vertex_vector_u16.resize(new_vertex_capacity)?;
        self.vertex_vector_u32.resize(new_vertex_capacity)?;
        self.vertex_vector_u64.resize(new_vertex_capacity)?;
        self.vertex_vector_f32.resize(new_vertex_capacity)?;
        self.vertex_vector_f64.resize(new_vertex_capacity)?;
        self.vertex_vector_isize.resize(new_vertex_capacity)?;
        self.vertex_vector_usize.resize(new_vertex_capacity)?;
        Ok(())
    }
}

pub(crate) trait SetVertexVectorValue<T: ValueType> {
    fn set_vertex_value(
        &mut self,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexVectorForValueType<T> + SetVectorElementTyped<T>>
    SetVertexVectorValue<T> for VertexVector
{
    fn set_vertex_value(
        &mut self,
        vertex_index: &VertexIndex,
        vertex_value: T,
    ) -> Result<(), GraphComputingError> {
        let element = VectorElement::new(*vertex_index, vertex_value);
        Ok(T::sparse_vector_mut_ref(self).set_element(element)?)
    }
}

pub(crate) trait ReadVertexValueInVertexVector<T: ValueType> {
    fn get_vertex_value(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value(&self, vertex_index: &VertexIndex) -> Result<T, GraphComputingError>;

    fn get_vertex_value_or_default(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<
        T: ValueType + SparseVertexVectorForValueType<T> + GetVectorElementValueTyped<T> + Default,
    > ReadVertexValueInVertexVector<T> for VertexVector
{
    fn get_vertex_value(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        Ok(T::sparse_vector_ref(self).get_element_value(vertex_index)?)
    }

    fn try_vertex_value(&self, vertex_index: &VertexIndex) -> Result<T, GraphComputingError> {
        match self.get_vertex_value(vertex_index)? {
            Some(value) => Ok(value),
            None => Err(LogicError::new(
                LogicErrorType::VertexMustExist,
                format!("No vertex for vertex index: {}", vertex_index),
                None,
            )
            .into()),
        }
    }

    fn get_vertex_value_or_default(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        Ok(T::get_element_value_or_default(
            T::sparse_vector_ref(self),
            vertex_index,
        )?)
    }
}

pub(crate) trait DeleteVertexValueInVertexVector {
    fn delete_vertex_value_for_all_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexValueInVertexVector for VertexVector {
    fn delete_vertex_value_for_all_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_vector_bool.drop_element(*vertex_index)?;
        self.vertex_vector_i8.drop_element(*vertex_index)?;
        self.vertex_vector_i16.drop_element(*vertex_index)?;
        self.vertex_vector_i32.drop_element(*vertex_index)?;
        self.vertex_vector_i64.drop_element(*vertex_index)?;
        self.vertex_vector_u8.drop_element(*vertex_index)?;
        self.vertex_vector_u16.drop_element(*vertex_index)?;
        self.vertex_vector_u32.drop_element(*vertex_index)?;
        self.vertex_vector_u64.drop_element(*vertex_index)?;
        self.vertex_vector_f32.drop_element(*vertex_index)?;
        self.vertex_vector_f64.drop_element(*vertex_index)?;
        self.vertex_vector_isize.drop_element(*vertex_index)?;
        self.vertex_vector_usize.drop_element(*vertex_index)?;
        Ok(())
    }
}

pub(crate) trait DeleteVertexValueInVertexVectorTyped<
    T: ValueType + SparseVertexVectorForValueType<T>,
>
{
    fn delete_vertex_value(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexVectorForValueType<T>> DeleteVertexValueInVertexVectorTyped<T>
    for VertexVector
{
    fn delete_vertex_value(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(T::sparse_vector_mut_ref(self).drop_element(*vertex_index)?)
    }
}

pub(crate) trait IsElementInVertexVector<T: ValueType> {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexVectorForValueType<T>> IsElementInVertexVector<T> for VertexVector {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError> {
        Ok(T::sparse_vector_ref(self).is_element(*vertex_index)?)
    }

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError> {
        Ok(T::sparse_vector_ref(self).try_is_element(*vertex_index)?)
    }
}
