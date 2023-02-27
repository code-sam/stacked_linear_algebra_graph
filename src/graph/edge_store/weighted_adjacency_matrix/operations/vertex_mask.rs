use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::context::ContextTrait;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::{
    ApplyElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionMonoidOperator,
};
use graphblas_sparse_linear_algebra::operators::monoid::{Any, LogicalOr};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::reduce::{MonoidReducer, MonoidVectorReducer};
use once_cell::sync::Lazy;

use crate::graph::edge_store::WeightedAdjacencyMatrixTrait;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::{error::GraphComputingError, graph::edge_store::WeightedAdjacencyMatrix};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

pub(crate) trait VertexMasking<T: ValueType> {
    fn mask_vertices_with_outgoing_edges(&self) -> Result<SparseVector<bool>, GraphComputingError>;
    fn mask_vertices_with_incoming_edges(&self) -> Result<SparseVector<bool>, GraphComputingError>;
    fn mask_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError>;
}

macro_rules! implement_vertex_indexing {
    ($value_type:ty) => {
        impl VertexMasking<$value_type> for WeightedAdjacencyMatrix<$value_type> {
            fn mask_vertices_with_outgoing_edges(
                &self,
            ) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut from_vertex_vector_mask = SparseVector::new(
                    self.sparse_matrix_ref().context_ref(),
                    &self.sparse_matrix_ref().row_height()?,
                )?;
                // TODO: think about caching for performance optimization
                let GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION =
                    MonoidReducer::<$value_type, bool>::new(
                        &Any::<$value_type>::new(),
                        &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                        None,
                    );
                GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION
                    .to_vector(&self.sparse_matrix_ref(), &mut from_vertex_vector_mask)?;
                Ok(from_vertex_vector_mask)
            }

            fn mask_vertices_with_incoming_edges(
                &self,
            ) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut to_vertex_vector_mask = SparseVector::new(
                    self.sparse_matrix_ref().context_ref(),
                    &self.sparse_matrix_ref().row_height()?,
                )?;
                let GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION =
                    MonoidReducer::<$value_type, bool>::new(
                        &Any::<$value_type>::new(),
                        &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                        None,
                    );
                GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION
                    .to_vector(&self.sparse_matrix_ref(), &mut to_vertex_vector_mask)?;
                Ok(to_vertex_vector_mask)
            }

            // TODO: this implementation is not type specific
            // TODO: wrap mask into a business struct
            fn mask_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut vertex_vector_mask = SparseVector::new(
                    self.sparse_matrix_ref().context_ref(),
                    &self.sparse_matrix_ref().row_height()?,
                )?;
                let GRAPHBLAS_VECTOR_OR_OPERATOR =
                    ElementWiseVectorAdditionMonoidOperator::<bool>::new(
                        &LogicalOr::<bool>::new(),
                        &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                        None,
                    );
                GRAPHBLAS_VECTOR_OR_OPERATOR.apply(
                    &self.mask_vertices_with_incoming_edges()?,
                    &self.mask_vertices_with_outgoing_edges()?,
                    &mut vertex_vector_mask,
                )?;
                Ok(vertex_vector_mask)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_vertex_indexing);
