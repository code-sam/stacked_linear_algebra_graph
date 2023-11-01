use std::fmt::Display;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElement;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::{GraphComputingError, GraphComputingErrorType};
use crate::error::{LogicError, LogicErrorType};

use crate::graph::indexer::IndexerTrait;
use crate::graph::indexer::{AssignedIndex, AssignedIndexTrait};

use crate::graph::value_type::SparseVertexVectorForValueType;
use crate::graph::value_type::ValueType;

use crate::graph::vertex::vertex::GetVertexValue;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndex;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndexTrait;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKeyTrait;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKey;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};

use crate::graph::vertex_store::SparseVertexVector;
use crate::graph::vertex_store::VertexVector;

pub(crate) trait AddVertex<T>
where
    T: ValueType,
{
    fn add_new_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_new_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_new_index_defined_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;

    // fn add_or_replace_key_defined_vertex(
    //     &mut self,
    //     vertex: VertexDefinedByKey<T>,
    // ) -> Result<NewVertexIndex, GraphComputingError>;

    // fn add_or_replace_vertex_with_type_index_and_vertex_key(
    //     &mut self,
    //     vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    // ) -> Result<NewVertexIndex, GraphComputingError>;

    fn add_or_update_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;

    fn add_or_update_index_defined_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_update_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;
}

impl<T> AddVertex<T> for VertexStore
where
    T: ValueType
        + SparseVertexVectorForValueType<T>
        + GetVectorElementValueTyped<T>
        + SetVectorElementTyped<T>
        + Default
        + Copy,
    VertexVector: SparseVertexVector<T>,
    SparseMatrix<T>: Display,
{
    fn add_new_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let type_index = *self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex.type_key_ref())?;
        let vertex_index = self
            .element_indexer_mut_ref()
            .add_or_reuse_key(vertex.key_ref())?;

        match vertex_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;
                let vertex_vector: &mut SparseVector<T> = self
                    .vertex_vector_mut_ref_by_index_unchecked(&type_index)
                    .sparse_vector_mut_ref();
                vertex_vector
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
            }
            None => {
                let vertex_vector: &mut SparseVector<T> = self
                    .vertex_vector_mut_ref_by_index_unchecked(&type_index)
                    .sparse_vector_mut_ref();
                match vertex_vector.get_element_value(vertex_index.index_ref())? {
                    Some(_) => {
                        // The index alrady exists, no need to roll-back index assignment.
                        return Err(
                                LogicError::new(
                                    LogicErrorType::VertexAlreadyExists,
                                    format!("Vertex already exists for vertex type {}, vertex type {}, value type {}",
                                    vertex.type_key_ref(),
                                    vertex.key_ref(),
                                    std::any::type_name::<T>()),
                                    None).into()
                            );
                    }
                    None => {
                        vertex_vector
                            .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                    }
                }
            }
        }

        Ok(vertex_index)
    }

    fn add_new_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<AssignedIndex, GraphComputingError> {
        // TODO: review if the implementation can be sped up to performing this check only once.
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex.type_index_ref())?;
        let vertex_index = self
            .element_indexer_mut_ref()
            .add_or_reuse_key(vertex.key_ref())?;

        match vertex_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;
                let vertex_vector: &mut SparseVector<T> = self
                    .vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref())
                    .sparse_vector_mut_ref();
                vertex_vector
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
            }
            None => {
                let vertex_vector: &mut SparseVector<T> = self
                    .vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref())
                    .sparse_vector_mut_ref();
                match vertex_vector.get_element_value(vertex_index.index_ref())? {
                    Some(_) => {
                        // The index alrady exists, no need to roll-back index assignment.
                        return Err(
                                LogicError::new(
                                    LogicErrorType::VertexAlreadyExists,
                                    format!("Vertex already exists for vertex type {}, vertex type {}, value type {}",
                                    self.vertex_type_indexer_ref().key_for_index_unchecked(vertex.type_index_ref()),
                                    vertex.key_ref(),
                                    std::any::type_name::<T>()),
                                    None).into()
                            );
                    }
                    None => {
                        vertex_vector
                            .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                    }
                }
            }
        }

        Ok(vertex_index)
    }

    fn add_new_index_defined_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector: &mut SparseVector<T> = self
            .vertex_vector_mut_ref_by_index(vertex.type_index_ref())?
            .sparse_vector_mut_ref();

        match vertex_vector.get_element_value(vertex.index_ref())? {
            Some(_) => {
                // The index alrady exists, no need to roll-back index assignment.
                return Err(LogicError::new(
                    LogicErrorType::VertexAlreadyExists,
                    format!(
                        "Vertex already exists for vertex type {}, vertex type {}, value type {}",
                        self.vertex_type_indexer_ref()
                            .key_for_index_unchecked(vertex.type_index_ref()),
                        self.element_indexer_ref()
                            .key_for_index_unchecked(vertex.index_ref()),
                        std::any::type_name::<T>()
                    ),
                    None,
                )
                .into());
            }
            None => {}
        }

        vertex_vector.set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
        Ok(())
    }

    // fn add_or_replace_key_defined_vertex(
    //     &mut self,
    //     vertex: VertexDefinedByKey<$value_type>,
    // ) -> Result<NewVertexIndex, GraphComputingError> {
    //     let type_index = *self
    //         .vertex_type_indexer_ref()
    //         .try_index_for_key(vertex.type_key_ref())?;
    //     let vertex_index = self
    //         .element_indexer_mut_ref()
    //         .add_or_replace_key(vertex.key_ref())?; // TODO
    //     self.vertex_vector_by_index_mut_ref(&type_index)?
    //         .sparse_vector_mut_ref()
    //         .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
    //     Ok(vertex_index)
    // }

    // fn add_or_replace_vertex_with_type_index_and_vertex_key(
    //     &mut self,
    //     vertex: VertexDefinedByTypeIndexAndVertexKey<$value_type>,
    // ) -> Result<NewVertexIndex, GraphComputingError> {
    //     let vertex_index = self
    //         .element_indexer_mut_ref()
    //         .add_or_replace_key(vertex.key_ref())?; // TODO
    //     self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
    //         .sparse_vector_mut_ref()
    //         .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
    //     Ok(vertex_index)
    // }

    fn add_or_update_key_defined_vertex(
        &mut self,
        vertex: VertexDefinedByKey<T>,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        let type_index = *self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex.type_key_ref())?;

        match self.element_indexer_mut_ref().add_new_key(vertex.key_ref()) {
            Ok(vertex_index) => {
                self.vertex_vector_mut_ref_by_index(&type_index)?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                return Ok(Some(vertex_index));
            }
            Err(error) => match error.error_type() {
                GraphComputingErrorType::LogicErrorType(LogicErrorType::KeyAlreadyExists) => {
                    let vertex_index;
                    match self
                                        .element_indexer_ref()
                                        .index_for_key(vertex.key_ref())
                                    {
                                        Some(index_ref) => {
                                            vertex_index = *index_ref;
                                        }
                                        None => {
                                            return Err(
                                                LogicError::new(
                                                    LogicErrorType::Other,
                                                    format!("Unable to find index for vertex key, although the key was checked for validity before"),
                                                    None).into())
                                        }
                                    }

                    self.vertex_vector_mut_ref_by_index(&type_index)?
                        .sparse_vector_mut_ref()
                        .set_element((vertex_index, *vertex.value_ref()).into())?;
                    Ok(None)
                }
                _ => return Err(error),
            },
        }

        // // TODO: do not clone self.element_indexer_ref()
        // match self
        //     .element_indexer_ref()
        //     .clone()
        //     .index_for_key(vertex.key_ref())
        // {
        //     Some(index_ref) => {
        //         self.vertex_vector_by_index_mut_ref(&type_index)?
        //             .sparse_vector_mut_ref()
        //             .set_element((*index_ref, *vertex.value_ref()).into())?;
        //         Ok(None)
        //     }
        //     None => {
        //         let vertex_index = self
        //             .element_indexer_mut_ref()
        //             .add_new_key(vertex.key_ref())?;
        //         self.vertex_vector_by_index_mut_ref(&type_index)?
        //             .sparse_vector_mut_ref()
        //             .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
        //         Ok(Some(vertex_index))
        //     }
        // }
    }

    fn add_or_update_index_defined_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector: &mut SparseVector<T> = self
            .vertex_vector_mut_ref_by_index(vertex.type_index_ref())?
            .sparse_vector_mut_ref();
        vertex_vector.set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
        Ok(())
    }

    fn add_or_update_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        match self.element_indexer_mut_ref().add_new_key(vertex.key_ref()) {
            Ok(vertex_index) => {
                self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?
                    .sparse_vector_mut_ref()
                    .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
                return Ok(Some(vertex_index));
            }
            Err(error) => match error.error_type() {
                GraphComputingErrorType::LogicErrorType(LogicErrorType::KeyAlreadyExists) => {}
                _ => return Err(error),
            },
        }

        let vertex_index;
        match self
        .element_indexer_ref()
        .index_for_key(vertex.key_ref())
    {
        Some(index_ref) => {
            vertex_index = *index_ref;
        },
        None => {
            return Err(
                LogicError::new(
                    LogicErrorType::Other,
                    format!("Unable to find index for vertex key, although the key was checked for validity before"),
                    None).into())
        }
    }

        self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?
            .sparse_vector_mut_ref()
            .set_element((vertex_index, *vertex.value_ref()).into())?;
        Ok(None)

        // // TODO: do not clone self.element_indexer_ref()
        // match self
        //     .element_indexer_ref()
        //     .clone()
        //     .index_for_key(vertex.key_ref())
        // {
        //     Some(index_ref) => {
        //         self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
        //             .sparse_vector_mut_ref()
        //             .set_element((*index_ref, *vertex.value_ref()).into())?;
        //         Ok(None)
        //     }
        //     None => {
        //         let vertex_index = self
        //             .element_indexer_mut_ref()
        //             .add_new_key(vertex.key_ref())?;
        //         self.vertex_vector_by_index_mut_ref(vertex.type_index_ref())?
        //             .sparse_vector_mut_ref()
        //             .set_element((*vertex_index.index_ref(), *vertex.value_ref()).into())?;
        //         Ok(Some(vertex_index))
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use graphblas_sparse_linear_algebra::context::{
        Context as GraphblasContext, Mode as GraphblasMode,
    };

    use crate::graph::vertex_store::type_operations::add_vertex_type::AddVertexType;

    #[test]
    fn test_add_new_key_defined_vertex() {
        let context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut store = VertexStore::with_initial_capacity(&context, &0, &0).unwrap();

        for i in 0..2 {
            store
                .add_new_vertex_type(format!("vertex_type_{}", i).as_str())
                .unwrap();
        }

        for i in 0..50 {
            let vertex =
                VertexDefinedByKey::new("vertex_type_1", format!("vertex_{}", i).as_str(), &i);
            store.add_new_key_defined_vertex(vertex).unwrap();
        }
    }
}
