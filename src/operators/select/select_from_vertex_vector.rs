use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::select::{SelectFromVector, VectorSelector};

use crate::graph::graph::{
    GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers, GetVertexStore, Graph,
};
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait SelectFromVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait SelectFromVertexVectorUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> SelectFromVertexVector<EvaluationDomain> for Graph
where
    VectorSelector: SelectFromVector<EvaluationDomain>,
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        SelectFromVertexVectorUnchecked::apply(
            self,
            selector,
            selector_argument,
            argument,
            accumlator,
            product,
            mask,
            options,
        )
    }
}

impl<EvaluationDomain> SelectFromVertexVectorUnchecked<EvaluationDomain> for Graph
where
    VectorSelector: SelectFromVector<EvaluationDomain>,
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product);

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .vector_selector()
                    .apply(
                        selector,
                        selector_argument,
                        vertex_vector_argument,
                        accumlator,
                        vertex_vector_product,
                        vertex_vector_mask,
                        options,
                    )?)
            }
            None => {
                let vertex_vector_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_vector_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .vector_selector()
                    .apply(
                        selector,
                        selector_argument,
                        vertex_vector_argument,
                        accumlator,
                        vertex_vector_product,
                        vertex_vector_mask,
                        options,
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::operators::add::{AddVertex, AddVertexType};
    use crate::operators::read::GetVertexValue;

    #[test]
    fn select_from_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let _vertex_result_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        SelectFromVertexVector::<u8>::by_index(
            &mut graph,
            &IsValueGreaterThan::<u8>::new(),
            &1,
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
            None
        );

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_2_index,)
                .unwrap(),
            Some(2)
        );
    }
}
