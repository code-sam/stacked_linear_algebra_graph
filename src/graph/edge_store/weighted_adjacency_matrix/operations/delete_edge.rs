use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;

use crate::error::GraphComputingError;
use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    WeightedAdjacencyMatrix, WeightedAdjacencyMatrixSparseMatrixTrait,
};

use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

pub(crate) trait DeleteEdge<T: ValueType> {
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_delete_edge {
    ($value_type:ty) => {
        impl DeleteEdge<$value_type> for WeightedAdjacencyMatrix {
            fn delete_edge_unchecked(
                &mut self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<(), GraphComputingError> {
                WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_mut_ref(
                    self,
                )
                .drop_element(coordinate.clone())?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_delete_edge);
