use std::fmt::Display;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::is_element;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    SetVectorElement, SetVectorElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetGraphblasSparseVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::collections::sparse_vector::VectorElement;
use graphblas_sparse_linear_algebra::context::GetContext;

use crate::error::{GraphComputingError, GraphComputingErrorType};
use crate::error::{LogicError, LogicErrorType};

use crate::graph::indexer::IndexerTrait;
use crate::graph::indexer::{AssignedIndex, AssignedIndexTrait};

use crate::graph::value_type::ValueType;

use crate::graph::vertex::vertex::GetVertexValue;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndex;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndexTrait;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKeyTrait;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKey;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
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
    T: ValueType + GetVectorElementValueTyped<T> + SetVectorElementTyped<T> + Default + Copy,
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

        let element: VectorElement<T> = (*vertex_index.index_ref(), *vertex.value_ref()).into();

        match vertex_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;

                let vertex_vector = self.vertex_vector_mut_ref_by_index_unchecked(&type_index);
                T::set_element(vertex_vector, element)?;
            }
            None => {
                let vertex_vector = self.vertex_vector_mut_ref_by_index_unchecked(&type_index);
                if is_element(vertex_vector, *vertex_index.index_ref())? {
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
                } else {
                    T::set_element(vertex_vector, element)?;
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
        let vertex_index: AssignedIndex = self
            .element_indexer_mut_ref()
            .add_or_reuse_key(vertex.key_ref())?;

        let element: VectorElement<T> = (*vertex_index.index_ref(), *vertex.value_ref()).into();

        match vertex_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;
                let vertex_vector: &mut VertexVector =
                    self.vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref());
                T::set_element(vertex_vector, element)?;
            }
            None => {
                let vertex_vector: &mut VertexVector =
                    self.vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref());
                if is_element(vertex_vector, *vertex_index.index_ref())? {
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
                } else {
                    T::set_element(vertex_vector, element)?;
                }
            }
        }

        Ok(vertex_index)
    }

    fn add_new_index_defined_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector: &mut VertexVector =
            self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;

        let element: VectorElement<T> = (*vertex.index_ref(), *vertex.value_ref()).into();

        if is_element(vertex_vector, *vertex.index_ref())? {
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

        T::set_element(vertex_vector, element)?;
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

        let element: VectorElement<T> = (type_index, *vertex.value_ref()).into();

        match self.element_indexer_mut_ref().add_new_key(vertex.key_ref()) {
            Ok(vertex_index) => {
                let vertex_vector: &mut VertexVector =
                    self.vertex_vector_mut_ref_by_index(&type_index)?;
                T::set_element(vertex_vector, element)?;
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

                    let vertex_vector: &mut VertexVector =
                        self.vertex_vector_mut_ref_by_index(&type_index)?;
                    T::set_element(vertex_vector, element)?;
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
        let vertex_vector: &mut VertexVector =
            self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
        let element: VectorElement<T> =
            (vertex.type_index_ref().to_owned(), *vertex.value_ref()).into();
        T::set_element(vertex_vector, element)?;
        Ok(())
    }

    fn add_or_update_vertex_with_type_index_and_vertex_key(
        &mut self,
        vertex: VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        match self.element_indexer_mut_ref().add_new_key(vertex.key_ref()) {
            Ok(vertex_index) => {
                let element = (*vertex_index.index_ref(), *vertex.value_ref()).into();
                let vertex_vector: &mut VertexVector =
                    self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
                T::set_element(vertex_vector, element)?;
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

        let vertex_vector = self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
        let element = (vertex_index, *vertex.value_ref()).into();
        T::set_element(vertex_vector, element)?;
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

    use crate::graph::vertex_store::operations::add_vertex_type::AddVertexType;

    #[test]
    fn test_add_new_key_defined_vertex() {
        let context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

        let mut store = VertexStore::with_initial_capacity(&context, &0, &0).unwrap();

        for i in 0..2 {
            AddVertexType::<i32>::add_new_vertex_type(
                &mut store,
                format!("vertex_type_{}", i).as_str(),
            )
            .unwrap();
        }

        for i in 0..50 {
            let vertex =
                VertexDefinedByKey::new("vertex_type_1", format!("vertex_{}", i).as_str(), &i);
            store.add_new_key_defined_vertex(vertex).unwrap();
        }
    }
}
