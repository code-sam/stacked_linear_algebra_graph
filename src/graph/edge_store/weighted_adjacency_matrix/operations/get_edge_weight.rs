use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueUntyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    GetCoordinateIndices, GetGraphblasSparseMatrix,
};

use crate::error::LogicErrorType;
use crate::error::{GraphComputingError, LogicError};
use crate::graph::edge_store::weighted_adjacency_matrix::{
    GetAdjacencyMatrixCoordinateIndices, WeightedAdjacencyMatrix,
};

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::{
    GetValueTypeIdentifierRef, IntoValueType, ValueType, ValueTypeIdentifier,
};

pub(crate) trait GetEdgeWeight<T> {
    fn edge_weight_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;
    fn edge_weight_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_or_default_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight_unchecked(
        &self,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError>;
    fn try_edge_weight_at_coordinate_unchecked(
        &self,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError>;
}

impl<T> GetEdgeWeight<T> for WeightedAdjacencyMatrix
where
    T: ValueType + Default,
    bool: IntoValueType<T>,
    i8: IntoValueType<T>,
    i16: IntoValueType<T>,
    i32: IntoValueType<T>,
    i64: IntoValueType<T>,
    u8: IntoValueType<T>,
    u16: IntoValueType<T>,
    u32: IntoValueType<T>,
    u64: IntoValueType<T>,
    f32: IntoValueType<T>,
    f64: IntoValueType<T>,
    isize: IntoValueType<T>,
    usize: IntoValueType<T>,
{
    fn edge_weight_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        match self.value_type_identifier_ref() {
            &ValueTypeIdentifier::Bool => unsafe {
                // get_matrix_element_value::<bool, T>(self, tail, head)
                match unsafe { bool::element_value(self, tail.index(), head.index())? } {
                    Some(edge_weight) => Ok(Some(edge_weight.into_value_type())),
                    None => Ok(None),
                }
            },
            &ValueTypeIdentifier::Int8 => unsafe {
                get_matrix_element_value::<i8, T>(self, tail, head)
            },
            &ValueTypeIdentifier::Int16 => unsafe {
                get_matrix_element_value::<i16, T>(self, tail, head)
            },
            &ValueTypeIdentifier::Int32 => unsafe {
                get_matrix_element_value::<i32, T>(self, tail, head)
            },
            &ValueTypeIdentifier::Int64 => unsafe {
                get_matrix_element_value::<i64, T>(self, tail, head)
            },
            &ValueTypeIdentifier::UInt8 => unsafe {
                get_matrix_element_value::<u8, T>(self, tail, head)
            },
            &ValueTypeIdentifier::UInt16 => unsafe {
                get_matrix_element_value::<u16, T>(self, tail, head)
            },
            &ValueTypeIdentifier::UInt32 => unsafe {
                get_matrix_element_value::<u32, T>(self, tail, head)
            },
            &ValueTypeIdentifier::UInt64 => unsafe {
                get_matrix_element_value::<u64, T>(self, tail, head)
            },
            &ValueTypeIdentifier::Float32 => unsafe {
                get_matrix_element_value::<f32, T>(self, tail, head)
            },
            &ValueTypeIdentifier::Float64 => unsafe {
                get_matrix_element_value::<f64, T>(self, tail, head)
            },
            &ValueTypeIdentifier::ISize => unsafe {
                get_matrix_element_value::<isize, T>(self, tail, head)
            },
            &ValueTypeIdentifier::USize => unsafe {
                get_matrix_element_value::<usize, T>(self, tail, head)
            },
        }
    }

    fn edge_weight_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_weight_unchecked(coordinate.tail_ref(), coordinate.head_ref())
    }

    fn edge_weight_or_default_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        match self.edge_weight_unchecked(tail, head)? {
            Some(edge_weight) => Ok(edge_weight),
            None => Ok(T::default()),
        }
    }

    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        self.edge_weight_or_default_unchecked(coordinate.tail_ref(), coordinate.head_ref())
    }

    fn try_edge_weight_unchecked(
        &self,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        match self.edge_weight_unchecked(tail, head)? {
            Some(weight) => Ok(weight),
            None => Err(LogicError::new(
                LogicErrorType::EdgeMustExist,
                format!(
                    "No edge exists at coordinate: [tail: {:?}, head: {:?}]",
                    tail, head
                ),
                None,
            )
            .into()),
        }
    }

    fn try_edge_weight_at_coordinate_unchecked(
        &self,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError> {
        GetEdgeWeight::<T>::try_edge_weight_unchecked(
            self,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }
}

unsafe fn get_matrix_element_value<M, T>(
    matrix: &(impl GetGraphblasSparseMatrix + GetValueTypeIdentifierRef),
    tail_index: &impl GetVertexIndexIndex,
    head_index: &impl GetVertexIndexIndex,
) -> Result<Option<T>, GraphComputingError>
where
    M: ValueType + Default + GetSparseMatrixElementValueUntyped<M> + IntoValueType<T>,
    // M: ValueType + Default + GetSparseMatrixElementValueUntyped<M>,
    T: ValueType,
{
    match unsafe { M::element_value(matrix, tail_index.index(), head_index.index())? } {
        Some(edge_weight) => Ok(Some(edge_weight.into_value_type())),
        None => Ok(None),
    }
}

// pub trait GetSparseMatrixElementValue<T: ValueType + Default> {
//     unsafe fn element_value(
//         matrix: &(impl GetGraphblasSparseMatrix + GetContext),
//         row_index: RowIndex,
//         column_index: ColumnIndex,
//     ) -> Result<Option<T>, SparseLinearAlgebraError>;
//     unsafe fn element_value_or_default(
//         matrix: &(impl GetGraphblasSparseMatrix + GetContext),
//         row_index: RowIndex,
//         column_index: ColumnIndex,
//     ) -> Result<T, SparseLinearAlgebraError>;

//     unsafe fn element_value_at_coordinate(
//         matrix: &(impl GetGraphblasSparseMatrix + GetContext),
//         coordinate: &impl GetCoordinateIndices,
//     ) -> Result<Option<T>, SparseLinearAlgebraError>;
//     unsafe fn element_value_or_default_at_coordinate(
//         matrix: &(impl GetGraphblasSparseMatrix + GetContext),
//         coordinate: &impl GetCoordinateIndices,
//     ) -> Result<T, SparseLinearAlgebraError>;
// }

// impl<T: > GetSparseMatrixElementValue<T> for T {

// }

// fn adjacency_matrix_element_value<T: ValueType>(
//     matrix: &(impl GetGraphblasSparseMatrix + GetValueTypeIdentifierRef),
//     tail: &impl GetVertexIndexIndex,
//     head: &impl GetVertexIndexIndex,
// ) -> Result<Option<T>, SparseLinearAlgebraError> {
//     match matrix.value_type_identifier_ref() {
//         &ValueTypeIdentifier::Bool => unsafe {
//             // get_matrix_element_value::<bool, T>(matrix, tail, head)
//             match unsafe { bool::element_value(matrix, tail.index(), head.index())? } {
//                 Some(edge_weight) => Ok(Some(edge_weight.into_value_type())),
//                 None => Ok(None),
//             }
//         },
//         &ValueTypeIdentifier::Int8 => unsafe {
//             get_matrix_element_value::<i8, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::Int16 => unsafe {
//             get_matrix_element_value::<i16, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::Int32 => unsafe {
//             get_matrix_element_value::<i32, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::Int64 => unsafe {
//             get_matrix_element_value::<i64, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::UInt8 => unsafe {
//             get_matrix_element_value::<u8, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::UInt16 => unsafe {
//             get_matrix_element_value::<u16, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::UInt32 => unsafe {
//             get_matrix_element_value::<u32, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::UInt64 => unsafe {
//             get_matrix_element_value::<u64, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::Float32 => unsafe {
//             get_matrix_element_value::<f32, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::Float64 => unsafe {
//             get_matrix_element_value::<f64, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::ISize => unsafe {
//             get_matrix_element_value::<isize, T>(self, tail, head)
//         },
//         &ValueTypeIdentifier::USize => unsafe {
//             get_matrix_element_value::<usize, T>(self, tail, head)
//         },
//     }
// }

// fn element_value<T: ValueType>(
//     matrix: &(impl GetGraphblasSparseMatrix + GetContext + GetValueTypeIdentifierRef),
//     row_index: RowIndex,
//     column_index: ColumnIndex,
// ) -> Result<Option<T>, SparseLinearAlgebraError> {
//     let element_value;
//     let row_index_to_get = row_index.as_graphblas_index()?;
//     let column_index_to_get = column_index.as_graphblas_index()?;

//     match matrix.value_type_identifier_ref() {
//         &ValueTypeIdentifier::Bool => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_BOOL(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Int8 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_INT8(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Int16 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_INT16(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Int32 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_INT32(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Int64 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_INT64(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::UInt8 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_UINT8(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::UInt16 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_UINT16(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::UInt32 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_UINT32(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::UInt64 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_UINT64(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Float32 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_FP32(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::Float64 => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_FP64(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::ISize => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_INT64(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//         &ValueTypeIdentifier::USize => unsafe {
//             let mut value = MaybeUninit::uninit();

//             let result = matrix.context_ref().call(
//                 || unsafe {
//                     GrB_Matrix_extractElement_UINT64(
//                         value.as_mut_ptr(),
//                         matrix.graphblas_matrix(),
//                         row_index_to_get,
//                         column_index_to_get,
//                     )
//                 },
//                 unsafe { &matrix.graphblas_matrix() },
//             );

//             match result {
//                 Ok(_) => {
//                     let value = unsafe { value.assume_init() };
//                     // Casting to support isize and usize, redundant for other types. TODO: review performance improvements
//                     Ok(Some(value.into_value_type()))
//                 }
//                 Err(error) => match error.error_type() {
//                     SparseLinearAlgebraErrorType::LogicErrorType(
//                         LogicErrorType::GraphBlas(GraphblasErrorType::NoValue),
//                     ) => Ok(None),
//                     _ => Err(error),
//                 },
//             }
//         },
//     }
// }
