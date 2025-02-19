use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::graph::indexing::GetVertexTypeIndex;
use crate::versioned_graph::indexing::GetVersionedVertexTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait GetSparseVertexVectorVersioned<T: ValueType> {
    fn sparse_vector(
        &self,
        type_index: &impl GetVersionedVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError>;
}

pub trait GetVersionedVertexVectorElementListVersioned<T: ValueType> {
    fn sparse_vector_element_list(
        &self,
        type_index: &impl GetVersionedVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
}
