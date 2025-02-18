use graphblas_sparse_linear_algebra::operators::apply::ApplyUnaryOperator as ApplyGraphBlasUnaryOperator;
use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::unary_operator::UnaryOperator;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::GraphblasOperatorApplierCollection;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_type::{CheckVertexTypeIndex, GetVertexVector};
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::graph_operators::operator_traits::apply_operator::ApplyUnaryOperatorToVertexVector;
use crate::graph_operators::operator_traits::apply_operator::ApplyUnaryOperatorToVertexVectorUnchecked;

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVector<EvaluationDomain> for Graph {
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_vertex_vector::<EvaluationDomain>(
            &mut self.public_vertex_store,
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

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for Graph
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_vertex_vector_unchecked::<EvaluationDomain>(
            &mut self.public_vertex_store,
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

pub(crate) fn apply_unary_operator_to_vertex_vector<EvaluationDomain>(
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    operator: &impl UnaryOperator<EvaluationDomain>,
    argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    vertex_store.try_vertex_type_index_validity(argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_unary_operator_to_vertex_vector_unchecked::<EvaluationDomain>(
        vertex_store,
        operator,
        argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_unary_operator_to_vertex_vector_unchecked<EvaluationDomain>(
    vertex_store: *mut impl GetVertexVector,
    operator: &impl UnaryOperator<EvaluationDomain>,
    argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let vertex_vector_argument = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(argument);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .unary_operator_applier()
                .apply_to_vector(
                    operator,
                    vertex_vector_argument,
                    accumlator,
                    vertex_vector_product,
                    vertex_vector_mask,
                    options,
                )?)
        }
        None => {
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
                .unary_operator_applier()
                .apply_to_vector(
                    operator,
                    vertex_vector_argument,
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
    use graphblas_sparse_linear_algebra::operators::unary_operator::ColumnIndex;

    use super::*;

    use crate::graph::indexing::GetIndex;
    use crate::graph_operators::operator_traits::new::{NewVertex, NewVertexType};
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn add_scalar_to_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();

        ApplyUnaryOperatorToVertexVector::<i32>::apply(
            &mut graph,
            &ColumnIndex::<i32>::new(),
            &vertex_type_1_index,
            &Assignment::new(),
            &vertex_type_1_index,
            None,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_1_index,)
                .unwrap(),
            Some(vertex_1_index.index() as u16)
        );
    }
}
