use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementListTyped, GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::operators::monoid::AnyMonoidTyped;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::edge::{EdgeTypeIndex, GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::{
    AddEdge as AddEdgeToAdjacencyMatrix, Indexing,
};
use crate::graph::edge_store::weighted_adjacency_matrix::IntoSparseMatrixForValueType;
use crate::graph::graph::{GetEdgeStore, Graph, VertexIndex};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::operators::indexing::Indexing as GraphIndexing;

pub trait AddEdge<T: ValueType> {
    fn add_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_edge(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> AddEdge<T> for Graph
where
    T: ValueType
        + IntoSparseMatrixForValueType<T>
        + GetSparseMatrixElementListTyped<T>
        + GetSparseMatrixElementValueTyped<T>
        + GetValueTypeIdentifier
        + AnyMonoidTyped<T>
        + SetSparseMatrixElementTyped<T>
        + Default
        + Copy,
{
    fn add_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError> {
        AddEdge::<T>::add_edge(
            self,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            *edge.weight_ref(),
        )
    }

    fn add_edge(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_index_validity(tail)?;
        self.try_vertex_index_validity(head)?;

        let adjacency_matrix = self
            .edge_store_mut_ref()
            .try_adjacency_matrix_mut_ref(edge_type)?;

        if Indexing::<T>::is_edge(adjacency_matrix, tail, head)? {
            return Err(LogicError::new(
                LogicErrorType::EdgeAlreadyExists,
                format!(
                    "An edge already existis for: [edge_type: {:?}, tail: {:?}, head: {:?}]",
                    edge_type, tail, head
                ),
                None,
            )
            .into());
        }

        adjacency_matrix.add_edge_unchecked(tail, head, weight)?;
        Ok(())
    }

    fn add_or_replace_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError> {
        AddEdge::<T>::add_edge(
            self,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            *edge.weight_ref(),
        )
    }

    fn add_or_replace_edge(
        &mut self,
        edge_type: &EdgeTypeIndex,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_index_validity(tail)?;
        self.try_vertex_index_validity(head)?;

        let adjacency_matrix = self
            .edge_store_mut_ref()
            .try_adjacency_matrix_mut_ref(edge_type)?;

        adjacency_matrix.add_edge_unchecked(tail, head, weight)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::graph::graph::Graph;
    use crate::graph::vertex::vertex::{GetVertexIndex, VertexDefinition};
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertexType, CreateVertexIndex};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn add_edge() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_type_index_2 = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex1_index = graph.new_vertex_index().unwrap();
        let vertex2_index = graph.new_vertex_index().unwrap();

        let vertex_1 = VertexDefinition::new(vertex_type_index, vertex1_index, 1u8);
        let vertex_2 = VertexDefinition::new(vertex_type_index, vertex2_index, 2u8);

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();

        let edge_vertex1_vertex2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_1_index,
                *vertex_1.index_ref(),
                *vertex_2.index_ref(),
            ),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_1_index,
                *vertex_2.index_ref(),
                *vertex_1.index_ref(),
            ),
            2u8,
        );

        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_2_index,
                *vertex_1.index_ref(),
                *vertex_2.index_ref(),
            ),
            3u8,
        );
        let edge_vertex2_vertex1_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                vertex_type_index_2,
                *vertex_2.index_ref(),
                *vertex_1.index_ref(),
            ),
            4u8,
        );

        graph.add_edge_from_edge(edge_vertex1_vertex2).unwrap();
        graph.add_edge_from_edge(edge_vertex2_vertex1).unwrap();
        graph
            .add_edge_from_edge(edge_vertex1_vertex2_type_2)
            .unwrap();
        graph
            .add_edge_from_edge(edge_vertex2_vertex1_type_2)
            .unwrap();

        assert_eq!(
            GetEdgeWeight::<u8>::try_apply(
                &graph,
                &edge_type_1_index,
                vertex_1.index_ref(),
                vertex_2.index_ref()
            )
            .unwrap(),
            1u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_apply_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_1_index,
                    *vertex_2.index_ref(),
                    *vertex_1.index_ref()
                )
            )
            .unwrap(),
            2u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_apply_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_2_index,
                    *vertex_1.index_ref(),
                    *vertex_2.index_ref()
                )
            )
            .unwrap(),
            3u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_apply_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_2_index,
                    *vertex_2.index_ref(),
                    *vertex_1.index_ref()
                )
            )
            .unwrap(),
            4u8
        );
    }
}
