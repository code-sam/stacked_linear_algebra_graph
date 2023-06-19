use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

use crate::graph::graph::VertexIndex;
// use crate::graph::value_type::ValueTypeIndex;
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};
use crate::graph::vertex::{VertexTypeKey, VertexTypeKeyRef};
use crate::{error::GraphComputingError, graph::index::ElementCount};

#[derive(Clone, Debug)]
pub struct VertexVector {
    vertex_type: VertexTypeKey,
    graphblas_context: Arc<GraphBLASContext>,
    sparse_vector_bool: SparseVector<bool>,
    sparse_vector_i8: SparseVector<i8>,
    sparse_vector_i16: SparseVector<i16>,
    sparse_vector_i32: SparseVector<i32>,
    sparse_vector_i64: SparseVector<i64>,
    sparse_vector_u8: SparseVector<u8>,
    sparse_vector_u16: SparseVector<u16>,
    sparse_vector_u32: SparseVector<u32>,
    sparse_vector_u64: SparseVector<u64>,
    sparse_vector_f32: SparseVector<f32>,
    sparse_vector_f64: SparseVector<f64>,
    sparse_vector_isize: SparseVector<isize>,
    sparse_vector_usize: SparseVector<usize>,
    // pub(crate) sparse_vector_bool: SparseVector<bool>,
    // pub(crate) sparse_vector_i8: SparseVector<i8>,
    // pub(crate) sparse_vector_i16: SparseVector<i16>,
    // pub(crate) sparse_vector_i32: SparseVector<i32>,
    // pub(crate) sparse_vector_i64: SparseVector<i64>,
    // pub(crate) sparse_vector_u8: SparseVector<u8>,
    // pub(crate) sparse_vector_u16: SparseVector<u16>,
    // pub(crate) sparse_vector_u32: SparseVector<u32>,
    // pub(crate) sparse_vector_u64: SparseVector<u64>,
    // pub(crate) sparse_vector_f32: SparseVector<f32>,
    // pub(crate) sparse_vector_f64: SparseVector<f64>,
    // pub(crate) sparse_vector_isize: SparseVector<isize>,
    // pub(crate) sparse_vector_usize: SparseVector<usize>,
}

impl VertexVector {
    pub(crate) fn new(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_vertex_capacity: &ElementCount,
        vertex_type: &VertexTypeKeyRef,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            vertex_type: vertex_type.to_owned(),
            graphblas_context: graphblas_context.clone(),
            sparse_vector_bool: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i8: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i16: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u8: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u16: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_f32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_f64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_isize: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_usize: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
        })
    }
}

// pub(crate) trait SparseVertexVectorDynamicDispatch<T: ValueType> {
//     fn sparse_vector_ref(&self) -> &dyn GraphblasSparseVectorTrait;
//     fn sparse_vector_mut_ref(&mut self) -> &mut dyn GraphblasSparseVectorTrait;
// }

// impl<T: ValueType> SparseVertexVectorDynamicDispatch<T> for VertexVector {
//     fn sparse_vector_ref(&self) -> &dyn GraphblasSparseVectorTrait {
//         match T::value_type_enum() {
//             ValueTypeIndex::Boolean => &self.sparse_vector_bool,
//             ValueTypeIndex::Integer8Bit => &self.sparse_vector_i8,
//             ValueTypeIndex::Integer16Bit => &self.sparse_vector_i16,
//             ValueTypeIndex::Integer32Bit => &self.sparse_vector_i32,
//             ValueTypeIndex::Integer64Bit => &self.sparse_vector_i64,
//             ValueTypeIndex::UnsignedInteger8Bit => &self.sparse_vector_u8,
//             ValueTypeIndex::UnsignedInteger16Bit => &self.sparse_vector_u16,
//             ValueTypeIndex::UnsignedInteger32Bit => &self.sparse_vector_u32,
//             ValueTypeIndex::UnsignedInteger64Bit => &self.sparse_vector_u64,
//             ValueTypeIndex::FloatingPoint32Bit => &self.sparse_vector_u32,
//             ValueTypeIndex::FloatingPoint64Bit => &self.sparse_vector_u64,
//             ValueTypeIndex::PointerSizedInteger => &self.sparse_vector_isize,
//             ValueTypeIndex::PointerSizedUnsizedInteger => &self.sparse_vector_usize,
//         }
//     }
//     fn sparse_vector_mut_ref(&mut self) -> &mut dyn GraphblasSparseVectorTrait {
//         match T::value_type_enum() {
//             ValueTypeIndex::Boolean => &mut self.sparse_vector_bool,
//             ValueTypeIndex::Integer8Bit => &mut self.sparse_vector_i8,
//             ValueTypeIndex::Integer16Bit => &mut self.sparse_vector_i16,
//             ValueTypeIndex::Integer32Bit => &mut self.sparse_vector_i32,
//             ValueTypeIndex::Integer64Bit => &mut self.sparse_vector_i64,
//             ValueTypeIndex::UnsignedInteger8Bit => &mut self.sparse_vector_u8,
//             ValueTypeIndex::UnsignedInteger16Bit => &mut self.sparse_vector_u16,
//             ValueTypeIndex::UnsignedInteger32Bit => &mut self.sparse_vector_u32,
//             ValueTypeIndex::UnsignedInteger64Bit => &mut self.sparse_vector_u64,
//             ValueTypeIndex::FloatingPoint32Bit => &mut self.sparse_vector_u32,
//             ValueTypeIndex::FloatingPoint64Bit => &mut self.sparse_vector_u64,
//             ValueTypeIndex::PointerSizedInteger => &mut self.sparse_vector_isize,
//             ValueTypeIndex::PointerSizedUnsizedInteger => &mut self.sparse_vector_usize,
//         }
//     }
// }

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
    sparse_vector
);

pub(crate) trait VertexVectorTrait {
    fn vertex_type_ref(&self) -> &VertexTypeKeyRef;
    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext>;

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;

    // TODO: this probably needs a lifetime, or a clone
    // pub fn size_ref(&self) -> Result<&Size, GraphComputingError>;

    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
    fn length(&self) -> Result<ElementCount, GraphComputingError>;

    fn delete_element_for_all_value_types(
        &mut self,
        element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl VertexVectorTrait for VertexVector {
    fn vertex_type_ref(&self) -> &VertexTypeKeyRef {
        &self.vertex_type.as_str()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context
    }

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
        Ok(self.sparse_vector_bool.length()?)
    }

    // TODO: find a more generic solution, e.g. by using TAITs as soon as they are stable
    // https://github.com/rust-lang/rust/issues/63063
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError> {
        self.sparse_vector_bool.resize(new_vertex_capacity)?;
        self.sparse_vector_i8.resize(new_vertex_capacity)?;
        self.sparse_vector_i16.resize(new_vertex_capacity)?;
        self.sparse_vector_i32.resize(new_vertex_capacity)?;
        self.sparse_vector_i64.resize(new_vertex_capacity)?;
        self.sparse_vector_u8.resize(new_vertex_capacity)?;
        self.sparse_vector_u16.resize(new_vertex_capacity)?;
        self.sparse_vector_u32.resize(new_vertex_capacity)?;
        self.sparse_vector_u64.resize(new_vertex_capacity)?;
        self.sparse_vector_f32.resize(new_vertex_capacity)?;
        self.sparse_vector_f64.resize(new_vertex_capacity)?;
        self.sparse_vector_isize.resize(new_vertex_capacity)?;
        self.sparse_vector_usize.resize(new_vertex_capacity)?;
        Ok(())
    }

    fn length(&self) -> Result<ElementCount, GraphComputingError> {
        // Size is the same for all types
        Ok(self.sparse_vector_bool.length()?)
    }

    fn delete_element_for_all_value_types(
        &mut self,
        element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.sparse_vector_bool.drop_element(*element_index)?;
        self.sparse_vector_i8.drop_element(*element_index)?;
        self.sparse_vector_i16.drop_element(*element_index)?;
        self.sparse_vector_i32.drop_element(*element_index)?;
        self.sparse_vector_i64.drop_element(*element_index)?;
        self.sparse_vector_u8.drop_element(*element_index)?;
        self.sparse_vector_u16.drop_element(*element_index)?;
        self.sparse_vector_u32.drop_element(*element_index)?;
        self.sparse_vector_u64.drop_element(*element_index)?;
        self.sparse_vector_f32.drop_element(*element_index)?;
        self.sparse_vector_f64.drop_element(*element_index)?;
        self.sparse_vector_isize.drop_element(*element_index)?;
        self.sparse_vector_usize.drop_element(*element_index)?;
        Ok(())
    }
}
