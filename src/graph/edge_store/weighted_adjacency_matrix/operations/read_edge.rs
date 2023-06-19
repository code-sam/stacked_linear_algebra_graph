use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetMatrixElementValue;

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    WeightedAdjacencyMatrix, WeightedAdjacencyMatrixSparseMatrixTrait,
};

use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

pub(crate) trait ReadEdge<T: ValueType> {
    fn edge_weight_unchecked(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<Option<T>, GraphComputingError>;
    fn edge_weight_or_default_unchecked(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<T, GraphComputingError>;
    fn try_edge_weight_unchecked(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<T, GraphComputingError>;
}

macro_rules! implement_delete_edge {
    ($value_type:ty) => {
        impl ReadEdge<$value_type> for WeightedAdjacencyMatrix {
            fn edge_weight_unchecked(
                &self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self,
                    )
                    .get_element_value(coordinate)?,
                )
            }
            fn edge_weight_or_default_unchecked(
                &self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<$value_type, GraphComputingError> {
                Ok(
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self,
                    )
                    .get_element_value_or_default(coordinate)?,
                )
            }
            fn try_edge_weight_unchecked(
                &self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<$value_type, GraphComputingError> {
                match WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                    self,
                )
                .get_element_value(coordinate)?
                {
                    Some(weight) => Ok(weight),
                    None => Err(LogicError::new(
                        LogicErrorType::EdgeMustExist,
                        format!(
                            "No edge exists at coordinate: {:?} for value type: {}",
                            coordinate,
                            std::any::type_name::<$value_type>()
                        ),
                        None,
                    )
                    .into()),
                }
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_delete_edge);
