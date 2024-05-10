use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValue, GetSparseMatrixElementValueTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    GetAdjacencyMatrixCoordinateIndices, IntoSparseMatrix, IntoSparseMatrixForValueType,
    WeightedAdjacencyMatrix,
};

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;

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
    T: ValueType + IntoSparseMatrixForValueType<T> + GetSparseMatrixElementValueTyped<T> + Default,
{
    fn edge_weight_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        Ok(self
            .sparse_matrix()?
            .element_value(tail.index_ref(), head.index_ref())?)
    }

    fn edge_weight_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        Ok(self
            .sparse_matrix()?
            .element_value_at_coordinate(coordinate)?)
    }

    fn edge_weight_or_default_unchecked(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        Ok(self
            .sparse_matrix()?
            .element_value_or_default(tail.index_ref(), head.index_ref())?)
    }

    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        Ok(self
            .sparse_matrix()?
            .element_value_or_default_at_coordinate(coordinate)?)
    }

    fn try_edge_weight_unchecked(
        &self,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        match self
            .sparse_matrix()?
            .element_value(tail.index_ref(), head.index_ref())?
        {
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
