use std::sync::Arc;

use graphblas_sparse_linear_algebra::operators::{
    apply::{BinaryOperatorApplier, IndexUnaryOperatorApplier, UnaryOperatorApplier},
    element_wise_addition::{
        ElementWiseMatrixAdditionBinaryOperator, ElementWiseMatrixAdditionMonoidOperator,
        ElementWiseMatrixAdditionSemiringOperator, ElementWiseVectorAdditionBinaryOperator,
        ElementWiseVectorAdditionMonoidOperator, ElementWiseVectorAdditionSemiringOperator,
    },
    element_wise_multiplication::{
        ElementWiseMatrixMultiplicationBinaryOperator,
        ElementWiseMatrixMultiplicationMonoidOperator,
        ElementWiseMatrixMultiplicationSemiringOperator,
        ElementWiseVectorMultiplicationBinaryOperator,
        ElementWiseVectorMultiplicationMonoidOperator,
        ElementWiseVectorMultiplicationSemiringOperator,
    },
    extract::{MatrixColumnExtractor, MatrixRowExtractor, SubMatrixExtractor, SubVectorExtractor},
    mask::{SelectEntireMatrix, SelectEntireVector},
    multiplication::{
        MatrixMultiplicationOperator, MatrixVectorMultiplicationOperator,
        VectorMatrixMultiplicationOperator,
    },
    select::{MatrixSelector, VectorSelector},
    transpose::MatrixTranspose,
};

use crate::graph::graph::GraphblasContext;

pub(crate) trait GetGraphblasOperatorApplierCollection {
    fn graphblas_operator_applier_collection_ref(&self) -> &GraphblasOperatorApplierCollection;
}

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

    element_wise_matrix_multiplication_binary_operator:
        ElementWiseMatrixMultiplicationBinaryOperator,
    element_wise_matrix_multiplication_monoid_operator:
        ElementWiseMatrixMultiplicationMonoidOperator,
    element_wise_matrix_multiplication_semiring_operator:
        ElementWiseMatrixMultiplicationSemiringOperator,
    element_wise_vector_multiplication_binary_operator:
        ElementWiseVectorMultiplicationBinaryOperator,
    element_wise_vector_multiplication_monoid_operator:
        ElementWiseVectorMultiplicationMonoidOperator,
    element_wise_vector_multiplication_semiring_operator:
        ElementWiseVectorMultiplicationSemiringOperator,

    matrix_column_extractor: MatrixColumnExtractor,
    matrix_row_extractor: MatrixRowExtractor,
    sub_matrix_extractor: SubMatrixExtractor,
    sub_vector_extractor: SubVectorExtractor,

    matrix_selector: MatrixSelector,
    vector_selector: VectorSelector,

    matrix_transposer: MatrixTranspose,

    matrix_multiplication_operator: MatrixMultiplicationOperator,
    matrix_vector_multiplication_operator: MatrixVectorMultiplicationOperator,
    vector_matrix_multiplication_operator: VectorMatrixMultiplicationOperator,

    entire_matrix_selector: SelectEntireMatrix,
    entire_vector_selector: SelectEntireVector,
}

impl GraphblasOperatorApplierCollection {
    pub(crate) fn new(context: &Arc<GraphblasContext>) -> Self {
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

            element_wise_matrix_multiplication_binary_operator:
                ElementWiseMatrixMultiplicationBinaryOperator::new(),
            element_wise_matrix_multiplication_monoid_operator:
                ElementWiseMatrixMultiplicationMonoidOperator::new(),
            element_wise_matrix_multiplication_semiring_operator:
                ElementWiseMatrixMultiplicationSemiringOperator::new(),
            element_wise_vector_multiplication_binary_operator:
                ElementWiseVectorMultiplicationBinaryOperator::new(),
            element_wise_vector_multiplication_monoid_operator:
                ElementWiseVectorMultiplicationMonoidOperator::new(),
            element_wise_vector_multiplication_semiring_operator:
                ElementWiseVectorMultiplicationSemiringOperator::new(),

            matrix_column_extractor: MatrixColumnExtractor::new(),
            matrix_row_extractor: MatrixRowExtractor::new(),
            sub_matrix_extractor: SubMatrixExtractor::new(),
            sub_vector_extractor: SubVectorExtractor::new(),

            matrix_selector: MatrixSelector::new(),
            vector_selector: VectorSelector::new(),

            matrix_transposer: MatrixTranspose::new(),

            matrix_multiplication_operator: MatrixMultiplicationOperator::new(),
            matrix_vector_multiplication_operator: MatrixVectorMultiplicationOperator::new(),
            vector_matrix_multiplication_operator: VectorMatrixMultiplicationOperator::new(),

            entire_matrix_selector: SelectEntireMatrix::new(context),
            entire_vector_selector: SelectEntireVector::new(context),
        }
    }
}

pub(crate) trait GetGraphblasOperatorAppliers {
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

    fn element_wise_matrix_multiplication_binary_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationBinaryOperator;
    fn element_wise_matrix_multiplication_monoid_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationMonoidOperator;
    fn element_wise_matrix_multiplication_semiring_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationSemiringOperator;
    fn element_wise_vector_multiplication_binary_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationBinaryOperator;
    fn element_wise_vector_multiplication_monoid_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationMonoidOperator;
    fn element_wise_vector_multiplication_semiring_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationSemiringOperator;

    fn matrix_column_extractor(&self) -> &MatrixColumnExtractor;
    fn matrix_row_extractor(&self) -> &MatrixRowExtractor;
    fn sub_matrix_extractor(&self) -> &SubMatrixExtractor;
    fn sub_vector_extractor(&self) -> &SubVectorExtractor;

    fn matrix_selector(&self) -> &MatrixSelector;
    fn vector_selector(&self) -> &VectorSelector;

    fn matrix_transposer(&self) -> &MatrixTranspose;

    fn matrix_multiplication_operator(&self) -> &MatrixMultiplicationOperator;
    fn matrix_vector_multiplication_operator(&self) -> &MatrixVectorMultiplicationOperator;
    fn vector_matrix_multiplication_operator(&self) -> &VectorMatrixMultiplicationOperator;

    fn entire_matrix_selector(&self) -> &SelectEntireMatrix;
    fn entire_vector_selector(&self) -> &SelectEntireVector;
}

impl GetGraphblasOperatorAppliers for GraphblasOperatorApplierCollection {
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

    fn element_wise_matrix_multiplication_binary_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationBinaryOperator {
        &self.element_wise_matrix_multiplication_binary_operator
    }
    fn element_wise_matrix_multiplication_monoid_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationMonoidOperator {
        &self.element_wise_matrix_multiplication_monoid_operator
    }
    fn element_wise_matrix_multiplication_semiring_operator(
        &self,
    ) -> &ElementWiseMatrixMultiplicationSemiringOperator {
        &self.element_wise_matrix_multiplication_semiring_operator
    }
    fn element_wise_vector_multiplication_binary_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationBinaryOperator {
        &self.element_wise_vector_multiplication_binary_operator
    }
    fn element_wise_vector_multiplication_monoid_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationMonoidOperator {
        &self.element_wise_vector_multiplication_monoid_operator
    }
    fn element_wise_vector_multiplication_semiring_operator(
        &self,
    ) -> &ElementWiseVectorMultiplicationSemiringOperator {
        &self.element_wise_vector_multiplication_semiring_operator
    }

    fn matrix_column_extractor(&self) -> &MatrixColumnExtractor {
        &self.matrix_column_extractor
    }
    fn matrix_row_extractor(&self) -> &MatrixRowExtractor {
        &self.matrix_row_extractor
    }
    fn sub_matrix_extractor(&self) -> &SubMatrixExtractor {
        &self.sub_matrix_extractor
    }
    fn sub_vector_extractor(&self) -> &SubVectorExtractor {
        &self.sub_vector_extractor
    }

    fn matrix_selector(&self) -> &MatrixSelector {
        &self.matrix_selector
    }

    fn vector_selector(&self) -> &VectorSelector {
        &self.vector_selector
    }

    fn matrix_transposer(&self) -> &MatrixTranspose {
        &self.matrix_transposer
    }

    fn matrix_multiplication_operator(&self) -> &MatrixMultiplicationOperator {
        &self.matrix_multiplication_operator
    }

    fn matrix_vector_multiplication_operator(&self) -> &MatrixVectorMultiplicationOperator {
        &self.matrix_vector_multiplication_operator
    }

    fn vector_matrix_multiplication_operator(&self) -> &VectorMatrixMultiplicationOperator {
        &self.vector_matrix_multiplication_operator
    }

    fn entire_matrix_selector(&self) -> &SelectEntireMatrix {
        &self.entire_matrix_selector
    }

    fn entire_vector_selector(&self) -> &SelectEntireVector {
        &self.entire_vector_selector
    }
}
