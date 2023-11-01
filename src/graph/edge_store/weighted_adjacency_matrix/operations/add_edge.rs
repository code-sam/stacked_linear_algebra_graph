use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;

use crate::error::GraphComputingError;
use crate::graph::edge::DirectedEdgeCoordinateDefinedByIndicesTrait;
use crate::graph::edge::WeightedDirectedEdgeDefinedByIndices;
use crate::graph::edge::WeightedDirectedEdgeDefinedByIndicesTrait;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::index::ElementIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait AddEdge<T: ValueType> {
    fn add_edge_defined_by_indices_unchecked(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_edge_defined_by_indices_without_edge_type_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
        weight: &T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> AddEdge<T> for WeightedAdjacencyMatrix
where
    T: ValueType + Copy + SetSparseMatrixElementTyped<T>,
{
    fn add_edge_defined_by_indices_unchecked(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError> {
        let element = MatrixElement::new(
            (
                edge.coordinate_ref().tail_ref().clone(),
                edge.coordinate_ref().head_ref().clone(),
            )
                .into(),
            *edge.weight_ref(),
        );

        T::set_graphblas_matrix_element(self, element)?;
        Ok(())
    }

    fn add_edge_defined_by_indices_without_edge_type_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
        weight: &T,
    ) -> Result<(), GraphComputingError> {
        let element = MatrixElement::new((*tail, *head).into(), *weight);
        T::set_graphblas_matrix_element(self, element)?;
        Ok(())
    }
}
