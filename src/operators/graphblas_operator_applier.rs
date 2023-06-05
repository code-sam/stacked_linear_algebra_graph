use graphblas_sparse_linear_algebra::operators::apply::{BinaryOperatorApplier, UnaryOperatorApplier, IndexUnaryOperatorApplier};

#[derive(Clone, Debug)]
pub(crate) struct GraphblasOperatorApplierCollection {
    binary_operator_applier: BinaryOperatorApplier,
    unary_operator_applier: UnaryOperatorApplier,
    index_unary_operator_applier: IndexUnaryOperatorApplier
}

impl GraphblasOperatorApplierCollection {
    pub(crate) fn new() -> Self {
        Self {
            binary_operator_applier: BinaryOperatorApplier::new(),
            unary_operator_applier: UnaryOperatorApplier::new(),
            index_unary_operator_applier: IndexUnaryOperatorApplier::new()
        }
    }
}

pub(crate) trait GraphblasOperatorApplierCollectionTrait {
    fn binary_operator_applier(&self) -> &BinaryOperatorApplier;
    fn unary_operator_applier(&self) -> &UnaryOperatorApplier;
    fn index_unary_operator_applier(&self) -> &IndexUnaryOperatorApplier;
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
}