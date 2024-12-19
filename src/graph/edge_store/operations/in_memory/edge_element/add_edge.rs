use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::operations::operations::edge_element::AddEdge;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;
use crate::graph::weighted_adjacency_matrix::operations::AddEdge as AddEdgeToAdjacencyMatrix;

impl<T> AddEdge<T> for EdgeStore
where
    T: ValueType + Copy + SetSparseMatrixElementTyped<T>,
{
    fn add_public_weighted_directed_edge(
        &mut self,
        edge_type_index: &impl crate::graph::indexing::GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        todo!()
    }
    
    fn add_public_edge(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        todo!()
    }
    
    fn add_private_weighted_directed_edge(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        todo!()
    }
    
    fn add_private_edge(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.is_valid_private_edge_type_index(edge_type_index)?;
    }
    
    fn add_weighted_directed_edge_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_mut_ref_unchecked(edge_type_index).add_weighted_directed_edge_unchecked(edge)
    }
    
    fn add_edge_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_mut_ref_unchecked(edge_type_index).add_edge_unchecked(tail, head, weight)
    }
}
