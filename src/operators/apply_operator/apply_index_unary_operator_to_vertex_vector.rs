use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyIndexUnaryOperator, IndexUnaryOperatorApplier},
    binary_operator::AccumulatorBinaryOperator,
    index_unary_operator::IndexUnaryOperator,
    options::OperatorOptions,
};

use crate::graph::{
    graph::GraphblasOperatorApplierCollectionTrait,
    vertex_store::{operations::get_vertex_vector::GetVertexVector, VertexStoreTrait},
};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, VertexTypeIndex},
        value_type::ValueType,
    },
};

pub trait ApplyIndexUnaryOperatorToVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> ApplyIndexUnaryOperatorToVertexVector<EvaluationDomain> for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
    EvaluationDomain: ValueType,
{
    fn by_index(
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

        let vertex_vector_argument = unsafe { &*vertex_store }.vertex_vector_ref(vertex_vector)?;

        let vertex_vector_product = unsafe { &mut *vertex_store }.vertex_vector_mut_ref(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                vertex_vector_argument,
                operator,
                argument,
                accumlator,
                vertex_vector_product,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
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
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(vertex_vector);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                vertex_vector_argument,
                operator,
                argument,
                accumlator,
                vertex_vector_product,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait ApplyScalarBinaryOperatorToMaskedVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        vertex_vector: &VertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToMaskedVertexVector<EvaluationDomain>
    for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn by_index(
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

        let vertex_vector_argument = unsafe { &*vertex_store }.vertex_vector_ref(vertex_vector)?;

        let vertex_vector_product = unsafe { &mut *vertex_store }.vertex_vector_mut_ref(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                vertex_vector_argument,
                operator,
                argument,
                accumlator,
                vertex_vector_product,
                vertex_vector_mask,
                options,
            )?)
    }

    fn by_unchecked_index(
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
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(vertex_vector);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .index_unary_operator_applier()
            .apply_to_vector(
                vertex_vector_argument,
                operator,
                argument,
                accumlator,
                vertex_vector_product,
                vertex_vector_mask,
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetVertexValue;

    #[test]
    fn add_scalar_to_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u16>::apply(&mut graph).unwrap();
        let result_edge_type_index = AddEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        ApplyIndexUnaryOperatorToVertexVector::<f32>::by_index(
            &mut graph,
            &vertex_type_1_index,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            &vertex_type_1_index,
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
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_2_index)
                .unwrap(),
            Some(1)
        );
    }
}
