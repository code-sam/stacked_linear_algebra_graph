use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    GetMatrixElementList, MatrixElementList as AdjacencyMatrixElementList,
};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::get_adjacency_matrix::GetAdjacencyMatrix,
            weighted_adjacency_matrix::WeightedAdjacencyMatrixSparseMatrixTrait,
        },
        graph::{Graph, GraphTrait, EdgeTypeIndex},
        value_type::{ValueType, implement_macro_for_all_native_value_types}, edge::EdgeTypeKeyRef,
    },
};

pub trait ReadAdjacencyMatrixElementList<T: ValueType> {
    fn with_index(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn with_index_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn with_key(
        &self,
        type_key: &EdgeTypeKeyRef,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

macro_rules! implement_read_adjacency_matrix {
    ($value_type: ty) => {
        impl ReadAdjacencyMatrixElementList<$value_type> for Graph {
            fn with_index(
                &self,
                type_index: &EdgeTypeIndex,
            ) -> Result<AdjacencyMatrixElementList<$value_type>, GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self.edge_store_ref()
                            .try_adjacency_matrix_ref_for_index(type_index)?,
                    )
                    .get_element_list()?,
                )
            }
        
            fn with_index_unchecked(
                &self,
                type_index: &EdgeTypeIndex,
            ) -> Result<AdjacencyMatrixElementList<$value_type>, GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self.edge_store_ref()
                            .adjacency_matrix_ref_for_index_unchecked(type_index),
                    )
                    .get_element_list()?,
                )
            }
        
            fn with_key(
                &self,
                type_key: &EdgeTypeKeyRef,
            ) -> Result<AdjacencyMatrixElementList<$value_type>, GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self.edge_store_ref()
                            .adjacency_matrix_ref_for_key(type_key)?,
                    )
                    .get_element_list()?,
                )
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_read_adjacency_matrix);
