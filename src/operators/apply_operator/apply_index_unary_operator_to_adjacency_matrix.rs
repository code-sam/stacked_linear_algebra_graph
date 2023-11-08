use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        apply::{ApplyIndexUnaryOperator, IndexUnaryOperatorApplier},
        binary_operator::AccumulatorBinaryOperator,
        index_unary_operator::IndexUnaryOperator,
        mask::MatrixMask,
        options::OperatorOptions,
    },
};

use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;

use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::{
    edge::EdgeTypeKeyRef, edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix,
};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{EdgeTypeIndex, Graph},
        value_type::ValueType,
        vertex::vertex::VertexTypeKeyRef,
    },
};

pub trait ApplyIndexUnaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, EvaluationDomain>
where
    AdjacencyMatrix: ValueType,
    Product: ValueType,
    EvaluationDomain: ValueType,
{
    fn with_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key(
        &mut self,
        adjacency_matrix: &VertexTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<AdjacencyMatrix, Product, EvaluationDomain>
    ApplyIndexUnaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, EvaluationDomain> for Graph
where
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
    AdjacencyMatrix: ValueType,
    Product: ValueType,
    EvaluationDomain: ValueType,
{
    fn with_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_unchecked_index(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_key(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(adjacency_matrix)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }
}

pub trait ApplyScalarBinaryOperatorToMaskedAdjacencyMatrix<
    AdjacencyMatrix,
    Product,
    EvaluationDomain,
    Mask,
> where
    AdjacencyMatrix: ValueType,
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    Product: ValueType,
    SparseMatrix<Product>: MatrixMask,
    EvaluationDomain: ValueType,
    Mask: ValueType,
    SparseMatrix<Mask>: MatrixMask,
{
    fn with_index_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &VertexTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        AdjacencyMatrix: ValueType,
        Product: ValueType,
        Mask: ValueType,
        EvaluationDomain: ValueType,
    >
    ApplyScalarBinaryOperatorToMaskedAdjacencyMatrix<
        AdjacencyMatrix,
        Product,
        EvaluationDomain,
        Mask,
    > for Graph
where
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    SparseMatrix<Mask>: MatrixMask,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn with_index_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(adjacency_matrix)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn with_unchecked_index_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &EdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(adjacency_matrix);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn with_key_defined_adjacency_matrix_as_adjacency_matrix_and_mask(
        &mut self,
        adjacency_matrix: &EdgeTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        mask: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(adjacency_matrix)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_matrix(
                adjacency_matrix_argument,
                operator,
                argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };

    use crate::graph::vertex::vertex_defined_by_key::{
        VertexDefinedByKey, VertexDefinedByKeyTrait,
    };
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadEdgeWeight;

    #[test]
    fn add_scalar_to_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_key = "vertex_type";
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
            2u8,
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

        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_1_key).unwrap();
        let _edge_type_2_index =
            AddEdgeType::<u16>::add_new_edge_type(&mut graph, edge_type_2_key).unwrap();
        let _result_edge_type_index =
            AddEdgeType::<f32>::add_new_edge_type(&mut graph, result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        ApplyIndexUnaryOperatorToAdjacencyMatrix::<u8, u16, f32>::with_key(
            &mut graph,
            &edge_type_1_key,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     WeightedAdjacencyMatrixSparseMatrixTrait::<u16>::sparse_matrix_ref(
        //         graph
        //             .edge_store_ref()
        //             .adjacency_matrix_ref_for_key(result_type_key)
        //             .unwrap()
        //     )
        //     .get_element_list()
        //     .unwrap()
        // );

        assert_eq!(
            ReadEdgeWeight::<u16>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_2.key_ref(),
                    vertex_1.key_ref(),
                ),
            )
            .unwrap(),
            Some(1)
        );
    }
}
