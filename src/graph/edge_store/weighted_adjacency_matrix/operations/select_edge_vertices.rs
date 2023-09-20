use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::{
    ApplyElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionMonoidOperator,
};
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::monoid::{Any, AnyMonoidTyped, LogicalOr};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::reduce::{MonoidReducer, MonoidVectorReducer};
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixTrait;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    SparseWeightedAdjacencyMatrix, SparseWeightedAdjacencyMatrixForValueType,
    WeightedAdjacencyMatrix,
};
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

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

impl<T: ValueType + SparseWeightedAdjacencyMatrixForValueType<T> + AnyMonoidTyped<T>>
    SelectEdgeVertices<T> for WeightedAdjacencyMatrix
{
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
            &Any::<T>::new(),
            SparseWeightedAdjacencyMatrix::<T>::sparse_matrix_ref(self),
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
            &Any::<T>::new(),
            SparseWeightedAdjacencyMatrix::<T>::sparse_matrix_ref(self),
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
            &SelectEdgeVertices::<T>::select_vertices_with_incoming_edges(self)?,
            &LogicalOr::<bool>::new(),
            &SelectEdgeVertices::<T>::select_vertices_with_outgoing_edges(self)?,
            &Assignment::new(),
            &mut vertex_vector_mask,
            &SelectEntireVector::new(self.graphblas_context_ref()),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
        )?;
        Ok(vertex_vector_mask)
    }
}
