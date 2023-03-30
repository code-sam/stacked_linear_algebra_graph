use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::graph::{VertexIndex, VertexTypeIndex};
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::indexer::{Indexer, IndexerTrait};
use crate::graph::value_type::NativeDataType as GraphNativeDataType;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::VertexDefinedByKeyTrait;
use crate::graph::vertex::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex::{
    VertexDefinedByIndex, VertexDefinedByIndexTrait, VertexDefinedByKey,
    VertexDefinedByTypeIndexAndVertexKey, VertexKeyRef,
};
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::VertexVectorTrait;

pub(crate) trait UpdateVertex<T: ValueType> {
    fn update_key_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByKey<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: &VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_index_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_index_defined_vertex_unchecked(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl UpdateVertex<$value_type> for VertexStore {
            fn update_key_defined_vertex(
                &mut self,
                vertex: &VertexDefinedByKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let vertex_index = *self
                    .element_indexer_ref()
                    .try_index_for_key(vertex.key_ref())?;
                self.vertex_vector_by_key_mut_ref(vertex.type_key_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((vertex_index, *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_vertex_defined_by_type_index_and_vertex_key(
                &mut self,
                vertex: &VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let vertex_index = *self
                    .element_indexer_ref()
                    .try_index_for_key(vertex.key_ref())?;
                self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((vertex_index, *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_index_defined_vertex(
                &mut self,
                vertex: &VertexDefinedByIndex<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.element_indexer_ref()
                    .try_index_validity(vertex.index_ref())?;
                self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_index_defined_vertex_unchecked(
                &mut self,
                vertex: &VertexDefinedByIndex<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.vertex_vector_by_index_mut_ref_unchecked(vertex.type_index_ref())
                    .sparse_vector_mut_ref()
                    .set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_set_vertex_data);
