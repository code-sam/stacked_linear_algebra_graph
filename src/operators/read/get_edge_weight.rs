use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::GetEdgeWeight as GetAdjacencyMatrixEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::IntoSparseMatrixForValueType;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;

use crate::operators::indexing::{CheckIndex, CheckPrivateIndex};

pub trait GetEdgeWeight<T: ValueType> {
    fn edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    // These still require valid indices
    fn edge_weight_or_default(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn edge_weight_or_default_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;
}

pub(crate) trait GetPrivateEdgeWeight<T: ValueType> {
    fn private_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    // These still require valid indices
    fn private_edge_weight_or_default(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn edge_weight_or_default_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_private_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<T> GetEdgeWeight<T> for Graph
where
    T: ValueType + IntoSparseMatrixForValueType<T> + GetSparseMatrixElementValueTyped<T> + Default,
{
    fn edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.try_edge_validity(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .edge_weight_unchecked(tail, head)
    }

    fn edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.try_edge_coordinate_validity(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .edge_weight_at_coordinate_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
    }

    fn try_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_edge_validity(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .try_edge_weight_unchecked(tail, head)
    }

    fn try_edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_edge_coordinate_validity(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .try_edge_weight_at_coordinate_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
    }

    /// Requires valid coordinate
    fn edge_weight_or_default(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_edge_validity(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .edge_weight_or_default_unchecked(tail, head)
    }

    /// Requires valid coordinate
    fn edge_weight_or_default_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_edge_coordinate_validity(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .edge_weight_or_default_at_coordinate_unchecked(
                &edge_coordinate.adjacency_matrix_coordinate(),
            )
    }
}

impl<T> GetPrivateEdgeWeight<T> for Graph
where
    T: ValueType + IntoSparseMatrixForValueType<T> + GetSparseMatrixElementValueTyped<T> + Default,
{
    fn private_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.try_is_valid_private_edge(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .edge_weight_unchecked(tail, head)
    }

    fn edge_weight_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.try_is_valid_private_edge_coordinate(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .edge_weight_at_coordinate_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
    }

    fn try_private_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_private_edge(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .try_edge_weight_unchecked(tail, head)
    }

    fn try_edge_weight_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_private_edge_coordinate(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .try_edge_weight_at_coordinate_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
    }

    /// Requires valid coordinate
    fn private_edge_weight_or_default(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_private_edge(edge_type, tail, head)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type)
            .edge_weight_or_default_unchecked(tail, head)
    }

    /// Requires valid coordinate
    fn edge_weight_or_default_for_private_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_private_edge_coordinate(edge_coordinate)?;
        self.edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_coordinate.edge_type_ref())
            .edge_weight_or_default_at_coordinate_unchecked(
                &edge_coordinate.adjacency_matrix_coordinate(),
            )
    }
}

#[cfg(test)]
mod tests {}
