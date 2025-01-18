use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_type::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};

use crate::graph::indexing::{GetAssignedIndexData, GetVertexTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::AddVertex as AddVertexToVertexVector;
use crate::operators::operators::new::NewVertex;

impl<T> NewVertex<T> for Graph
where
    T: ValueType + SetSparseVectorElementTyped<T> + Copy,
{
    fn new_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError> {
        let assigned_vertex_index = self
            .vertex_store_mut_ref()
            .add_new_vertex(vertex_type, value)?;

        match assigned_vertex_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref().resize_adjacency_matrices(new_vertex_capacity)?
            },
            None => {}
        }
        
        Ok(VertexIndex::new(assigned_vertex_index.index()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::operators::operators::{new::NewVertexType, read::GetVertexValue};

    #[test]
    fn add_vertex() {
        let mut graph = Graph::with_initial_capacity(1, 5, 5).unwrap();

        let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_value = 1u8;
        let another_vertex_value = 2u8;

        let vertex_index = graph.new_vertex(&vertex_type_index, vertex_value).unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index)
            .unwrap();
        assert_eq!(value, vertex_value);

        let vertex_index_2 = graph
            .new_vertex(&vertex_type_index, another_vertex_value)
            .unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index_2)
            .unwrap();
        assert_eq!(value, another_vertex_value);
    }

    #[test]
    fn add_new_vertex() {
        let mut graph = Graph::with_initial_capacity(1, 1, 1).unwrap();

        for _i in 0..3 {
            let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

            for i in 0..50 {
                graph.new_vertex(&vertex_type_index, i).unwrap();
            }
        }
    }
}
