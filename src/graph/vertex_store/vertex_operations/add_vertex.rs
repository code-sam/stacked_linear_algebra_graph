use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::indexer::{Indexer, IndexerTrait};
use crate::graph::indexer::{NewIndex as NewVertexIndex, NewIndexTrait};
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types,
    NativeDataType as GraphNativeDataType,
};
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex::{
    VertexDefinedByIndex, VertexDefinedByIndexTrait, VertexDefinedByKey, VertexDefinedByKeyTrait,
    VertexKeyRef,
};
use crate::graph::vertex::{
    VertexDefinedByTypeIndexAndVertexKey, VertexDefinedByTypeKeyAndVertexIndexTrait,
};
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::vertex_vector::SparseVertexVector;
use crate::graph::vertex_store::VertexVectorTrait;

pub(crate) trait AddVertex<T: ValueType> {
    fn add_new_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<NewVertexIndex, GraphComputingError>;

    fn add_new_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<NewVertexIndex, GraphComputingError>;

    fn add_or_replace_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<NewVertexIndex, GraphComputingError>;

    fn add_or_replace_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<NewVertexIndex, GraphComputingError>;

    fn add_or_update_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<Option<NewVertexIndex>, GraphComputingError>;

    fn add_or_update_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<NewVertexIndex>, GraphComputingError>;
}

// TODO: review expansion of vertex matrix
macro_rules! implement_add_vertex_element {
    ($vertex_vector_mut_ref:ident, $value_type:ty) => {
        impl AddVertex<$value_type> for VertexStore {
            fn add_new_key_defined_vertex(
                &mut self,
                vertex: VertexDefinedByKey<$value_type>,
            ) -> Result<NewVertexIndex, GraphComputingError> {
                let type_index = *self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex.type_key_ref())?;
                let vertex_index = self
                    .element_indexer_mut_ref()
                    .add_new_key(vertex.key_ref())?;
                self.vertex_vector_by_index_mut_ref(&type_index)?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                Ok(vertex_index)
            }

            fn add_new_vertex_with_type_index_and_vertex_key(
                &mut self,
                vertex: VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<NewVertexIndex, GraphComputingError> {
                let vertex_index = self
                    .element_indexer_mut_ref()
                    .add_new_key(vertex.key_ref())?;
                self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                Ok(vertex_index)
            }

            fn add_or_replace_key_defined_vertex(
                &mut self,
                vertex: VertexDefinedByKey<$value_type>,
            ) -> Result<NewVertexIndex, GraphComputingError> {
                let type_index = *self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex.type_key_ref())?;
                let vertex_index = self
                    .element_indexer_mut_ref()
                    .add_or_replace_key(vertex.key_ref())?; // TODO
                self.vertex_vector_by_index_mut_ref(&type_index)?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                Ok(vertex_index)
            }

            fn add_or_replace_vertex_with_type_index_and_vertex_key(
                &mut self,
                vertex: VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<NewVertexIndex, GraphComputingError> {
                let vertex_index = self
                    .element_indexer_mut_ref()
                    .add_or_replace_key(vertex.key_ref())?; // TODO
                self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                Ok(vertex_index)
            }

            fn add_or_update_key_defined_vertex(
                &mut self,
                vertex: VertexDefinedByKey<$value_type>,
            ) -> Result<Option<NewVertexIndex>, GraphComputingError> {
                let type_index = *self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex.type_key_ref())?;
                // TODO: do not clone self.element_indexer_ref()
                match self
                    .element_indexer_ref()
                    .clone()
                    .index_for_key(vertex.key_ref())
                {
                    Some(index_ref) => {
                        self.vertex_vector_by_index_mut_ref(&type_index)?
                            .sparse_vector_mut_ref()
                            .set_element((*index_ref, *vertex.value_ref()).into())?;
                        Ok(None)
                    }
                    None => {
                        let vertex_index = self
                            .element_indexer_mut_ref()
                            .add_new_key(vertex.key_ref())?;
                        self.vertex_vector_by_index_mut_ref(&type_index)?
                            .sparse_vector_mut_ref()
                            .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                        Ok(Some(vertex_index))
                    }
                }
            }

            fn add_or_update_vertex_with_type_index_and_vertex_key(
                &mut self,
                vertex: VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<Option<NewVertexIndex>, GraphComputingError> {
                // TODO: do not clone self.element_indexer_ref()
                match self
                    .element_indexer_ref()
                    .clone()
                    .index_for_key(vertex.key_ref())
                {
                    Some(index_ref) => {
                        self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                            .sparse_vector_mut_ref()
                            .set_element((*index_ref, *vertex.value_ref()).into())?;
                        Ok(None)
                    }
                    None => {
                        let vertex_index = self
                            .element_indexer_mut_ref()
                            .add_new_key(vertex.key_ref())?;
                        self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
                            .sparse_vector_mut_ref()
                            .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                        Ok(Some(vertex_index))
                    }
                }
            }
        }
    };
}

implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_add_vertex_element,
    vertex_vector_mut_ref
);
