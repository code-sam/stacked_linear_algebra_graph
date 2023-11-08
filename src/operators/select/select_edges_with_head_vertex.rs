use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector as VertexSelector;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixColumn;
use graphblas_sparse_linear_algebra::operators::mask::VectorMask;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;

use crate::graph::graph::{Graph, VertexIndex, VertexTypeIndex};
use crate::graph::graph::{GraphTrait, GraphblasOperatorApplierCollectionTrait};
use crate::graph::indexer::IndexerTrait;
use crate::graph::vertex::vertex::VertexKeyRef;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType, vertex::vertex::VertexTypeKeyRef},
};

pub trait SelectEdgesWithHeadVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseVector<EvaluationDomain>: VectorMask,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector, // Selecting a subset of the the tail vertices will result in a collection with incompatible size
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        head_vertex: &VertexKeyRef,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> SelectEdgesWithHeadVertex<EvaluationDomain>
    for Graph
where
    SparseVector<EvaluationDomain>: VectorMask,
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(extract_to)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(extract_to);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        head_vertex: &VertexKeyRef,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(extract_to)?;

        let head_vertex_index = self
            .vertex_store_ref()
            .element_indexer_ref()
            .try_index_for_key(head_vertex)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex_index,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait SelectMaskedEdgesWithHeadVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        head_vertex: &VertexKeyRef,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain>
    SelectMaskedEdgesWithHeadVertex<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(extract_to)?;

        let vertex_vector_mask =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                vertex_vector_mask,
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        head_vertex: &VertexIndex,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(extract_to);

        let vertex_vector_mask =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(mask);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                vertex_vector_mask,
                options,
            )?)
    }

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        head_vertex: &VertexKeyRef,
        // tail_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(extract_to)?;

        let vertex_vector_mask =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(mask)?;

        let head_vertex_index = self
            .vertex_store_ref()
            .element_indexer_ref()
            .try_index_for_key(head_vertex)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_column_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                head_vertex_index,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                vertex_vector_mask,
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };
    use crate::graph::vertex::vertex_defined_by_key::{
        VertexDefinedByKey, VertexDefinedByKeyTrait,
    };
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadVertexValue;

    #[test]
    fn select_edges_with_head_vertex() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_key = "vertex_type";
        let vertex_result_type_key = "vertex_result_type";
        let edge_type_1_key = "edge_type_1";
        let edge_type_2_key = "edge_type_2";
        let result_type_key = "result_type";

        let vertex_1 = VertexDefinedByKey::new(vertex_type_key, "vertex_1", &1u8);
        let vertex_2 = VertexDefinedByKey::new(vertex_type_key, "vertex_2", &2u8);

        let edge_vertex1_vertex2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1_key,
                vertex_2.key_ref(),
                vertex_1.key_ref(),
            ),
            25usize,
        );
        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_2_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            3u32,
        );

        let _vertex_type_1_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_type_key).unwrap();
        let _vertex_type_2_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_result_type_key).unwrap();
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_1_key).unwrap();
        let _edge_type_2_index =
            AddEdgeType::<u16>::add_new_edge_type(&mut graph, edge_type_2_key).unwrap();
        let _result_edge_type_index =
            AddEdgeType::<isize>::add_new_edge_type(&mut graph, result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        SelectEdgesWithHeadVertex::<isize>::by_key(
            &mut graph,
            &edge_type_1_key,
            vertex_2.key_ref(),
            // &VertexSelector::All,
            &Plus::<isize>::new(),
            vertex_result_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            ReadVertexValue::<isize>::vertex_value_by_key(
                &graph,
                vertex_result_type_key,
                vertex_1.key_ref(),
            )
            .unwrap(),
            Some(1)
        );

        SelectEdgesWithHeadVertex::<isize>::by_key(
            &mut graph,
            &edge_type_1_key,
            vertex_1.key_ref(),
            // &VertexSelector::All,
            &Assignment::new(),
            vertex_result_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     ReadAdjacencyMatrixElementList::<u8>::with_key(&graph, edge_type_1_key).unwrap()
        // );
        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<isize>::with_key(&graph, vertex_result_type_key).unwrap()
        // );
        assert_eq!(
            ReadVertexValue::<isize>::vertex_value_by_key(
                &graph,
                vertex_result_type_key,
                vertex_2.key_ref(),
            )
            .unwrap(),
            Some(25)
        );
    }
}
