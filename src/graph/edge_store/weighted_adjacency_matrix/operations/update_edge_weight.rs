use graphblas_sparse_linear_algebra::collections::sparse_matrix::SetMatrixElement;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;

use crate::error::GraphComputingError;

use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    WeightedAdjacencyMatrix, WeightedAdjacencyMatrixSparseMatrixTrait,
};

use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

pub(crate) trait UpdateEdgeWeight<T: ValueType> {
    fn update_edge_weight_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
        weigth: &T,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_update_edge_weigth {
    ($value_type:ty) => {
        impl UpdateEdgeWeight<$value_type> for WeightedAdjacencyMatrix {
            fn update_edge_weight_unchecked(
                &mut self,
                coordinate: &AdjacencyMatrixCoordinate,
                weigth: &$value_type,
            ) -> Result<(), GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_mut_ref(
                        self,
                    )
                    .set_element(MatrixElement::new(*coordinate, *weigth))?,
                )
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_update_edge_weigth);
