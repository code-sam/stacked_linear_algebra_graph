use crate::graph::value_type::SparseVertexVectorForValueType;
use crate::graph::vertex_store::VertexVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector as VertexSelector;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixColumn;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixRow;
use graphblas_sparse_linear_algebra::operators::mask::VectorMask;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
        mask::MatrixMask,
        options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::GraphTrait;
use crate::graph::graph::{Graph, VertexIndex, VertexTypeIndex};
use crate::graph::indexer::IndexerTrait;
use crate::graph::vertex::VertexKeyRef;
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::SparseVertexVector;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        value_type::{
            implement_macro_for_all_native_value_types, SparseAdjacencyMatrixForValueType,
            ValueType,
        },
        vertex::VertexTypeKeyRef,
    },
};

pub trait SelectEdgesWithTailVertex<AdjacencyMatrix, ExtractTo>
where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    ExtractTo: ValueType + SparseVertexVectorForValueType<ExtractTo>,
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseVector<ExtractTo>: VectorMask,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        tail_vertex: &VertexKeyRef,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

        impl<AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>, EvaluationDomain: ValueType + SparseVertexVectorForValueType<EvaluationDomain>>
            SelectEdgesWithTailVertex<AdjacencyMatrix, EvaluationDomain> for Graph
        where
            SparseMatrix<AdjacencyMatrix>: MatrixMask,
            SparseVector<EvaluationDomain>: VectorMask,
            VertexVector: SparseVertexVector<EvaluationDomain>
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
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

                let vertex_vector_extract_to =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(extract_to)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        tail_vertex,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
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

                let adjacency_matrix_adjacency_matrix = unsafe { &*edge_store }
                    .adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

                let vertex_vector_extract_to = unsafe { &mut *vertex_store }
                    .vertex_vector_mut_ref_by_index_unchecked(extract_to);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        tail_vertex,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
                        unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                        options,
                    )?)
            }

            fn by_key(
                &mut self,
                adjacency_matrix: &EdgeTypeKeyRef,
                tail_vertex: &VertexKeyRef,
                // head_vertex_selector: &VertexSelector,
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
                    .try_index_for_key(tail_vertex)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        head_vertex_index,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
                        unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                        options,
                    )?)
            }
        }

pub trait SelectMaskedEdgesWithTailVertex<AdjacencyMatrix, ExtractTo, Mask>
where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    ExtractTo: ValueType + SparseAdjacencyMatrixForValueType<ExtractTo>,
    SparseMatrix<ExtractTo>: MatrixMask,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    SparseVector<Mask>: VectorMask,
{
    fn by_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        tail_vertex: &VertexIndex,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        tail_vertex: &VertexKeyRef,
        // head_vertex_selector: &VertexSelector,
        accumlator: &impl AccumulatorBinaryOperator<ExtractTo>,
        extract_to: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

        impl<
                AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
                Mask: ValueType + SparseVertexVectorForValueType<Mask>,
                EvaluationDomain: ValueType + SparseAdjacencyMatrixForValueType<EvaluationDomain>
            > SelectMaskedEdgesWithTailVertex<AdjacencyMatrix, EvaluationDomain, Mask> for Graph
        where
            SparseMatrix<AdjacencyMatrix>: MatrixMask,
            SparseVector<Mask>: VectorMask,
            SparseMatrix<EvaluationDomain>: MatrixMask,
            VertexVector: SparseVertexVector<EvaluationDomain>
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
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

                let vertex_vector_extract_to =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(extract_to)?;

                let vertex_vector_mask =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(mask)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        tail_vertex,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
                        Mask::sparse_vector_ref(vertex_vector_mask),
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

                let adjacency_matrix_adjacency_matrix = unsafe { &*edge_store }
                    .adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

                let vertex_vector_extract_to = unsafe { &mut *vertex_store }
                    .vertex_vector_mut_ref_by_index_unchecked(extract_to);

                let vertex_vector_mask =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        tail_vertex,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
                        Mask::sparse_vector_ref(vertex_vector_mask),
                        options,
                    )?)
            }

            fn by_key(
                &mut self,
                adjacency_matrix: &EdgeTypeKeyRef,
                tail_vertex: &VertexKeyRef,
                // head_vertex_selector: &VertexSelector,
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
                    .try_index_for_key(tail_vertex)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_row_extractor()
                    .apply(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_adjacency_matrix),
                        head_vertex_index,
                        &VertexSelector::All,
                        accumlator,
                        SparseVertexVector::<EvaluationDomain>::sparse_vector_mut_ref(
                            vertex_vector_extract_to,
                        ),
                        Mask::sparse_vector_ref(vertex_vector_mask),
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
        WeightedDirectedEdgeDefinedByKeysTrait,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operators::add_edge::AddEdge;
    use crate::operators::add_vertex::AddVertex;
    use crate::operators::{
        AddEdgeType, AddVertexType, ReadAdjacencyMatrixElementList, ReadEdgeWeight,
        ReadVertexValue, ReadVertexVectorElementList,
    };

    #[test]
    fn add_adjacency_matrices() {
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

        let vertex_type_1_index = graph.add_new_vertex_type(vertex_type_key).unwrap();
        let vertex_type_2_index = graph.add_new_vertex_type(vertex_result_type_key).unwrap();
        let vertex_1_index = graph.add_new_vertex(vertex_1.clone()).unwrap();
        let vertex_2_index = graph.add_new_vertex(vertex_2.clone()).unwrap();

        let edge_type_1_index = graph.add_new_edge_type(edge_type_1_key).unwrap();
        let edge_type_2_index = graph.add_new_edge_type(edge_type_2_key).unwrap();
        let result_edge_type_index = graph.add_new_edge_type(result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        SelectEdgesWithTailVertex::<u8, isize>::by_key(
            &mut graph,
            &edge_type_1_key,
            vertex_1.key_ref(),
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
                vertex_2.key_ref(),
            )
            .unwrap(),
            Some(1)
        );

        SelectEdgesWithTailVertex::<usize, isize>::by_key(
            &mut graph,
            &edge_type_1_key,
            vertex_2.key_ref(),
            // &VertexSelector::Index(&vec![0]),
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
                vertex_1.key_ref(),
            )
            .unwrap(),
            Some(25)
        );
    }
}
