use std::sync::Arc;

use super::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::indexer::Indexer as EdgeTypeIndexer;
use crate::graph::value_type::ValueType;

#[derive(Clone, Debug)]
pub(crate) struct EdgeStore<T: ValueType> {
    graphblas_context: Arc<GraphblasContext>,
    adjacency_matrices: Vec<WeightedAdjacencyMatrix<T>>,
    edge_type_indexer: EdgeTypeIndexer,
}

impl<T: ValueType> EdgeStore<T> {
    pub(crate) fn with_initial_capacity(
        graphblas_context: &Arc<GraphblasContext>,
        initial_vertex_capacity: &ElementCount,
        initial_edge_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            graphblas_context: graphblas_context.clone(),
            edge_type_indexer: EdgeTypeIndexer::with_initial_capacity(
                graphblas_context,
                initial_edge_type_capacity,
            )?,
            adjacency_matrices: Vec::<WeightedAdjacencyMatrix<T>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
        })
    }
}

pub(crate) trait EdgeStoreTrait<T: ValueType> {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix<T>];
    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix<T>];
    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix<T>>;

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer;
    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer;
}

impl<T: ValueType> EdgeStoreTrait<T> for EdgeStore<T> {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix<T>] {
        self.adjacency_matrices.as_slice()
    }

    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix<T>] {
        self.adjacency_matrices.as_mut_slice()
    }

    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix<T>> {
        &mut self.adjacency_matrices
    }

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer {
        &self.edge_type_indexer
    }

    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer {
        &mut self.edge_type_indexer
    }
}
