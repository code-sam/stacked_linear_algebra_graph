use graphblas_sparse_linear_algebra::operators::apply::{
    ApplyIndexUnaryOperator, IndexUnaryOperatorApplier,
};
use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollection;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToVertexVector;
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToVertexVectorUnchecked;

impl<EvaluationDomain: ValueType> ApplyIndexUnaryOperatorToVertexVector<EvaluationDomain> for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        vertex_vector: &impl GetVertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_index_unary_operator_to_vertex_vector::<EvaluationDomain>(
            &mut self.public_vertex_store,
            vertex_vector,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<EvaluationDomain: ValueType> ApplyIndexUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        vertex_vector: &impl GetVertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_index_unary_operator_to_vertex_vector_unchecked::<EvaluationDomain>(
            &mut self.public_vertex_store,
            vertex_vector,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn apply_index_unary_operator_to_vertex_vector<EvaluationDomain>(
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    vertex_vector: &impl GetVertexTypeIndex,
    operator: &impl IndexUnaryOperator<EvaluationDomain>,
    argument: &EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    vertex_store.try_vertex_type_index_validity(vertex_vector)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_index_unary_operator_to_vertex_vector_unchecked::<EvaluationDomain>(
        vertex_store,
        vertex_vector,
        operator,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_index_unary_operator_to_vertex_vector_unchecked<EvaluationDomain>(
    vertex_store: *mut impl GetVertexVector,
    vertex_vector: &impl GetVertexTypeIndex,
    operator: &impl IndexUnaryOperator<EvaluationDomain>,
    argument: &EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    let vertex_vector_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(vertex_vector);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
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
        None => {
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
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
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::operators::operators::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operators::read::GetVertexValue;

    #[test]
    fn add_scalar_to_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u16>::apply(&mut graph).unwrap();
        let _result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

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

        ApplyIndexUnaryOperatorToVertexVector::<f32>::apply(
            &mut graph,
            &vertex_type_1_index,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            &vertex_type_1_index,
            None,
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
