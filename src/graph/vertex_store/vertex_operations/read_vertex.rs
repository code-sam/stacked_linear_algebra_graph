use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetMatrixElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetVectorElementValue;

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::graph::VertexIndex;
use crate::graph::graph::VertexTypeIndex;
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::ValueType;

use crate::graph::vertex::vertex::VertexKeyRef;
use crate::graph::vertex::vertex::VertexTypeKeyRef;
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::vertex_vector::SparseVertexVector;

pub(crate) trait ReadVertex<T: ValueType> {
    // fn edge_weight_unchecked(
    //     &self,
    //     coordinate: &AdjacencyMatrixCoordinate,
    // ) -> Result<Option<T>, GraphComputingError>;
    // fn edge_weight_or_default_unchecked(
    //     &self,
    //     coordinate: &AdjacencyMatrixCoordinate,
    // ) -> Result<T, GraphComputingError>;
    // fn try_edge_weight_unchecked(
    //     &self,
    //     coordinate: &AdjacencyMatrixCoordinate,
    // ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    // fn vertex_value_by_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl ReadVertex<$value_type> for VertexStore {
            // fn vertex_value_by_key(
            //     &self,
            //     vertex_type_key: &VertexTypeKeyRef,
            //     vertex_key: &VertexKeyRef,
            // ) -> Result<$value_type, GraphComputingError> {
            //     let type_index = self
            //         .vertex_type_indexer_ref()
            //         .try_index_for_key(vertex_type_key)?;
            //     let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
            //     self.vertex_value_by_index_unchecked(type_index, vertex_index)
            // }

            fn vertex_value_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                let type_index = self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex_type_key)?;
                let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
                self.vertex_value_by_index_unchecked(type_index, vertex_index)
            }

            fn try_vertex_value_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<$value_type, GraphComputingError> {
                let type_index = self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex_type_key)?;
                let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
                self.try_vertex_value_by_index_unchecked(type_index, vertex_index)
            }

            fn vertex_value_or_default_by_key(
                &self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_key: &VertexKeyRef,
            ) -> Result<$value_type, GraphComputingError> {
                let type_index = self
                    .vertex_type_indexer_ref()
                    .try_index_for_key(vertex_type_key)?;
                let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
                self.vertex_value_or_default_by_index_unchecked(type_index, vertex_index)
            }

            // fn vertex_value_by_type_index_and_vertex_key(
            //     &self,
            //     vertex_type_index: &VertexTypeIndex,
            //     vertex_key: &VertexKeyRef,
            // ) -> Result<$value_type, GraphComputingError> {
            //     self.vertex_type_indexer_ref()
            //         .try_index_validity(vertex_type_index);
            //     let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
            //     self.vertex_value_by_index_unchecked(vertex_type_index, vertex_index)
            // }

            fn vertex_value_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                Ok(self
                    .vertex_vector_ref_by_index(vertex_type_index)?
                    .sparse_vector_ref()
                    .get_element_value(vertex_index)?)
            }

            fn try_vertex_value_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                match self
                    .vertex_vector_ref_by_index(vertex_type_index)?
                    .sparse_vector_ref()
                    .get_element_value(vertex_index)? {
                        Some(weight) => Ok(weight),
                        None => Err(LogicError::new(
                            LogicErrorType::EdgeMustExist,
                            format!("No vertex value exists at vertex index: {:?}, for vertex type index: {:?}, and value type: {}",
                                vertex_index, vertex_type_index, std::any::type_name::<$value_type>()),
                            None,
                        )
                        .into()),
                    }
            }

            fn vertex_value_or_default_by_index(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                Ok(self
                    .vertex_vector_ref_by_index(vertex_type_index)?
                    .sparse_vector_ref()
                    .get_element_value_or_default(vertex_index)?)
            }

            fn vertex_value_by_index_unchecked(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                Ok(self
                    .vertex_vector_ref_by_index_unchecked(vertex_type_index)
                    .sparse_vector_ref()
                    .get_element_value(vertex_index)?)
            }

            fn try_vertex_value_by_index_unchecked(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                match self
                    .vertex_vector_ref_by_index_unchecked(vertex_type_index)
                    .sparse_vector_ref()
                    .get_element_value(vertex_index)? {
                        Some(weight) => Ok(weight),
                        None => Err(LogicError::new(
                            LogicErrorType::EdgeMustExist,
                            format!("No vertex value exists at vertex index: {:?}, for vertex type index: {:?}, and value type: {}",
                                vertex_index, vertex_type_index, std::any::type_name::<$value_type>()),
                            None,
                        )
                        .into()),
                    }
            }

            fn vertex_value_or_default_by_index_unchecked(
                &self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                Ok(self
                    .vertex_vector_ref_by_index_unchecked(vertex_type_index)
                    .sparse_vector_ref()
                    .get_element_value_or_default(vertex_index)?)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_set_vertex_data);
