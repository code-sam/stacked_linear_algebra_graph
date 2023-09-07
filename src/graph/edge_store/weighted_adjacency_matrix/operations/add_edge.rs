use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElement;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;
use graphblas_sparse_linear_algebra::value_type::ValueType as GraphblasValueType;

use crate::error::GraphComputingError;
use crate::graph::edge::DirectedEdgeCoordinateDefinedByIndicesTrait;
use crate::graph::edge::WeightedDirectedEdgeDefinedByIndices;
use crate::graph::edge::WeightedDirectedEdgeDefinedByIndicesTrait;
use crate::graph::edge_store::weighted_adjacency_matrix::SparseWeightedAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::SparseWeightedAdjacencyMatrixForValueType;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::index::ElementIndex;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

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
    T: ValueType + SetMatrixElementTyped<T> + SparseWeightedAdjacencyMatrixForValueType<T>,
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

        self.sparse_matrix_mut_ref().set_element(element)?;
        Ok(())
    }

    fn add_edge_defined_by_indices_without_edge_type_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
        weight: &T,
    ) -> Result<(), GraphComputingError> {
        self.sparse_matrix_mut_ref()
            .set_element(MatrixElement::new((*tail, *head).into(), *weight))?;
        Ok(())
    }
}
