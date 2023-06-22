use std::sync::Arc;

use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use super::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixTrait;

use crate::graph::index::ElementCount;
use crate::graph::indexer::Indexer as EdgeTypeIndexer;
use crate::graph::indexer::IndexerTrait;

#[derive(Clone, Debug)]
pub(crate) struct EdgeStore {
    graphblas_context: Arc<GraphblasContext>,
    adjacency_matrices: Vec<WeightedAdjacencyMatrix>,
    edge_type_indexer: EdgeTypeIndexer,
    adjacency_matrix_size: ElementCount,
    mask_to_select_entire_adjacency_matrix: SelectEntireMatrix,
}

impl EdgeStore {
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
            adjacency_matrices: Vec::<WeightedAdjacencyMatrix>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
            adjacency_matrix_size: *initial_vertex_capacity,
            mask_to_select_entire_adjacency_matrix: SelectEntireMatrix::new(graphblas_context),
        })
    }
}

pub(crate) trait EdgeStoreTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;
    fn adjacency_matrix_size_ref(&self) -> &ElementCount;
    fn adjacency_matrix_size_mut_ref(&mut self) -> &mut ElementCount;

    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix];
    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix];
    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix>;

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer;
    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer;

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix;

    ///
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl EdgeStoreTrait for EdgeStore {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        &self.adjacency_matrix_size
    }
    fn adjacency_matrix_size_mut_ref(&mut self) -> &mut ElementCount {
        &mut self.adjacency_matrix_size
    }

    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix] {
        self.adjacency_matrices.as_slice()
    }

    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix] {
        self.adjacency_matrices.as_mut_slice()
    }

    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix> {
        &mut self.adjacency_matrices
    }

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer {
        &self.edge_type_indexer
    }

    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer {
        &mut self.edge_type_indexer
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        &self.mask_to_select_entire_adjacency_matrix
    }

    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_adjacency_matrices(|adjacency_matrix: &mut WeightedAdjacencyMatrix| {
            adjacency_matrix.resize(new_vertex_capacity)
        })?;
        self.adjacency_matrix_size = new_vertex_capacity;
        Ok(())
    }
}

impl EdgeStore {
    /// Apply function to all adjacency matrices
    pub(crate) fn map_mut_all_adjacency_matrices<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut WeightedAdjacencyMatrix) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.adjacency_matrices
            .as_mut_slice()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }

    pub(crate) fn map_mut_all_valid_adjacency_matrices<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut WeightedAdjacencyMatrix) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.edge_type_indexer
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(&mut self.adjacency_matrices_mut_ref()[i])
            })?;
        Ok(())
    }
}
