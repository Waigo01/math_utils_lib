use std::{error::Error, fmt::{self, Display}};

/// Provides an enum with the corresponding From implementations in order to use as a convenient return
/// error type for this library.
#[derive(Debug, PartialEq, Clone)]
pub enum MathLibError {
    ParserError(ParserError),
    EvalError(EvalError),
    QuickEvalError(QuickEvalError),
    LatexError(LatexError),
    TokenizerError(TokenizerError),
    Other(String)
}

impl Display for MathLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathLibError::ParserError(s) => return write!(f, "{}", s),
            MathLibError::EvalError(s) => return write!(f, "{}", s),
            MathLibError::QuickEvalError(s) => return write!(f, "{}", s),
            MathLibError::LatexError(s) => return write!(f, "{}", s),
            MathLibError::TokenizerError(s) => return write!(f, "{}", s),
            MathLibError::Other(s) => return write!(f, "{}", s)
        }
    }
}

impl Error for MathLibError {}

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

impl From<QuickEvalError> for MathLibError {
    fn from(value: QuickEvalError) -> Self {
        MathLibError::QuickEvalError(value)
    }
}

impl From<TokenizerError> for MathLibError {
    fn from(value: TokenizerError) -> Self {
        MathLibError::TokenizerError(value)
    }
}

impl From<LatexError> for MathLibError {
    fn from(value: LatexError) -> Self {
        MathLibError::LatexError(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    ParseValue(String),
    EmptyVec,
    NotRectMatrix,
    EmptyExpr,
    EquationWithoutEqual,
    TooManyEquals,
    NoEquation,
    UnrecognizedPunct,
    InvalidArg(String),
    WrongNumberOfArgs(String),
    OperationNeedsLeftValue,
    DerivNotITOVar,
    LeftSideOfAssignmentIncorrect,
    InvalidState,
    TokenizerError(TokenizerError)
}

impl From<TokenizerError> for ParserError {
    fn from(value: TokenizerError) -> Self {
        ParserError::TokenizerError(value)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::ParseValue(s) => return write!(f, "could not parse value {s}"),
            ParserError::EmptyVec => return write!(f, "could not parse vector/matrix because it is (partially) empty"),
            ParserError::NotRectMatrix => return write!(f, "could not parse matrix because it is not rectangular"),
            ParserError::EmptyExpr => return write!(f, "could not parse empty expression"),
            ParserError::UnrecognizedPunct => return write!(f, "unrecognized punctuation"),
            ParserError::EquationWithoutEqual => return write!(f, "must have = in equation"),
            ParserError::TooManyEquals => return write!(f, "too many = in equation. If you want to specify a system of equations please seperate each equation with a ','"),
            ParserError::NoEquation => return write!(f, "equation does not contain an '='"),
            ParserError::InvalidArg(s) => return write!(f, "invalid argument for operation {s}"),
            ParserError::WrongNumberOfArgs(s) => return write!(f, "wrong number of arguments for {s} operation"),
            ParserError::OperationNeedsLeftValue => return write!(f, "the operation needs to have a left side"),
            ParserError::LeftSideOfAssignmentIncorrect => return write!(f, "the left side of an assignment needs to be a variable or a function"),
            ParserError::InvalidState => return write!(f, "an invalid state was reached"),
            ParserError::DerivNotITOVar => return write!(f, "second input (in terms of) to derivative/integral function must be a variable"),
            ParserError::TokenizerError(e) => return write!(f, "{e}")
        }
    }
}

impl Error for ParserError {}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenizerError {
    InvalidIdentName,
    UnmatchedDelimiter,
    InvalidLiteral(String),
    InvalidPunct(char),
    InvalidState
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::InvalidIdentName => return write!(f, "a function/variable name is invalid"),
            TokenizerError::UnmatchedDelimiter => return write!(f, "unmatched delimiter"),
            TokenizerError::InvalidLiteral(l) => return write!(f, "invalid literal {l}"),
            TokenizerError::InvalidPunct(c) => return write!(f, "invalid punt {c}"),
            TokenizerError::InvalidState => return write!(f, "an invalid state was encountered")
        }
    }
}

impl Error for TokenizerError {}

#[derive(Debug, PartialEq, Clone)]
pub enum EvalError {
    NonScalarInVector,
    NonScalarInMatrix,
    RecursiveFunctionDepth,
    VectorInEq,
    MatrixInEq,
    NothingToDoEq,
    UnderdeterminedSystem,
    InfiniteSolutions,
    NaNOrInf,
    ExpressionCheckFailed,
    SearchVarsInVars,
    MultiVariableAssignmentItemNumber,
    NoVariable(String),
    NoFunction(String),
    WrongNumberOfArgs((usize, usize)),
    MathError(String),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::RecursiveFunctionDepth => return write!(f, "maximum recursive function depth exceeded"),
            EvalError::NonScalarInVector => return write!(f, "vectors can only contain scalars"),
            EvalError::NonScalarInMatrix => return write!(f, "matrices can only contain scalars"),
            EvalError::VectorInEq => return write!(f, "can't have vectors in equations, please convert your equation into a system of equations"),
            EvalError::MatrixInEq => return write!(f, "can't have matrices in equations"),
            EvalError::NothingToDoEq => return write!(f, "nothing to do"),
            EvalError::UnderdeterminedSystem => return write!(f, "underdetermined system of equations"),
            EvalError::InfiniteSolutions => return write!(f, "infinite solutions"),
            EvalError::NaNOrInf => return write!(f, "nan or inf"),
            EvalError::ExpressionCheckFailed => return write!(f, "expression check failed"),
            EvalError::SearchVarsInVars => return write!(f, "the given solve variables already exist in the context"),
            EvalError::NoVariable(s) => return write!(f, "could not find variable {s}"),
            EvalError::NoFunction(s) => return write!(f, "Could not find function {s}"),
            EvalError::WrongNumberOfArgs((e, g)) => return write!(f, "wrong number of arguments, expected {e} arguments, {g} were given"),
            EvalError::MathError(s) => return write!(f, "{s}"),
            EvalError::MultiVariableAssignmentItemNumber => return write!(f, "number of variables does not match number of results on the right side of the assignment"),
        }
    }
}

impl From<String> for EvalError {
    fn from(value: String) -> Self {
        EvalError::MathError(value)
    }
}

impl Error for EvalError {}

#[derive(Debug, PartialEq, Clone)]
pub enum QuickEvalError {
    DuplicateVars,
    ParserError(ParserError),
    EvalError(EvalError)
}

impl Display for QuickEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuickEvalError::DuplicateVars => return write!(f, "can't specify e and pi twice"),
            QuickEvalError::EvalError(e) => return write!(f, "{e}"),
            QuickEvalError::ParserError(e) => return write!(f, "{e}")
        }
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

impl Error for QuickEvalError {}

#[derive(Debug, PartialEq, Clone)]
pub enum LatexError {
    LatexToPdfError(String),
    LatexToImageError(String),
    LatexToSvgError(String)
}

impl Display for LatexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LatexError::LatexToPdfError(s) => return write!(f, "could not convert latex to pdf: {s}"),
            LatexError::LatexToImageError(s) => return write!(f, "could not convert latex to image: {s}"),
            LatexError::LatexToSvgError(s) => return write!(f, "could not convert latex to svg: {s}")
        }
    }
}

#[cfg(feature = "output")]
impl From<mathjax_svg::Error> for LatexError {
    fn from(value: mathjax_svg::Error) -> Self {
        LatexError::LatexToSvgError(value.to_string())
    }
}

#[cfg(feature = "output")]
impl From<tectonic::Error> for LatexError {
    fn from(value: tectonic::Error) -> Self {
        LatexError::LatexToPdfError(value.to_string())
    }
}

#[cfg(feature = "output")]
impl From<resvg::usvg::Error> for LatexError {
    fn from(value: resvg::usvg::Error) -> Self {
        LatexError::LatexToImageError(value.to_string())
    }
}

impl Error for LatexError {}
