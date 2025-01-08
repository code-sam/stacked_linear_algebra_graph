use std::error::Error;
use std::fmt;
use std::{convert::Infallible, error};

use graphblas_sparse_linear_algebra::error::{
    SparseLinearAlgebraError, SparseLinearAlgebraErrorType,
};

use super::GraphComputingError;

#[derive(Debug)]
pub struct LogicError {
    error_type: LogicErrorType,
    explanation: String,
    source: Option<LogicErrorSource>,
}

#[derive(Debug)]
pub enum LogicErrorSource {
    SparseLinearAlgebra(SparseLinearAlgebraError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicErrorType {
    SparseLinearAlgebra(SparseLinearAlgebraErrorType),
    DimensionMismatch,
    EdgeAlreadyExists,
    EdgeTypeDoesNotExist,
    EdgeTypeAlreadyExists,
    EdgeTypeMustExist,
    EdgeMustExist,
    IndexOutOfBounds,
    InvalidCharacter,
    InvalidIndex,
    InvalidKey,
    KeyAlreadyExists,
    ProductCannotEqualArgument,
    UnsafeTypeConversion,
    VertexAlreadyExists,
    VertexTypeDoesNotExist,
    VertexTypeAlreadyExsists,
    VertexTypeMustExist,
    VertexMustExist,
    VertexElementNotEmpty,
    Other,
}

impl LogicError {
    pub fn new(
        error_type: LogicErrorType,
        explanation: String,
        source: Option<LogicErrorSource>,
    ) -> Self {
        Self {
            error_type,
            explanation,
            source,
        }
    }

    pub fn error_type(&self) -> LogicErrorType {
        self.error_type.clone()
    }
    pub fn explanation(&self) -> String {
        self.explanation.clone()
    }
}

impl error::Error for LogicError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.source {
            Some(ref error) => match error {
                LogicErrorSource::SparseLinearAlgebra(error) => Some(error),
            },
            None => None,
        }
    }
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            // LogicErrorType::SparseLinearAlgebra(_err) => writeln!(f, "Context:\n{}", &self.context)?,
            _ => writeln!(f, "Context:\n{}", &self.explanation)?,
        };

        match &self.source() {
            Some(err) => writeln!(f, "Source error:\n{}", err)?,
            &None => (),
        }
        Ok(())
    }
}

impl From<SparseLinearAlgebraError> for LogicError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        Self {
            error_type: LogicErrorType::SparseLinearAlgebra(error.error_type()),
            explanation: String::new(),
            source: Some(LogicErrorSource::SparseLinearAlgebra(error)),
        }
    }
}

impl From<Infallible> for GraphComputingError {
    fn from(_: Infallible) -> Self {
        // Since Infallible can never actually occur, you can return a default or a specific error
        GraphComputingError::LogicError(LogicError {
            error_type: LogicErrorType::UnsafeTypeConversion,
            explanation: String::from("Infallible error occurred"),
            source: None,
        })
    }
}
