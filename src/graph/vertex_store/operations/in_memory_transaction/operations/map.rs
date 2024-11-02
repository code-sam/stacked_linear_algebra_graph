use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::{
    GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices,
};
use crate::graph::indexing::{Index, VertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    RegisterVertexCapacityToRestore,
};
use crate::graph::vertex_store::operations::{
    indexed_map_mut_all_valid_vertex_vectors, map_all_vertex_vectors,
    map_mut_all_valid_vertex_vectors, map_mut_all_vertex_vectors, MapPrivateVertexVectors,
    MapPublicVertexVectors, MapValidVertexVectors,
};
use crate::graph::vertex_store::{
    GetVertexTypeIndexer, GetVertexVectors, VertexStore, VertexVector,
};

// impl<'s> MapAllVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
//     fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
//     {
//         map_all_vertex_vectors(self.vertex_store_ref(), function_to_apply)
//     }

//     fn map_mut_all_vertex_vectors<F>(
//         &mut self,
//         function_to_apply: F,
//     ) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
//     {
//         todo!();
//         self.vertex_store_ref().vertex_type_indexer_ref().iter_valid_indices()

//         self.register_vertex_vector_to_restore(vertex_type_index)
//         map_mut_all_vertex_vectors(self.vertex_store_mut_ref(), function_to_apply)
//     }
// }

impl<'s> MapValidVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
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
        self.map_mut_all_valid_vertex_vectors(function_to_apply)
    }
}

impl<'s> MapPublicVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
    // fn map_all_valid_public_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_public_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_mut_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_public_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.map_mut_all_valid_public_vertex_vectors(function_to_apply)
    }
}

impl<'s> MapPrivateVertexVectors for AtomicInMemoryVertexStoreTransaction<'s> {
    // fn map_all_valid_private_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_private_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_private_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.map_mut_all_valid_private_vertex_vectors(function_to_apply)
    }
}
