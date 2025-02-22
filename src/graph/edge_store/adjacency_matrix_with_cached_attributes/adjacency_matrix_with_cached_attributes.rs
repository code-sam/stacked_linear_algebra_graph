use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::{
    CachedAdjacencyMatrixAttributes, GetAdjacencyMatrixTranspose,
    InvalidateChachedAdjacencyMatrixAttributes,
};
use crate::graph::edge_store::weighted_adjacency_matrix::{
    CreateWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
};
use crate::graph::indexing::ElementCount;
use crate::graph::value_type::{
    GetValueTypeIdentifier, GetValueTypeIdentifierRef, ValueType, ValueTypeIdentifier,
};

#[derive(Clone, Debug)]
pub(crate) struct WeightedAdjacencyMatrixWithCachedAttributes {
    adjacency_matrix: WeightedAdjacencyMatrix,
    cached_attributes: CachedAdjacencyMatrixAttributes,
}

pub(crate) trait CreateWeightedAdjacencyMatrixWithCachedAttributes<T> {
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> CreateWeightedAdjacencyMatrixWithCachedAttributes<T>
    for WeightedAdjacencyMatrixWithCachedAttributes
{
    fn new(
        graphblas_context: Arc<GraphBLASContext>,
        initial_vertex_capacity: ElementCount,
    ) -> Result<WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError> {
        let adjacency_matrix = <WeightedAdjacencyMatrix as CreateWeightedAdjacencyMatrix<T>>::new(
            graphblas_context.clone(),
            initial_vertex_capacity,
        )?;
        let cached_attributes = CachedAdjacencyMatrixAttributes::new(graphblas_context);

        Ok(WeightedAdjacencyMatrixWithCachedAttributes {
            adjacency_matrix,
            cached_attributes,
        })
    }
}

pub(crate) trait GetWeightedAdjacencyMatrix {
    fn weighted_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix;
    fn weighted_adjacency_matrix_mut_ref(&mut self) -> &mut WeightedAdjacencyMatrix;
}

pub(crate) trait GetCachedAttributesOfAdjacencyMatrix {
    fn weighted_adjacency_matrix_cached_attributes_ref(&self) -> &CachedAdjacencyMatrixAttributes;
    fn transposed_weighted_adjacency_matrix_ref(
        &mut self,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
}

impl GetWeightedAdjacencyMatrix for WeightedAdjacencyMatrixWithCachedAttributes {
    fn weighted_adjacency_matrix_ref(&self) -> &WeightedAdjacencyMatrix {
        &self.adjacency_matrix
    }

    fn weighted_adjacency_matrix_mut_ref(&mut self) -> &mut WeightedAdjacencyMatrix {
        self.cached_attributes.invalidate_all_attributes();
        &mut self.adjacency_matrix
    }
}

impl GetCachedAttributesOfAdjacencyMatrix for WeightedAdjacencyMatrixWithCachedAttributes {
    fn weighted_adjacency_matrix_cached_attributes_ref(&self) -> &CachedAdjacencyMatrixAttributes {
        &self.cached_attributes
    }

    fn transposed_weighted_adjacency_matrix_ref(
        &mut self,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.cached_attributes.transpose_ref(&self.adjacency_matrix)
    }

    // fn weighted_adjacency_matrix_cached_attributes_mut_ref(&mut self) -> &mut CachedAdjacencyMatrixAttributes {
    //     todo!()
    // }
}

impl GetValueTypeIdentifierRef for WeightedAdjacencyMatrixWithCachedAttributes {
    fn value_type_identifier_ref(&self) -> &ValueTypeIdentifier {
        &self.adjacency_matrix.value_type_identifier_ref()
    }
}

impl InvalidateChachedAdjacencyMatrixAttributes for WeightedAdjacencyMatrixWithCachedAttributes {
    fn invalidate_all_attributes(&mut self) {
        self.cached_attributes.invalidate_all_attributes();
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::context::Context;

    use crate::graph::{
        edge_store::weighted_adjacency_matrix::operations::{GetEdgeWeight, SetEdge},
        indexing::VertexIndex,
    };

    use super::*;

    #[test]
    fn cached_adjacency_matrix_transpose() {
        let context = Context::init_default().unwrap();

        let mut adjacency_matrix = <WeightedAdjacencyMatrixWithCachedAttributes as CreateWeightedAdjacencyMatrixWithCachedAttributes<u32>>::new(context.clone(), 10)
        .unwrap();

        adjacency_matrix
            .weighted_adjacency_matrix_mut_ref()
            .set_edge_unchecked(&VertexIndex::new(0), &VertexIndex::new(0), 1e3)
            .unwrap();
        adjacency_matrix
            .weighted_adjacency_matrix_mut_ref()
            .set_edge_unchecked(&VertexIndex::new(1), &VertexIndex::new(0), 2e3)
            .unwrap();

        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                adjacency_matrix
                    .transposed_weighted_adjacency_matrix_ref()
                    .unwrap(),
                &VertexIndex::new(0),
                &VertexIndex::new(0)
            )
            .unwrap()
            .unwrap(),
            1000u32
        );

        adjacency_matrix
            .weighted_adjacency_matrix_mut_ref()
            .set_edge_unchecked(&VertexIndex::new(1), &VertexIndex::new(0), 2e2)
            .unwrap();

        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                adjacency_matrix
                    .transposed_weighted_adjacency_matrix_ref()
                    .unwrap(),
                &VertexIndex::new(0),
                &VertexIndex::new(1)
            )
            .unwrap()
            .unwrap(),
            200u32
        );

        assert_eq!(
            GetEdgeWeight::<u32>::edge_weight_unchecked(
                adjacency_matrix
                    .transposed_weighted_adjacency_matrix_ref()
                    .unwrap(),
                &VertexIndex::new(1),
                &VertexIndex::new(0)
            )
            .unwrap(),
            None
        );
    }
}
