use crate::{
    graph::indexing::operations::{AtomicInMemoryIndexerTransaction, IndexerStateRestorer},
    operators::transaction::UseAtomicTransaction,
};

pub(crate) trait UseVertexStoreTransaction: UseAtomicTransaction {}

// #[derive(Clone, Debug)]
// pub(crate) struct VertexStore {
//     graphblas_context: Arc<GraphblasContext>,
//     vertex_type_indexer: VertexTypeIndexer,
//     vertex_vectors: Vec<VertexVector>,
//     element_indexer: VertexElementIndexer,

//     mask_to_select_entire_vertex_vector: SelectEntireVector,
// }

pub(crate) struct AtomicInMemoryVertexStoreTransaction<'t> {
    vertex_type_indexer_transaction: AtomicInMemoryIndexerTransaction<'t>,
    element_indexer_transaction: AtomicInMemoryIndexerTransaction<'t>,
    vertex_vector_transactions: IndexerStateRestorer,
}

// impl<'a> UseAtomicTransaction for AtomicInMemoryVertexStoreTransaction<'a> {
//     fn revert(&mut self) -> Result<(), GraphComputingError> {
//         let reset_indexer_state_restorer =
//             self.indexer_state_restorer.with_reset_state_to_restore();
//         let indexer_state_restorer = mem::replace(
//             &mut self.indexer_state_restorer,
//             reset_indexer_state_restorer,
//         );

//         indexer_state_restorer.restore(&mut self.indexer)
//     }

//     fn commit(&mut self) -> Result<(), GraphComputingError> {
//         self.indexer_state_restorer = IndexerStateRestorer::new_for_indexer(self.indexer)?;
//         Ok(())
//     }
// }

// impl<'t> Drop for AtomicInMemoryVertexStoreTransaction<'t> {
//     fn drop(&mut self) {
//         self.revert();
//     }
// }

// pub(in crate::graph::indexing::indexer::operations::in_memory_transaction) trait GetIndexerUnderTransaction
// {
//     fn indexer_ref(&self) -> &Indexer;
//     fn indexer_mut_ref(&mut self) -> &mut Indexer;
// }

// pub(in crate::graph::indexing::indexer::operations::in_memory_transaction) trait GetIndexerStateRestorer
// {
//     fn indexer_state_restorer_ref(&self) -> &IndexerStateRestorer;
//     fn indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer;
// }

// impl<'t> GetIndexerUnderTransaction for AtomicInMemoryVertexStoreTransaction<'t> {
//     fn indexer_ref(&self) -> &Indexer {
//         &self.indexer
//     }

//     fn indexer_mut_ref(&mut self) -> &mut Indexer {
//         &mut self.indexer
//     }
// }

// impl<'t> GetIndexerStateRestorer for AtomicInMemoryVertexStoreTransaction<'t> {
//     fn indexer_state_restorer_ref(&self) -> &IndexerStateRestorer {
//         &self.indexer_state_restorer
//     }

//     fn indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer {
//         &mut self.indexer_state_restorer
//     }
// }

// impl<'a> AtomicInMemoryVertexStoreTransaction<'a> {
//     pub(crate) fn new(indexer: &'a mut Indexer) -> Result<Self, GraphComputingError> {
//         let indexer_state_restorer = IndexerStateRestorer::new_for_indexer(indexer)?;
//         Ok(Self {
//             indexer,
//             indexer_state_restorer,
//         })
//     }
// }
