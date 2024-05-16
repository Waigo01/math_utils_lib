use std::fmt::{self, Display};

///provides an enum with the corresponding From implementations in order to use as convenient return
///error type for this library.
#[derive(Debug, PartialEq)]
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
            MathLibError::ParserError(s) => return s.get_reason(),
            MathLibError::EvalError(s) => return s.get_reason(),
            MathLibError::SolveError(s) => return s.get_reason(),
            MathLibError::QuickEvalError(s) => return s.get_reason(),
            MathLibError::QuickSolveError(s) => return s.get_reason(),
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

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ParseValue(String),
    MissingBracket,
    EmptyVec,
    NotRectMatrix,
    EmptyExpr,
    UnmatchedOpenDelimiter,
    UnmatchedCloseDelimiter,
    EquationWithoutEqual,
    TooManyEquals,
    WrongNumberOfArgs(String)
}

impl ParserError {
    pub fn get_reason(&self) -> String {
        match self {
            ParserError::ParseValue(s) => return format!("Could not parse value {}!", s),
            ParserError::MissingBracket => return "Could not parse vector/matrix because of missing brackets!".to_string(),
            ParserError::EmptyVec => return "Could not parse vector/matrix because it is (partially) empty!".to_string(),
            ParserError::NotRectMatrix => return "Could not parse matrix because it is not rectangular!".to_string(),
            ParserError::EmptyExpr => return "Could not parse empty expression!".to_string(),
            ParserError::UnmatchedOpenDelimiter => return "Unmatched opening delimiter!".to_string(),
            ParserError::UnmatchedCloseDelimiter => return "Unmatched closing delimiter!".to_string(),
            ParserError::EquationWithoutEqual => return "Must have = in equation!".to_string(),
            ParserError::TooManyEquals => return "Too many = in equation. If you want to specify a system of equations please seperate each equation with a ','.".to_string(),
            ParserError::WrongNumberOfArgs(s) => return format!("Wrong number of arguments for {} operation!", s)
        }
    } 
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    NoVariable(String),
    MathError(String),
    SolveError(Box<SolveError>)
}

impl EvalError {
    pub fn get_reason(&self) -> String {
        match self {
            EvalError::NoVariable(s) => return format!("Could not find variable {}!", s),
            EvalError::MathError(s) => return s.to_string(),
            EvalError::SolveError(s) => return s.get_reason()
        }
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

impl From<String> for EvalError {
    fn from(value: String) -> Self {
        EvalError::MathError(value)
    }
}

impl From<SolveError> for EvalError {
    fn from(value: SolveError) -> Self {
        EvalError::SolveError(Box::new(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum SolveError {
    VectorInEq,
    MatrixInEq,
    NothingToDo, 
    EvalError(EvalError),
    NewtonError(NewtonError)
}

impl SolveError {
    pub fn get_reason(&self) -> String {
        match self {
            SolveError::VectorInEq => return "Can't have vectors in equations! Please convert your equation into a system of equations!".to_string(),
            SolveError::MatrixInEq => return "Can't have matrices in equations!".to_string(),
            SolveError::NothingToDo => return "Nothing to do!".to_string(),
            SolveError::EvalError(e) => return e.get_reason(),
            SolveError::NewtonError(e) => return e.get_reason()
        }
    }
}

impl Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

impl From<EvalError> for SolveError {
    fn from(value: EvalError) -> Self {
        SolveError::EvalError(value)
    }
}

impl From<String> for SolveError {
    fn from(value: String) -> Self {
        SolveError::EvalError(EvalError::MathError(value))
    }
}

impl From<NewtonError> for SolveError {
    fn from(value: NewtonError) -> Self {
        SolveError::NewtonError(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum NewtonError {
    UnderdeterminedSystem,
    InfiniteSolutions,
    NaNOrInf,
    ExpressionCheckFailed,
    EvalError(EvalError)
}

impl NewtonError {
    pub fn get_reason(&self) -> String {
        match self {
            NewtonError::UnderdeterminedSystem => return "Underdetermined system of equations!".to_string(),
            NewtonError::InfiniteSolutions => return "Infinite Solutions!".to_string(),
            NewtonError::NaNOrInf => return "NaN or Inf".to_string(),
            NewtonError::ExpressionCheckFailed => return "Expression Check Failed!".to_string(),
            NewtonError::EvalError(e) => return e.get_reason()
        }
    }
}

impl Display for NewtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

impl From<EvalError> for NewtonError {
    fn from(value: EvalError) -> Self {
        NewtonError::EvalError(value)
    }
}

impl From<String> for NewtonError {
    fn from(value: String) -> Self {
        NewtonError::EvalError(EvalError::MathError(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum QuickEvalError {
    DuplicateVars,
    ParserError(ParserError),
    EvalError(EvalError)
}

impl QuickEvalError {
    pub fn get_reason(&self) -> String {
        match self {
            QuickEvalError::DuplicateVars => return "Can't specify e and pi twice!".to_string(),
            QuickEvalError::EvalError(e) => return e.get_reason(),
            QuickEvalError::ParserError(e) => return e.get_reason()
        }
    }
}

impl Display for QuickEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

impl From<EvalError> for QuickEvalError {
    fn from(value: EvalError) -> Self {
        QuickEvalError::EvalError(value)
    }
}

impl From<ParserError> for QuickEvalError {
    fn from(value: ParserError) -> Self {
        QuickEvalError::ParserError(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum QuickSolveError {
    DuplicateVars,
    NoEq,
    ParserError(ParserError),
    SolveError(SolveError)
}

impl QuickSolveError {
    pub fn get_reason(&self) -> String {
        match self {
            QuickSolveError::DuplicateVars => return "Can't specify e and pi twice!".to_string(),
            QuickSolveError::NoEq => return "No \"=\" in equation!".to_string(),
            QuickSolveError::ParserError(e) => return e.get_reason(),
            QuickSolveError::SolveError(e) => return e.get_reason()
        }
    }
}

impl Display for QuickSolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

impl From<SolveError> for QuickSolveError {
    fn from(value: SolveError) -> Self {
        QuickSolveError::SolveError(value)
    }
}

impl From<ParserError> for QuickSolveError {
    fn from(value: ParserError) -> Self {
        QuickSolveError::ParserError(value)
    }
}
