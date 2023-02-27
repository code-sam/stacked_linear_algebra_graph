use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::Collection;
use graphblas_sparse_linear_algebra::context::ContextTrait;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionMonoidOperator;
use graphblas_sparse_linear_algebra::operators::reduce::MonoidVectorReducer;
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::graph::vertex::VertexIndex;

use crate::graph::edge::{EdgeTypeIndex, EdgeTypeKey, EdgeTypeKeyRef};

use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::{
        Coordinate, GetMatrixElementList, GetMatrixElementValue, MatrixElement, SetMatrixElement,
        Size, SparseMatrix,
    },
    collections::sparse_vector::{GetVectorElementList, SparseVector},
    context::Context,
    index::ElementIndex,
    operators::{
        element_wise_addition::ElementWiseVectorAdditionMonoidOperator,
        monoid::{Any, LogicalOr},
        options::OperatorOptions,
        reduce::MonoidReducer,
    },
};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_OPERATOR_OPTIONS_TRANSPOSE_INPUT0: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new(false, false, false, true, false));
// static GRAPHBLAS_ANY_MONOID: Lazy<Any<bool>> =
//     Lazy::new(|| Any::<bool>::new());

// static GRAPHBLAS_ANY_OPERATOR_IN_HORIZONTAL_DIRECTION: Lazy<MonoidReducer<ValueType, ValueType>> =
//     Lazy::new(|| {
//         MonoidReducer::<ValueType, ValueType>::new(
//             &Any::<ValueType>::new(),
//             &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
//             None,
//         )
//     });

// static GRAPHBLAS_ANY_OPERATOR_IN_VERTICAL_DIRECTION: Lazy<MonoidReducer<bool, bool>> = Lazy::new(|| {
//     MonoidReducer::<bool, bool>::new(
//         &Any::<bool>::new(),
//         &GRAPHBLAS_OPERATOR_OPTIONS_TRANSPOSE_INPUT0,
//         None,
//     )
// });

// static GRAPHBLAS_VECTOR_OR_OPERATOR: Lazy<ElementWiseVectorAdditionMonoidOperator<bool>> =
//     Lazy::new(|| {
//         ElementWiseVectorAdditionMonoidOperator::<bool>::new(
//             &LogicalOr::<bool>::new(),
//             &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
//             None,
//         )
//     });

#[derive(Clone, Debug)]
pub(crate) struct WeightedAdjacencyMatrix<T: ValueType> {
    edge_type: EdgeTypeKey,
    sparse_matrix: SparseMatrix<T>,
}

impl<T: ValueType> WeightedAdjacencyMatrix<T> {
    pub(crate) fn new(
        graphblas_context: &Arc<Context>,
        edge_type: EdgeTypeKey,
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
}

pub(crate) trait WeightedAdjacencyMatrixTrait<T: ValueType> {
    fn edge_type_ref(&self) -> &EdgeTypeKeyRef;
    fn graphblas_context_ref(&self) -> &Arc<Context>;

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn get_vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError>;

    // TODO: this probably needs a lifetime, or a clone
    // pub fn size_ref(&self) -> Result<&Size, GraphComputingError>;

    fn resize(&mut self, target_vertex_capacity: ElementIndex) -> Result<(), GraphComputingError>;
    fn size(&self) -> Result<Size, GraphComputingError>;
    fn number_of_edges(&self) -> Result<ElementIndex, GraphComputingError>;

    fn sparse_matrix_ref(&self) -> &SparseMatrix<T>;
    fn sparse_matrix_mut_ref(&mut self) -> &mut SparseMatrix<T>;
}

impl<T: ValueType> WeightedAdjacencyMatrixTrait<T> for WeightedAdjacencyMatrix<T> {
    fn edge_type_ref(&self) -> &EdgeTypeKeyRef {
        &self.edge_type.as_str()
    }

    fn graphblas_context_ref(&self) -> &Arc<Context> {
        self.sparse_matrix.context_ref()
    }

    // The API suggests a design problem. Returning a ref would be safer, but technically not possible.
    fn get_vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.sparse_matrix.row_height()?)
    }

    // TODO: this probably needs a lifetime, or a clone
    // pub fn size_ref(&self) -> Result<&Size, GraphComputingError> {
    //     Ok(&self.sparse_matrix.size()?)
    // }

    fn resize(&mut self, target_vertex_capacity: ElementIndex) -> Result<(), GraphComputingError> {
        Ok(self
            .sparse_matrix
            .resize(&Size::new(target_vertex_capacity, target_vertex_capacity))?)
    }

    fn size(&self) -> Result<Size, GraphComputingError> {
        Ok(self.sparse_matrix.size()?)
    }

    fn number_of_edges(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.sparse_matrix.number_of_stored_elements()?)
    }

    fn sparse_matrix_ref(&self) -> &SparseMatrix<T> {
        &self.sparse_matrix
    }

    fn sparse_matrix_mut_ref(&mut self) -> &mut SparseMatrix<T> {
        &mut self.sparse_matrix
    }
}

macro_rules! implement_display {
    ($value_type:ty) => {
        impl std::fmt::Display for WeightedAdjacencyMatrix<$value_type> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                writeln! {f, "Edge type: {}", self.edge_type};
                return writeln! {f, "Adjancency matrix: {}", self.sparse_matrix};
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_display);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use graphblas_sparse_linear_algebra::collections::sparse_vector::GetVectorElementValue;
//     use graphblas_sparse_linear_algebra::context::Mode;

//     #[test]
//     fn test_adjacency_matrix_construction() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;

//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();
//     }

//     #[test]
//     fn test_basic_operations() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;

//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add = EdgeCoordinate::new(2, 1);
//         assert!(!adjacency_matrix.is_edge(&edge_to_add).unwrap());

//         adjacency_matrix.add_edge(&edge_to_add).unwrap();
//         assert!(adjacency_matrix.is_edge(&edge_to_add).unwrap());

//         adjacency_matrix.delete_edge(&edge_to_add).unwrap();
//         assert!(!adjacency_matrix.is_edge(&edge_to_add).unwrap());
//     }

//     #[test]
//     fn test_get_edge_coordinates() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;

//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let edge_coordinates = adjacency_matrix.get_edge_coordinates().unwrap();
//         assert_eq!(edge_coordinates.len(), 2);
//         assert!(edge_coordinates.contains(&edge_to_add_1));
//         assert!(edge_coordinates.contains(&edge_to_add_2));
//     }

//     #[test]
//     fn test_vector_index_conversion() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;

//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         // let coordinate = Coordinate::new(0, 1);
//         // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
//         // assert_eq!(vector_index, 10);
//         // let retrieved_coordinate =
//         //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
//         // assert_eq!(retrieved_coordinate, coordinate);

//         // let coordinate = Coordinate::new(1, 1);
//         // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
//         // assert_eq!(vector_index, 11);
//         // let retrieved_coordinate =
//         //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
//         // assert_eq!(retrieved_coordinate, coordinate);

//         // let coordinate = Coordinate::new(3, 3);
//         // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
//         // assert_eq!(vector_index, 33);
//         // let retrieved_coordinate =
//         //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
//         // assert_eq!(retrieved_coordinate, coordinate);

//         // let coordinate = Coordinate::new(0, 0);
//         // let vector_index = adjacency_matrix.convert_coordinate_to_vector_index(coordinate);
//         // assert_eq!(vector_index, 0);
//         // let retrieved_coordinate =
//         //     adjacency_matrix.convert_vector_index_to_coordinate(vector_index);
//         // assert_eq!(retrieved_coordinate, coordinate);
//     }

//     #[test]
//     fn test_get_from_vertex_index_mask() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let from_vertex_index_map = adjacency_matrix.get_from_vertex_index_mask().unwrap();
//         assert_eq!(
//             from_vertex_index_map
//                 .get_element_value_or_default(&0)
//                 .unwrap(),
//             false
//         );
//         assert_eq!(
//             from_vertex_index_map
//                 .get_element_value_or_default(&1)
//                 .unwrap(),
//             true
//         );
//         assert_eq!(
//             from_vertex_index_map
//                 .get_element_value_or_default(&2)
//                 .unwrap(),
//             true
//         );
//         assert_eq!(
//             from_vertex_index_map
//                 .get_element_value_or_default(&3)
//                 .unwrap(),
//             false
//         );
//     }

//     #[test]
//     fn test_get_from_vertex_indices() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let from_vertex_indices = adjacency_matrix.get_from_vertex_indices().unwrap();
//         assert_eq!(from_vertex_indices.len(), 2);
//         assert!(from_vertex_indices.contains(&VertexIndex::new(1)));
//         assert!(from_vertex_indices.contains(&VertexIndex::new(2)));
//     }

//     #[test]
//     fn test_get_to_vertex_index_mask() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let to_vertex_index_map = adjacency_matrix.get_to_vertex_index_mask().unwrap();
//         assert_eq!(
//             to_vertex_index_map
//                 .get_element_value_or_default(&0)
//                 .unwrap(),
//             false
//         );
//         assert_eq!(
//             to_vertex_index_map
//                 .get_element_value_or_default(&1)
//                 .unwrap(),
//             true
//         );
//         assert_eq!(
//             to_vertex_index_map
//                 .get_element_value_or_default(&2)
//                 .unwrap(),
//             false
//         );
//         assert_eq!(
//             to_vertex_index_map
//                 .get_element_value_or_default(&3)
//                 .unwrap(),
//             false
//         );
//     }

//     #[test]
//     fn test_get_to_vertex_indices() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let to_vertex_indices = adjacency_matrix.get_to_vertex_indices().unwrap();
//         assert_eq!(to_vertex_indices.len(), 1);
//         assert!(to_vertex_indices.contains(&VertexIndex::new(1)));
//     }

//     #[test]
//     fn test_get_vertex_index_mask() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();
//         println!("{}", adjacency_matrix);

//         let vertex_index_map = adjacency_matrix.get_vertex_index_mask().unwrap();
//         println!("{}", vertex_index_map);
//         assert_eq!(
//             vertex_index_map.get_element_value_or_default(&0).unwrap(),
//             false
//         );
//         assert_eq!(
//             vertex_index_map.get_element_value_or_default(&1).unwrap(),
//             true
//         );
//         assert_eq!(
//             vertex_index_map.get_element_value_or_default(&2).unwrap(),
//             true
//         );
//         assert_eq!(
//             vertex_index_map.get_element_value_or_default(&3).unwrap(),
//             false
//         );
//     }

//     #[test]
//     fn test_get_vertex_indices() {
//         let context = Context::init_ready(Mode::NonBlocking).unwrap();
//         let vertex_capacity = 10;
//         let edge_type: EdgeTypeKey = String::from("Test edge type");

//         let mut adjacency_matrix =
//             WeightedAdjacencyMatrix::new(&context, edge_type, vertex_capacity).unwrap();

//         let edge_to_add_1 = EdgeCoordinate::new(2, 1);
//         adjacency_matrix.add_edge(&edge_to_add_1).unwrap();

//         let edge_to_add_2 = EdgeCoordinate::new(1, 1);
//         adjacency_matrix.add_edge(&edge_to_add_2).unwrap();

//         let vertex_indices = adjacency_matrix.get_vertex_indices().unwrap();
//         assert_eq!(vertex_indices.len(), 2);
//         assert!(vertex_indices.contains(&VertexIndex::new(1)));
//         assert!(vertex_indices.contains(&VertexIndex::new(2)));
//     }
// }
