use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::{
    ApplyElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionMonoidOperator,
};
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::monoid::{Any, LogicalOr};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::reduce::{MonoidReducer, MonoidVectorReducer};
use once_cell::sync::Lazy;

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixSparseMatrixTrait;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixTrait;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::{
    error::GraphComputingError,
    graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix,
};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

pub(crate) trait SelectEdgeVertices<T: ValueType> {
    fn select_vertices_with_outgoing_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_vertices_with_incoming_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError>;
}

macro_rules! implement_vertex_indexing {
    ($value_type:ty) => {
        impl SelectEdgeVertices<$value_type> for WeightedAdjacencyMatrix {
            fn select_vertices_with_outgoing_edges(
                &self,
            ) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut from_vertex_vector_mask =
                    SparseVector::new(self.graphblas_context_ref(), &self.vertex_capacity()?)?;

                // TODO: think about caching for performance optimization
                // let GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION =
                //     MonoidReducer::<$value_type>::new(
                //         &Any::<$value_type>::new(),
                //         &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                //         &Assignment::new(),
                //     );
                MonoidReducer::new().to_column_vector(
                    &Any::<$value_type>::new(),
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self,
                    ),
                    &Assignment::new(),
                    &mut from_vertex_vector_mask,
                    &SelectEntireVector::new(self.graphblas_context_ref()),
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;
                Ok(from_vertex_vector_mask)
            }

            fn select_vertices_with_incoming_edges(
                &self,
            ) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut to_vertex_vector_mask =
                    SparseVector::new(self.graphblas_context_ref(), &self.vertex_capacity()?)?;
                // let GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION =
                //     MonoidReducer::<$value_type>::new(
                //         &Any::<$value_type>::new(),
                //         &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                //         &Assignment::new(),
                //     );
                // GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION.to_vector(
                //     WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                //         self,
                //     ),
                //     &mut to_vertex_vector_mask,
                // )?;
                MonoidReducer::new().to_row_vector(
                    &Any::<$value_type>::new(),
                    WeightedAdjacencyMatrixSparseMatrixTrait::<$value_type>::sparse_matrix_ref(
                        self,
                    ),
                    &Assignment::new(),
                    &mut to_vertex_vector_mask,
                    &SelectEntireVector::new(self.graphblas_context_ref()),
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;
                Ok(to_vertex_vector_mask)
            }

            // TODO: wrap mask into a business struct
            fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError> {
                let mut vertex_vector_mask =
                    SparseVector::new(self.graphblas_context_ref(), &self.vertex_capacity()?)?;

                ElementWiseVectorAdditionMonoidOperator::new().apply(
                    &SelectEdgeVertices::<$value_type>::select_vertices_with_incoming_edges(self)?,
                    &LogicalOr::<bool>::new(),
                    &SelectEdgeVertices::<$value_type>::select_vertices_with_outgoing_edges(self)?,
                    &Assignment::new(),
                    &mut vertex_vector_mask,
                    &SelectEntireVector::new(self.graphblas_context_ref()),
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;
                Ok(vertex_vector_mask)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_vertex_indexing);
