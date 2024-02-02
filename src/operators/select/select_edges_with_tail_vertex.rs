use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;

use graphblas_sparse_linear_algebra::index::ElementIndexSelector as VertexSelector;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixRow;

use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;

use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;
use crate::graph::graph::{Graph, VertexIndex, VertexTypeIndex};
use crate::graph::vertex_store::VertexStoreTrait;

use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType},
};

pub trait SelectEdgesWithTailVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> SelectEdgesWithTailVertex<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref(extract_to)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_row_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                tail_vertex,
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
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(adjacency_matrix);

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(extract_to);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_row_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                tail_vertex,
                &VertexSelector::All,
                accumlator,
                vertex_vector_extract_to,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait SelectMaskedEdgesWithTailVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> SelectMaskedEdgesWithTailVertex<EvaluationDomain> for Graph
where
    SparseMatrix<EvaluationDomain>: MatrixMask,
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref(adjacency_matrix)?;

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref(extract_to)?;

        let vertex_vector_mask = unsafe { &mut *vertex_store }.vertex_vector_mut_ref(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_row_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                tail_vertex,
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
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_adjacency_matrix =
            unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(adjacency_matrix);

        let vertex_vector_extract_to =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(extract_to);

        let vertex_vector_mask =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(mask);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_row_extractor()
            .apply(
                adjacency_matrix_adjacency_matrix,
                tail_vertex,
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

    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetVertexValue;

    #[test]
    fn select_edges_with_tail_vertex() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_result_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u16>::apply(&mut graph).unwrap();
        let _result_edge_type_index = AddEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        SelectEdgesWithTailVertex::<isize>::by_index(
            &mut graph,
            &edge_type_1_index,
            &vertex_1_index,
            // &VertexSelector::All,
            &Plus::<isize>::new(),
            &vertex_result_type_index,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_2_index,
            )
            .unwrap(),
            Some(1)
        );

        SelectEdgesWithTailVertex::<isize>::by_index(
            &mut graph,
            &edge_type_1_index,
            &vertex_2_index,
            // &VertexSelector::Index(&vec![0]),
            &Assignment::new(),
            &vertex_result_type_index,
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
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_1_index,
            )
            .unwrap(),
            Some(2)
        );
    }
}
