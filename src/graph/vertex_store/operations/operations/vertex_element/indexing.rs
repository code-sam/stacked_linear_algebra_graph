use crate::graph::indexing::GetVertexIndexIndex;
use crate::error::GraphComputingError;

pub(crate) trait CheckVertexIndex {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

// pub(crate) trait IsElementInVertexVector<T: ValueType> {
//     fn is_vertex_element(
//         &self,
//         vertex_index: &impl GetVertexIndexIndex,
//     ) -> Result<bool, GraphComputingError>;

//     fn try_is_vertex_element(
//         &self,
//         vertex_index: &impl GetVertexIndexIndex,
//     ) -> Result<(), GraphComputingError>;
// }
