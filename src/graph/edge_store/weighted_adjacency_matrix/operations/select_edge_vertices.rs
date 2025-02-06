use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorLength;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use graphblas_sparse_linear_algebra::context::GetContext;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::{
    ApplyElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionMonoidOperator,
};
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use graphblas_sparse_linear_algebra::operators::monoid::{Any, AnyMonoidTyped, LogicalOr};
use graphblas_sparse_linear_algebra::operators::reduce::{MonoidReducer, MonoidVectorReducer};

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::value_type::ValueType;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;

use super::GetMatrixSize;

pub(crate) trait SelectEdgeVertices {
    fn select_vertices_with_outgoing_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_vertices_with_incoming_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError>;
}

impl SelectEdgeVertices for WeightedAdjacencyMatrix {
    fn select_vertices_with_outgoing_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut from_vertex_vector_mask =
            SparseVector::new(self.graphblas_context(), self.vertex_capacity()?)?;

        // TODO: think about caching for performance optimization
        // let GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION =
        //     MonoidReducer::<$value_type>::new(
        //         &Any::<$value_type>::new(),
        //         &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
        //         &Assignment::new(),
        //     );
        MonoidReducer::new().to_column_vector(
            &Any::<bool>::new(),
            self,
            &Assignment::new(),
            &mut from_vertex_vector_mask,
            &SelectEntireVector::new(self.graphblas_context()), // TODO: cache this operator?
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )?;
        Ok(from_vertex_vector_mask)
    }

    fn select_vertices_with_incoming_edges(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut to_vertex_vector_mask =
            SparseVector::new(self.graphblas_context(), self.vertex_capacity()?)?;
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
            &Any::<bool>::new(),
            self,
            &Assignment::new(),
            &mut to_vertex_vector_mask,
            &SelectEntireVector::new(self.graphblas_context()), // TODO: cache this operator?
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )?;
        Ok(to_vertex_vector_mask)
    }

    // TODO: wrap mask into a business struct
    fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError> {
        select_connected_vertices(
            &SelectEdgeVertices::select_vertices_with_incoming_edges(self)?,
            &SelectEdgeVertices::select_vertices_with_outgoing_edges(self)?,
        )
    }
}

pub(crate) fn select_connected_vertices(
    vertices_with_incoming_edges: &SparseVector<bool>,
    vertices_with_outgoing_edges: &SparseVector<bool>,
) -> Result<SparseVector<bool>, GraphComputingError> {
    let mut vertex_vector_mask = SparseVector::new(
        vertices_with_incoming_edges.context(),
        vertices_with_incoming_edges.length()?,
    )?;

    ElementWiseVectorAdditionMonoidOperator::new().apply(
        vertices_with_incoming_edges,
        &LogicalOr::<bool>::new(),
        vertices_with_outgoing_edges,
        &Assignment::new(),
        &mut vertex_vector_mask,
        &SelectEntireVector::new(vertices_with_incoming_edges.context()), // TODO: cache this operator?
        &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
    )?;
    Ok(vertex_vector_mask)
}
