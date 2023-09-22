use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    operators::{
        apply::{ApplyIndexUnaryOperator, IndexUnaryOperatorApplier},
        binary_operator::AccumulatorBinaryOperator,
        index_unary_operator::IndexUnaryOperator,
        options::OperatorOptions,
    },
};

use crate::graph::{
    edge::EdgeTypeKeyRef,
    graph::GraphblasOperatorApplierCollectionTrait,
    value_type::SparseVertexVectorForValueType,
    vertex_store::{type_operations::get_vertex_vector::GetVertexVector, VertexStoreTrait},
};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, VertexTypeIndex},
        value_type::ValueType,
        vertex::vertex::VertexTypeKeyRef,
    },
};
use graphblas_sparse_linear_algebra::operators::mask::VectorMask;

pub trait ApplyIndexUnaryOperatorToVertexVector<VertexVector, Product, EvaluationDomain>
where
    VertexVector: ValueType,
    Product: ValueType,
    EvaluationDomain: ValueType,
{
    fn with_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key(
        &mut self,
        vertex_vector: &VertexTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        VertexVector,
        Product,
        EvaluationDomain,
    > ApplyIndexUnaryOperatorToVertexVector<VertexVector, Product, EvaluationDomain> for Graph
where
    SparseVector<VertexVector>: VectorMask,
    SparseVector<Product>: VectorMask,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>, VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>, Product: ValueType + SparseVertexVectorForValueType<Product>, EvaluationDomain: ValueType
{
    fn with_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
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
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(vertex_vector)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_unchecked_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(vertex_vector);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_key(
        &mut self,
        vertex_vector: &EdgeTypeKeyRef,
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
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(vertex_vector)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait ApplyScalarBinaryOperatorToMaskedVertexVector<
    VertexVector,
    Product,
    EvaluationDomain,
    Mask,
> where
    VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
    SparseVector<VertexVector>: VectorMask,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    SparseVector<Product>: VectorMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    SparseVector<Mask>: VectorMask,
{
    fn with_index_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &VertexTypeKeyRef,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        Mask: ValueType + SparseVertexVectorForValueType<Mask>,
        EvaluationDomain: ValueType,
    > ApplyScalarBinaryOperatorToMaskedVertexVector<VertexVector, Product, EvaluationDomain, Mask>
    for Graph
where
    SparseVector<VertexVector>: VectorMask,
    SparseVector<Product>: VectorMask,
    SparseVector<Mask>: VectorMask,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn with_index_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
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
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(vertex_vector)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_unchecked_index_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(vertex_vector);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_key_defined_vertex_vector_as_vertex_vector_and_mask(
        &mut self,
        vertex_vector: &EdgeTypeKeyRef,
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
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(vertex_vector)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
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
    use crate::operators::read::ReadVertexValue;

    #[test]
    fn add_scalar_to_vertex_vector() {
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

        let _vertex_type_1_index = graph.add_new_vertex_type(vertex_type_key).unwrap();
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index = graph.add_new_edge_type(edge_type_1_key).unwrap();
        let _edge_type_2_index = graph.add_new_edge_type(edge_type_2_key).unwrap();
        let _result_edge_type_index = graph.add_new_edge_type(result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        ApplyIndexUnaryOperatorToVertexVector::<u8, u16, f32>::with_key(
            &mut graph,
            &vertex_type_key,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            vertex_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     WeightedVertexVectorSparseVectorTrait::<u16>::sparse_vector_ref(
        //         graph
        //             .vertex_store_ref()
        //             .vertex_vector_ref_by_key(result_type_key)
        //             .unwrap()
        //     )
        //     .get_element_list()
        //     .unwrap()
        // );

        assert_eq!(
            ReadVertexValue::<u16>::vertex_value_by_key(
                &graph,
                vertex_type_key,
                vertex_2.key_ref()
            )
            .unwrap(),
            Some(1)
        );
    }
}
