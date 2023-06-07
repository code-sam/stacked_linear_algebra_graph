use graphblas_sparse_linear_algebra::operators::{
    apply::{BinaryOperatorApplier, IndexUnaryOperatorApplier, UnaryOperatorApplier},
    element_wise_addition::{
        ElementWiseMatrixAdditionBinaryOperator, ElementWiseMatrixAdditionMonoidOperator,
        ElementWiseMatrixAdditionSemiringOperator, ElementWiseVectorAdditionBinaryOperator,
        ElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionSemiringOperator,
    },
};

#[derive(Clone, Debug)]
pub(crate) struct GraphblasOperatorApplierCollection {
    binary_operator_applier: BinaryOperatorApplier,
    unary_operator_applier: UnaryOperatorApplier,
    index_unary_operator_applier: IndexUnaryOperatorApplier,

    element_wise_matrix_addition_binary_operator: ElementWiseMatrixAdditionBinaryOperator,
    element_wise_matrix_addition_monoid_operator: ElementWiseMatrixAdditionMonoidOperator,
    element_wise_matrix_addition_semiring_operator: ElementWiseMatrixAdditionSemiringOperator,
    element_wise_vector_addition_binary_operator: ElementWiseVectorAdditionBinaryOperator,
    element_wise_vector_addition_monoid_operator: ElementWiseVectorAdditionMonoidOperator,
    element_wise_vector_addition_semiring_operator: ElementWiseVectorAdditionSemiringOperator,
}

impl GraphblasOperatorApplierCollection {
    pub(crate) fn new() -> Self {
        Self {
            binary_operator_applier: BinaryOperatorApplier::new(),
            unary_operator_applier: UnaryOperatorApplier::new(),
            index_unary_operator_applier: IndexUnaryOperatorApplier::new(),

            element_wise_matrix_addition_binary_operator:
                ElementWiseMatrixAdditionBinaryOperator::new(),
            element_wise_matrix_addition_monoid_operator:
                ElementWiseMatrixAdditionMonoidOperator::new(),
            element_wise_matrix_addition_semiring_operator:
                ElementWiseMatrixAdditionSemiringOperator::new(),
            element_wise_vector_addition_binary_operator:
                ElementWiseVectorAdditionBinaryOperator::new(),
            element_wise_vector_addition_monoid_operator:
                ElementWiseVectorAdditionMonoidOperator::new(),
            element_wise_vector_addition_semiring_operator:
                ElementWiseVectorAdditionSemiringOperator::new(),
        }
    }
}

pub(crate) trait GraphblasOperatorApplierCollectionTrait {
    fn binary_operator_applier(&self) -> &BinaryOperatorApplier;
    fn unary_operator_applier(&self) -> &UnaryOperatorApplier;
    fn index_unary_operator_applier(&self) -> &IndexUnaryOperatorApplier;

    fn element_wise_matrix_addition_binary_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionBinaryOperator;
    fn element_wise_matrix_addition_monoid_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionMonoidOperator;
    fn element_wise_matrix_addition_semiring_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionSemiringOperator;
    fn element_wise_vector_addition_binary_operator(
        &self,
    ) -> &ElementWiseVectorAdditionBinaryOperator;
    fn element_wise_vector_addition_monoid_operator(
        &self,
    ) -> &ElementWiseVectorAdditionMonoidOperator;
    fn element_wise_vector_addition_semiring_operator(
        &self,
    ) -> &ElementWiseVectorAdditionSemiringOperator;
}

impl GraphblasOperatorApplierCollectionTrait for GraphblasOperatorApplierCollection {
    fn binary_operator_applier(&self) -> &BinaryOperatorApplier {
        &self.binary_operator_applier
    }

    fn unary_operator_applier(&self) -> &UnaryOperatorApplier {
        &self.unary_operator_applier
    }

    fn index_unary_operator_applier(&self) -> &IndexUnaryOperatorApplier {
        &self.index_unary_operator_applier
    }

    fn element_wise_matrix_addition_binary_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionBinaryOperator {
        &self.element_wise_matrix_addition_binary_operator
    }

    fn element_wise_matrix_addition_monoid_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionMonoidOperator {
        &self.element_wise_matrix_addition_monoid_operator
    }

    fn element_wise_matrix_addition_semiring_operator(
        &self,
    ) -> &ElementWiseMatrixAdditionSemiringOperator {
        &self.element_wise_matrix_addition_semiring_operator
    }

    fn element_wise_vector_addition_binary_operator(
        &self,
    ) -> &ElementWiseVectorAdditionBinaryOperator {
        &self.element_wise_vector_addition_binary_operator
    }

    fn element_wise_vector_addition_monoid_operator(
        &self,
    ) -> &ElementWiseVectorAdditionMonoidOperator {
        &self.element_wise_vector_addition_monoid_operator
    }

    fn element_wise_vector_addition_semiring_operator(
        &self,
    ) -> &ElementWiseVectorAdditionSemiringOperator {
        &self.element_wise_vector_addition_semiring_operator
    }
}
