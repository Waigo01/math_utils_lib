use std::fmt::{self, Display};

///provides an enum with the corresponding From implementations in order to use as convenient return
///error type for this library.
#[derive(Debug)]
pub enum MathLibError {
    ParserError(ParserError),
    EvalError(EvalError),
    SolveError(SolveError),
    QuickSolveError(QuickSolveError),
    QuickEvalError(QuickEvalError),
    Other(String)
}

impl MathLibError {
    ///returns the reason behind the MathLibError.
    pub fn get_reason(&self) -> String {
        match self {
            MathLibError::ParserError(s) => return s.reason.clone(),
            MathLibError::EvalError(s) => return s.reason.clone(),
            MathLibError::SolveError(s) => return s.reason.clone(),
            MathLibError::QuickEvalError(s) => return s.reason.clone(),
            MathLibError::QuickSolveError(s) => return s.reason.clone(),
            MathLibError::Other(s) => return s.to_string(),
        }
    }
}

impl From<ParserError> for MathLibError {
    fn from(value: ParserError) -> Self {
        MathLibError::ParserError(value)
    }
}

impl From<EvalError> for MathLibError {
    fn from(value: EvalError) -> Self {
        MathLibError::EvalError(value)
    }
}

impl From<SolveError> for MathLibError {
    fn from(value: SolveError) -> Self {
        MathLibError::SolveError(value)
    }
}

impl From<QuickSolveError> for MathLibError {
    fn from(value: QuickSolveError) -> Self {
        MathLibError::QuickSolveError(value)
    }
}

impl From<QuickEvalError> for MathLibError {
    fn from(value: QuickEvalError) -> Self {
        MathLibError::QuickEvalError(value)
    }
}

#[derive(Debug)]
pub enum ParserErrorCode {
    ParseValue,
    MissingBracket,
    EmptyVec,
    UnmatchedOpenDelimiter,
    UnmatchedCloseDelimiter
}

#[derive(Debug)]
pub struct ParserError{
    pub code: ParserErrorCode,
    pub reason: String
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

#[derive(Debug)]
pub enum EvalErrorCode {
    NoVariable,
    MathError
}

#[derive(Debug)]
pub struct EvalError{
    pub code: EvalErrorCode,
    pub reason: String
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<String> for EvalError {
    fn from(value: String) -> Self {
        EvalError { code: EvalErrorCode::MathError, reason: value }
    }
}

#[derive(Debug)]
pub enum SolveErrorCode {
    VectorInEq,
    MatrixInEq,
    NothingToDo,
    EvalError(EvalErrorCode)
}

#[derive(Debug)]
pub struct SolveError {
    pub code: SolveErrorCode,
    pub reason: String
}

impl Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<EvalError> for SolveError {
    fn from(value: EvalError) -> Self {
        SolveError{code: SolveErrorCode::EvalError(value.code), reason: value.reason}
    }
}

#[derive(Debug)]
pub enum QuickEvalErrorCode {
    DuplicateVars,
    ParserError(ParserErrorCode),
    EvalError(EvalErrorCode)
}

#[derive(Debug)]
pub struct QuickEvalError {
    pub code: QuickEvalErrorCode,
    pub reason: String
}

impl Display for QuickEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<EvalError> for QuickEvalError {
    fn from(value: EvalError) -> Self {
        QuickEvalError { code: QuickEvalErrorCode::EvalError(value.code), reason: value.reason }
    }
}

impl From<ParserError> for QuickEvalError {
    fn from(value: ParserError) -> Self {
        QuickEvalError { code: QuickEvalErrorCode::ParserError(value.code), reason: value.reason }
    }
}

#[derive(Debug)]
pub enum QuickSolveErrorCode {
    DuplicateVars,
    NoEq,
    ParserError(ParserErrorCode),
    SolveError(SolveErrorCode)
}

#[derive(Debug)]
pub struct QuickSolveError {
    pub code: QuickSolveErrorCode,
    pub reason: String
}

impl Display for QuickSolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<SolveError> for QuickSolveError {
    fn from(value: SolveError) -> Self {
        QuickSolveError { code: QuickSolveErrorCode::SolveError(value.code), reason: value.reason }
    }
}

impl From<ParserError> for QuickSolveError {
    fn from(value: ParserError) -> Self {
        QuickSolveError { code: QuickSolveErrorCode::ParserError(value.code), reason: value.reason }
    }
}
