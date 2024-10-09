use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorLength;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::GetGraphblasContext,
        indexing::{
            operations::{GeneratePrivateIndex, GeneratePublicIndex, GetValidIndices},
            AssignedIndex, GetAssignedIndexData, VertexTypeIndex,
        },
        value_type::{GetValueTypeIdentifier, ValueType},
        vertex_store::{
            CreateVertexVector, GetVertexElementIndexer, GetVertexTypeIndexer, GetVertexVectors,
            VertexStore, VertexVector,
        },
    },
};

pub(crate) trait AddPublicVertexType<'a, T: ValueType> {
    fn apply(&'a mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}

pub(crate) trait AddPrivateVertexType<'a, T: ValueType> {
    fn apply(&'a mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}
