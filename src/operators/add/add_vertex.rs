use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};

use crate::graph::indexing::{GetAssignedIndexData, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex::{GetVertexIndex, GetVertexValue};
use crate::graph::vertex_store::AddVertex as AddVertexToStore;

pub trait AddVertex<T: ValueType> {
    fn add_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

impl<T> AddVertex<T> for Graph
where
    T: ValueType + SetVectorElementTyped<T> + Copy,
{
    fn add_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError> {
        let new_index = self
            .vertex_store_mut_ref()
            .add_new_public_vertex(vertex_type, value)?;
        match new_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref()
                    .resize_adjacency_matrices(new_vertex_capacity)?;
            }
            None => (),
        }
        Ok(*new_index.index_ref())
    }

    fn add_or_update_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        match self.vertex_store_mut_ref().add_or_update_public_vertex(
            vertex_type,
            vertex_index,
            value,
        )? {
            Some(new_index) => {
                match new_index.new_index_capacity() {
                    Some(new_vertex_capacity) => {
                        self.edge_store_mut_ref()
                            .resize_adjacency_matrices(new_vertex_capacity)?;
                    }
                    None => (),
                }
                Ok(Some(*new_index.index_ref()))
            }
            None => Ok(None),
        }
    }

    fn add_or_update_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        self.add_or_update_vertex(
            vertex.type_index_ref(),
            vertex.index_ref(),
            *vertex.value_ref(),
        )
    }
}

pub(crate) trait AddPrivateVertex<T: ValueType> {
    fn add_private_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_private_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_private_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

impl<T> AddPrivateVertex<T> for Graph
where
    T: ValueType + SetVectorElementTyped<T> + Copy,
{
    fn add_private_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError> {
        let new_index = self
            .vertex_store_mut_ref()
            .add_new_private_vertex(vertex_type, value)?;
        match new_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref()
                    .resize_adjacency_matrices(new_vertex_capacity)?;
            }
            None => (),
        }
        Ok(*new_index.index_ref())
    }

    fn add_or_update_private_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        match self.vertex_store_mut_ref().add_or_update_private_vertex(
            vertex_type,
            vertex_index,
            value,
        )? {
            Some(new_index) => {
                match new_index.new_index_capacity() {
                    Some(new_vertex_capacity) => {
                        self.edge_store_mut_ref()
                            .resize_adjacency_matrices(new_vertex_capacity)?;
                    }
                    None => (),
                }
                Ok(Some(*new_index.index_ref()))
            }
            None => Ok(None),
        }
    }

    fn add_or_update_private_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError> {
        self.add_or_update_private_vertex(
            vertex.type_index_ref(),
            vertex.index_ref(),
            *vertex.value_ref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        graph::vertex::vertex::VertexDefinition,
        operators::{
            add::{AddVertexType, CreateVertexIndex},
            read::GetVertexValue,
        },
    };

    #[test]
    fn add_or_update_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &5, &5).unwrap();

        let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_index = graph.new_vertex_index().unwrap();

        let vertex_value = 1u8;
        let vertex_to_add = VertexDefinition::new(vertex_type_index, vertex_index, vertex_value);

        let _updated_vertex = graph
            .add_or_update_vertex_from_vertex(&vertex_to_add.clone())
            .unwrap();

        let vertex_property_2 = 2u8;

        let vertex_to_add_2 =
            VertexDefinition::new(vertex_type_index, vertex_index, vertex_property_2);

        let updated_vertex_2 = graph
            .add_or_update_vertex_from_vertex(&vertex_to_add_2)
            .unwrap();

        assert_eq!(None, updated_vertex_2);

        let value_2: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index)
            .unwrap();
        assert_eq!(value_2, vertex_property_2);
    }

    #[test]
    fn add_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &5, &5).unwrap();

        let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_value = 1u8;
        let another_vertex_value = 2u8;

        let vertex_index = graph.add_vertex(&vertex_type_index, vertex_value).unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index)
            .unwrap();
        assert_eq!(value, vertex_value);

        let vertex_index_2 = graph
            .add_vertex(&vertex_type_index, another_vertex_value)
            .unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index_2)
            .unwrap();
        assert_eq!(value, another_vertex_value);
    }

    #[test]
    fn add_new_vertex() {
        let mut graph = Graph::with_initial_capacity(&1, &1, &1).unwrap();

        for _i in 0..3 {
            let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

            for i in 0..50 {
                graph.add_vertex(&vertex_type_index, i).unwrap();
            }
        }
    }
}
