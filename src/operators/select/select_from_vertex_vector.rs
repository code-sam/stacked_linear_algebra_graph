use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::select::{SelectFromVector, VectorSelector};
use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    operators::{
        binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
        mask::VectorMask,
        options::OperatorOptions,
    },
};

use crate::graph::graph::{Graph, VertexTypeIndex};
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        value_type::{
            implement_macro_for_all_native_value_types, SparseVertexVectorForValueType, ValueType,
        },
        vertex::VertexTypeKeyRef,
    },
};

pub trait SelectFromVertexVector<Argument, Product, EvaluationDomain>
where
    Argument: ValueType + SparseVertexVectorForValueType<Argument>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
    SparseVector<Argument>: VectorMask,
    SparseVector<Product>: VectorMask,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        Argument: ValueType + SparseVertexVectorForValueType<Argument>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        EvaluationDomain: ValueType,
    > SelectFromVertexVector<Argument, Product, EvaluationDomain> for Graph
where
    SparseVector<Argument>: VectorMask,
    SparseVector<Product>: VectorMask,
    VectorSelector: SelectFromVector<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait MaskedVectorSelector<Argument, Product, EvaluationDomain, Mask>
where
    Argument: ValueType + SparseVertexVectorForValueType<Argument>,
    SparseVector<Argument>: VectorMask,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    SparseVector<Product>: VectorMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    SparseVector<Mask>: VectorMask,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        Argument: ValueType + SparseVertexVectorForValueType<Argument>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        EvaluationDomain: ValueType,
        Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    > MaskedVectorSelector<Argument, Product, EvaluationDomain, Mask> for Graph
where
    SparseVector<Argument>: VectorMask,
    SparseVector<Product>: VectorMask,
    SparseVector<Mask>: VectorMask,
    VectorSelector: SelectFromVector<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .vector_selector()
            .apply(
                selector,
                selector_argument,
                Argument::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
        WeightedDirectedEdgeDefinedByKeysTrait,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operators::add_edge::AddEdge;
    use crate::operators::add_vertex::AddVertex;
    use crate::operators::{AddEdgeType, AddVertexType, ReadEdgeWeight, ReadVertexValue};

    #[test]
    fn add_adjacency_matrices() {
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

        let vertex_type_1_index = graph.add_new_vertex_type(vertex_type_key).unwrap();
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

        SelectFromVertexVector::<u8, u16, u8>::by_key(
            &mut graph,
            &IsValueGreaterThan::<u8>::new(),
            &1,
            vertex_type_key,
            &Assignment::new(),
            vertex_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            ReadVertexValue::<u16>::vertex_value_by_key(&graph, vertex_type_key, "vertex_1",)
                .unwrap(),
            None
        );

        assert_eq!(
            ReadVertexValue::<u16>::vertex_value_by_key(&graph, vertex_type_key, "vertex_2",)
                .unwrap(),
            Some(2)
        );
    }
}
