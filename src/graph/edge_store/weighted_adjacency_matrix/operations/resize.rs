// ALREADY IMPLEMENTED IN WEIGHTED_ADJACENCY_MATRIX_TRAIT

// use graphblas_sparse_linear_algebra::collections::sparse_matrix::{Coordinate, SparseMatrixTrait};

// use crate::error::GraphComputingError;
// use crate::graph::edge::AdjacencyMatrixCoordinate;
// use crate::graph::edge_store::{WeightedAdjacencyMatrix, WeightedAdjacencyMatrixTrait};
// use crate::graph::index::ElementCount;
// use crate::graph::value_type::ValueType;

// pub(crate) trait Resize {
//     fn resize(
//         &mut self,
//         new_vertex_capacity: &ElementCount,
//     ) -> Result<(), GraphComputingError>;
// }

// impl<T: ValueType> Resize for WeightedAdjacencyMatrix<T> {
//     fn resize(
//         &mut self,
//         new_vertex_capacity: &ElementCount,
//     ) -> Result<(), GraphComputingError> {
//         self.sparse_matrix_mut_ref().resize(&(*new_vertex_capacity, *new_vertex_capacity).into())?;
//         Ok(())
//     }
// }
