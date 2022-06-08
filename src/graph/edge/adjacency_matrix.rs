use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::vertex::VertexIndex;

use super::edge_type::{EdgeType, EdgeTypeRef};

use graphblas_sparse_linear_algebra::{
    context::Context,
    operators::{
        element_wise_addition::ElementWiseVectorAdditionMonoidOperator,
        monoid::{Any, LogicalOr},
        options::OperatorOptions,
        reduce::MonoidReducer,
    },
    util::ElementIndex,
    value_types::sparse_matrix::{
        Coordinate, GetMatrixElementList, GetMatrixElementValue, MatrixElement, SetMatrixElement,
        Size, SparseMatrix,
    },
    value_types::sparse_vector::{GetVectorElementList, SparseVector},
};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_OPERATOR_OPTIONS_TRANSPOSE_INPUT0: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new(false, false, false, true, false));
// static GRAPHBLAS_ANY_MONOID: Lazy<Any<bool>> =
//     Lazy::new(|| Any::<bool>::new());

static GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION: Lazy<MonoidReducer<bool>> =
    Lazy::new(|| {
        MonoidReducer::<bool>::new(
            &Any::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            None,
        )
    });

static GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION: Lazy<MonoidReducer<bool>> = Lazy::new(|| {
    MonoidReducer::<bool>::new(
        &Any::<bool>::new(),
        &GRAPHBLAS_OPERATOR_OPTIONS_TRANSPOSE_INPUT0,
        None,
    )
});

static GRAPHBLAS_VECTOR_OR_OPERATOR: Lazy<ElementWiseVectorAdditionMonoidOperator<bool>> =
    Lazy::new(|| {
        ElementWiseVectorAdditionMonoidOperator::<bool>::new(
            &LogicalOr::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            None,
        )
    });

pub type EdgeCoordinate = Coordinate;

#[derive(Clone, Debug)]
pub(crate) struct AdjacencyMatrix {
    edge_type: EdgeType,
    sparse_matrix: SparseMatrix<bool>,
}

// TODO: consider the use of a GraphBLAS iso-matrix (currently not supported by graphblas_sparse_linear_algebra)
impl AdjacencyMatrix {
    pub(crate) fn new(
        graphblas_context: &Arc<Context>,
        edge_type: EdgeType,
        vertex_capacity: ElementIndex,
    ) -> Result<Self, GraphComputingError> {
        let sparse_matrix = SparseMatrix::new(
            &graphblas_context,
            &Size::new(vertex_capacity, vertex_capacity),
        )?;
        Ok(Self {
            edge_type,
            sparse_matrix,
        })
    }

    pub(crate) fn edge_type_ref(&self) -> &EdgeTypeRef {
        &self.edge_type.as_str()
    }

    pub(crate) fn add_edge(
        &mut self,
        coordinate: &EdgeCoordinate,
    ) -> Result<(), GraphComputingError> {
        self.sparse_matrix
            .set_element(MatrixElement::new(*coordinate, true))?;
        Ok(())
    }

    pub(crate) fn graphblas_context_ref(&self) -> &Arc<Context> {
        self.sparse_matrix.context_ref()
    }

    pub(crate) fn delete_edge(
        &mut self,
        coordinate: &EdgeCoordinate,
    ) -> Result<(), GraphComputingError> {
        self.sparse_matrix.drop_element(*coordinate)?;
        Ok(())
    }

    pub(crate) fn is_edge(&self, coordinate: &EdgeCoordinate) -> Result<bool, GraphComputingError> {
        // TODO: change to ref
        Ok(self.sparse_matrix.get_element_value(coordinate)?)
    }

    pub(crate) fn as_sparse_matrix(&self) -> &SparseMatrix<bool> {
        &self.sparse_matrix
    }

    pub(crate) fn as_mut_sparse_matrix(&mut self) -> &mut SparseMatrix<bool> {
        &mut self.sparse_matrix
    }

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    pub(crate) fn get_vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.sparse_matrix.row_height()?)
    }

    // TODO: this probably needs a lifetime, or a clone
    // pub fn size_ref(&self) -> Result<&Size, GraphComputingError> {
    //     Ok(&self.sparse_matrix.size()?)
    // }

    pub(crate) fn resize(
        &mut self,
        target_vertex_capacity: ElementIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(self
            .sparse_matrix
            .resize(&Size::new(target_vertex_capacity, target_vertex_capacity))?)
    }

    pub(crate) fn size(&self) -> Result<Size, GraphComputingError> {
        Ok(self.sparse_matrix.size()?)
    }

    pub(crate) fn number_of_edges(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.sparse_matrix.number_of_stored_elements()?)
    }

    pub(crate) fn get_edge_coordinates(&self) -> Result<Vec<EdgeCoordinate>, GraphComputingError> {
        let matrix_element_list = self.sparse_matrix.get_element_list()?;
        let element_indices_from_vertices = matrix_element_list.row_indices_ref();
        let element_indices_to_vertices = matrix_element_list.column_indices_ref();

        let mut edge_coordinates: Vec<EdgeCoordinate> =
            Vec::with_capacity(matrix_element_list.length());
        for element_index in 0..matrix_element_list.length() {
            let element_coordinate = EdgeCoordinate::new(
                element_indices_from_vertices[element_index],
                element_indices_to_vertices[element_index],
            );
            edge_coordinates.push(element_coordinate);
        }
        Ok(edge_coordinates)
    }

    pub(crate) fn get_from_vertex_indices(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        let from_index_elements = self.get_from_vertex_index_mask()?.get_element_list()?;
        Ok(index_elements_to_vertex_indices(
            from_index_elements.indices_ref().to_vec(),
        ))
    }

    pub(crate) fn get_from_vertex_index_mask(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut from_vertex_vector_mask = SparseVector::new(
            self.sparse_matrix.context_ref(),
            &self.sparse_matrix.row_height()?,
        )?;
        GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION
            .to_vector(&self.sparse_matrix, &mut from_vertex_vector_mask)?;
        Ok(from_vertex_vector_mask)
    }

    pub(crate) fn get_to_vertex_indices(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        let to_index_elements = self.get_to_vertex_index_mask()?.get_element_list()?;
        Ok(index_elements_to_vertex_indices(
            to_index_elements.indices_ref().to_vec(),
        ))
    }

    pub(crate) fn get_to_vertex_index_mask(
        &self,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut to_vertex_vector_mask = SparseVector::new(
            self.sparse_matrix.context_ref(),
            &self.sparse_matrix.row_height()?,
        )?;
        GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION
            .to_vector(&self.sparse_matrix, &mut to_vertex_vector_mask)?;
        Ok(to_vertex_vector_mask)
    }

    ///
    pub(crate) fn get_vertex_indices(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        let index_elements = self.get_vertex_index_mask()?.get_element_list()?;
        Ok(index_elements_to_vertex_indices(
            index_elements.indices_ref().to_vec(),
        ))
    }

    pub(crate) fn get_vertex_index_mask(&self) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut vertex_vector_mask = SparseVector::new(
            self.sparse_matrix.context_ref(),
            &self.sparse_matrix.row_height()?,
        )?;
        GRAPHBLAS_VECTOR_OR_OPERATOR.apply(
            &self.get_to_vertex_index_mask()?,
            &self.get_from_vertex_index_mask()?,
            &mut vertex_vector_mask,
        )?;
        Ok(vertex_vector_mask)
    }
}

fn index_elements_to_vertex_indices(index_elements: Vec<ElementIndex>) -> Vec<VertexIndex> {
    let mut vertex_indices: Vec<VertexIndex> = Vec::with_capacity(index_elements.len());
    for vertex_index in index_elements.into_iter() {
        vertex_indices.push(VertexIndex::new(vertex_index));
    }
    return vertex_indices;
}

impl std::fmt::Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln! {f, "Edge type: {}", self.edge_type};
        return writeln! {f, "Adjancency matrix: {}", self.sparse_matrix};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::context::Mode;
    use graphblas_sparse_linear_algebra::value_types::sparse_vector::GetVectorElementValue;

    #[test]
    fn test_adjacency_matrix_construction() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;

        let edge_type: EdgeType = String::from("Test edge type");

        let adjacency_matrix = AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();
    }

    #[test]
    fn test_basic_operations() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;

        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add = EdgeCoordinate::new(2, 1);
        assert!(!adjacency_matrix.is_edge(&edge_to_add).unwrap());

        adjacency_matrix.add_edge(&edge_to_add).unwrap();
        assert!(adjacency_matrix.is_edge(&edge_to_add).unwrap());

        adjacency_matrix.delete_edge(&edge_to_add).unwrap();
        assert!(!adjacency_matrix.is_edge(&edge_to_add).unwrap());
    }

    #[test]
    fn test_get_edge_coordinates() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;

        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let edge_coordinates = adjacency_matrix.get_edge_coordinates().unwrap();
        assert_eq!(edge_coordinates.len(), 2);
        assert!(edge_coordinates.contains(&edge_to_add_1));
        assert!(edge_coordinates.contains(&edge_to_add_2));
    }

    #[test]
    fn test_vector_index_conversion() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;

        let edge_type: EdgeType = String::from("Test edge type");

        let adjacency_matrix = AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        // let coordinate = Coordinate::new(0, 1);
        // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
        // assert_eq!(vector_index, 10);
        // let retrieved_coordinate =
        //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
        // assert_eq!(retrieved_coordinate, coordinate);

        // let coordinate = Coordinate::new(1, 1);
        // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
        // assert_eq!(vector_index, 11);
        // let retrieved_coordinate =
        //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
        // assert_eq!(retrieved_coordinate, coordinate);

        // let coordinate = Coordinate::new(3, 3);
        // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
        // assert_eq!(vector_index, 33);
        // let retrieved_coordinate =
        //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
        // assert_eq!(retrieved_coordinate, coordinate);

        // let coordinate = Coordinate::new(0, 0);
        // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
        // assert_eq!(vector_index, 0);
        // let retrieved_coordinate =
        //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
        // assert_eq!(retrieved_coordinate, coordinate);
    }

    #[test]
    fn test_get_from_vertex_index_mask() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let from_vertex_index_map = adjacency_matrix.get_from_vertex_index_mask().unwrap();
        assert_eq!(from_vertex_index_map.get_element_value(&0).unwrap(), false);
        assert_eq!(from_vertex_index_map.get_element_value(&1).unwrap(), true);
        assert_eq!(from_vertex_index_map.get_element_value(&2).unwrap(), true);
        assert_eq!(from_vertex_index_map.get_element_value(&3).unwrap(), false);
    }

    #[test]
    fn test_get_from_vertex_indices() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let from_vertex_indices = adjacency_matrix.get_from_vertex_indices().unwrap();
        assert_eq!(from_vertex_indices.len(), 2);
        assert!(from_vertex_indices.contains(&VertexIndex::new(1)));
        assert!(from_vertex_indices.contains(&VertexIndex::new(2)));
    }

    #[test]
    fn test_get_to_vertex_index_mask() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let to_vertex_index_map = adjacency_matrix.get_to_vertex_index_mask().unwrap();
        assert_eq!(to_vertex_index_map.get_element_value(&0).unwrap(), false);
        assert_eq!(to_vertex_index_map.get_element_value(&1).unwrap(), true);
        assert_eq!(to_vertex_index_map.get_element_value(&2).unwrap(), false);
        assert_eq!(to_vertex_index_map.get_element_value(&3).unwrap(), false);
    }

    #[test]
    fn test_get_to_vertex_indices() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let to_vertex_indices = adjacency_matrix.get_to_vertex_indices().unwrap();
        assert_eq!(to_vertex_indices.len(), 1);
        assert!(to_vertex_indices.contains(&VertexIndex::new(1)));
    }

    #[test]
    fn test_get_vertex_index_mask() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();
        println!("{}", adjacency_matrix);

        let vertex_index_map = adjacency_matrix.get_vertex_index_mask().unwrap();
        println!("{}", vertex_index_map);
        assert_eq!(vertex_index_map.get_element_value(&0).unwrap(), false);
        assert_eq!(vertex_index_map.get_element_value(&1).unwrap(), true);
        assert_eq!(vertex_index_map.get_element_value(&2).unwrap(), true);
        assert_eq!(vertex_index_map.get_element_value(&3).unwrap(), false);
    }

    #[test]
    fn test_get_vertex_indices() {
        let context = Context::init_ready(Mode::NonBlocking).unwrap();
        let vertex_capacity = 10;
        let edge_type: EdgeType = String::from("Test edge type");

        let mut adjacency_matrix =
            AdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

        let edge_to_add_1 = EdgeCoordinate::new(2, 1);
        adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

        let edge_to_add_2 = EdgeCoordinate::new(1, 1);
        adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

        let vertex_indices = adjacency_matrix.get_vertex_indices().unwrap();
        assert_eq!(vertex_indices.len(), 2);
        assert!(vertex_indices.contains(&VertexIndex::new(1)));
        assert!(vertex_indices.contains(&VertexIndex::new(2)));
    }
}
