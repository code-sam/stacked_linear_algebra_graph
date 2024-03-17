use std::sync::Arc;

use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use super::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;

use super::weighted_adjacency_matrix::GetGraphblasContext;
use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;

use crate::graph::index::ElementCount;
use crate::graph::indexer::Indexer as EdgeTypeIndexer;

#[derive(Clone, Debug)]
pub(crate) struct EdgeStore {
    graphblas_context: Arc<GraphblasContext>,
    adjacency_matrices: Vec<WeightedAdjacencyMatrixWithCachedAttributes>,
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
            adjacency_matrices: Vec::<WeightedAdjacencyMatrixWithCachedAttributes>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
            adjacency_matrix_size: *initial_vertex_capacity,
            mask_to_select_entire_adjacency_matrix: SelectEntireMatrix::new(graphblas_context),
        })
    }
}

pub(crate) trait GetAdjacencyMatrices {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrixWithCachedAttributes];
    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrixWithCachedAttributes];
    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes>;

    fn adjacency_matrix_size_ref(&self) -> &ElementCount;
    fn adjacency_matrix_size_mut_ref(&mut self) -> &mut ElementCount;

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix;
}

pub(super) trait GetEdgeTypeIndicer {
    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer;
    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer;
}

impl GetGraphblasContext for EdgeStore {
    fn graphblas_context(&self) -> Arc<GraphblasContext> {
        self.graphblas_context.to_owned()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
}

impl GetAdjacencyMatrices for EdgeStore {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrixWithCachedAttributes] {
        self.adjacency_matrices.as_slice()
    }

    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrixWithCachedAttributes] {
        self.adjacency_matrices.as_mut_slice()
    }

    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrixWithCachedAttributes> {
        &mut self.adjacency_matrices
    }

    fn adjacency_matrix_size_ref(&self) -> &ElementCount {
        &self.adjacency_matrix_size
    }

    fn adjacency_matrix_size_mut_ref(&mut self) -> &mut ElementCount {
        &mut self.adjacency_matrix_size
    }

    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix {
        &self.mask_to_select_entire_adjacency_matrix
    }
}

impl GetEdgeTypeIndicer for EdgeStore {
    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer {
        &self.edge_type_indexer
    }

    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer {
        &mut self.edge_type_indexer
    }
}
