use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::vertex_type::{
    map_mut_all_valid_vertex_vectors, MapValidVertexVectors,
};
use crate::graph::vertex_store::{VertexStore, VertexVector};

// impl MapAllVertexVectors for VertexStore {
//     fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
//     {
//         map_all_vertex_vectors(self, function_to_apply)
//     }

//     fn map_mut_all_vertex_vectors<F>(
//         &mut self,
//         function_to_apply: F,
//     ) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
//     {
//         map_mut_all_vertex_vectors(self, function_to_apply)
//     }
// }

impl MapValidVertexVectors for VertexStore {
    // fn map_all_valid_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        map_mut_all_valid_vertex_vectors(self, function_to_apply)
    }
}
