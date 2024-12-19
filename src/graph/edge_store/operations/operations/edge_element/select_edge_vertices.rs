use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::value_type::ValueType;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;

static DEFAULT_OPERATOR_OPTIONS: Lazy<OptionsForOperatorWithAdjacencyMatrixArgument> =
    Lazy::new(|| OptionsForOperatorWithAdjacencyMatrixArgument::new_default());

pub(crate) trait SelectEdgeVertices<T: ValueType> {
    fn select_vertices_with_outgoing_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_vertices_with_incoming_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError>;
}
