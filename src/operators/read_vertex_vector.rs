use graphblas_sparse_linear_algebra::collections::{sparse_vector::{
    GetVectorElementList, VectorElementList as VertexVectorElementList,
}, sparse_matrix::GetMatrixElementList};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::get_adjacency_matrix::GetAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrixSparseMatrixTrait,
        },
        graph::{Graph, GraphTrait, VertexTypeIndex},
        value_type::{ValueType, implement_macro_for_all_native_value_types},
        vertex::VertexTypeKeyRef, vertex_store::{SparseVertexVector, type_operations::get_vertex_vector::GetVertexVector},
    },
};

pub trait ReadVertexVectorElementList<T: ValueType> {
    fn with_index(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
    fn with_index_unchecked(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
    fn with_key(
        &self,
        type_key: &VertexTypeKeyRef,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
}

macro_rules! implement_read_vertex_vector {
    ($value_type: ty) => {
        impl ReadVertexVectorElementList<$value_type> for Graph {
            fn with_index(
                &self,
                type_index: &VertexTypeIndex,
            ) -> Result<VertexVectorElementList<$value_type>, GraphComputingError> {
                Ok(
                    SparseVertexVector::<$value_type>::sparse_vector_ref(
                        self.vertex_store_ref()
                            .vertex_vector_ref_by_index(type_index)?,
                    )
                    .get_element_list()?,
                )
            }
        
            fn with_index_unchecked(
                &self,
                type_index: &VertexTypeIndex,
            ) -> Result<VertexVectorElementList<$value_type>, GraphComputingError> {
                Ok(
                    SparseVertexVector::<$value_type>::sparse_vector_ref(
                        self.vertex_store_ref()
                            .vertex_vector_ref_by_index_unchecked(type_index),
                    )
                    .get_element_list()?,
                )
            }
        
            fn with_key(
                &self,
                type_key: &VertexTypeKeyRef,
            ) -> Result<VertexVectorElementList<$value_type>, GraphComputingError> {
                Ok(
                    SparseVertexVector::<$value_type>::sparse_vector_ref(
                        self.vertex_store_ref()
                            .vertex_vector_ref_by_key(type_key)?,
                    )
                    .get_element_list()?,
                )
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_read_vertex_vector);

