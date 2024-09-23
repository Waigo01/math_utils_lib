use std::fmt::{self, Display};

///provides an enum with the corresponding From implementations in order to use as a convenient return
///error type for this library.
#[derive(Debug, PartialEq, Clone)]
pub enum MathLibError {
    ParserError(ParserError),
    EvalError(EvalError),
    QuickEvalError(QuickEvalError),
    #[cfg(feature = "output")]
    LatexError(LatexError),
    Other(String)
}

impl MathLibError {
    ///returns the reason behind the MathLibError.
    pub fn get_reason(&self) -> String {
        match self {
            MathLibError::ParserError(s) => return s.get_reason(),
            MathLibError::EvalError(s) => return s.get_reason(),
            MathLibError::QuickEvalError(s) => return s.get_reason(),
            #[cfg(feature = "output")]
            MathLibError::LatexError(s) => return s.get_reason(),
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

impl From<QuickEvalError> for MathLibError {
    fn from(value: QuickEvalError) -> Self {
        MathLibError::QuickEvalError(value)
    }
}

#[cfg(feature = "output")]
impl From<LatexError> for MathLibError {
    fn from(value: LatexError) -> Self {
        MathLibError::LatexError(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    NoEquation,
    InvalidVariableName(String),
    InvalidFunctionName(String),
    WrongNumberOfArgs(String),
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
            ParserError::NoEquation => return "Equation does not contain an '='!".to_string(),
            ParserError::InvalidVariableName(s) => return format!("Found invalid variable name: {}!", s),
            ParserError::InvalidFunctionName(s) => return format!("Found invalid function name: {}!", s),
            ParserError::WrongNumberOfArgs(s) => return format!("Wrong number of arguments for {} operation!", s),
        }
    } 
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_reason())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EvalError {
    NonScalarInVector,
    NonScalarInMatrix,
    RecursiveFunction,
    VectorInEq,
    MatrixInEq,
    NothingToDoEq,
    UnderdeterminedSystem,
    InfiniteSolutions,
    NaNOrInf,
    ExpressionCheckFailed,
    SearchVarsInVars,
    NoVariable(String),
    NoFunction(String),
    WrongNumberOfArgs((usize, usize)),
    MathError(String),
}

impl EvalError {
    pub fn get_reason(&self) -> String {
        match self {
            EvalError::RecursiveFunction => return "Can't call a recursive function!".to_string(),
            EvalError::NonScalarInVector => return "Vectors can only contain scalars!".to_string(),
            EvalError::NonScalarInMatrix => return "Matrices can only contain scalars!".to_string(),
            EvalError::VectorInEq => return "Can't have vectors in equations! Please convert your equation into a system of equations!".to_string(),
            EvalError::MatrixInEq => return "Can't have matrices in equations!".to_string(),
            EvalError::NothingToDoEq => return "Nothing to do!".to_string(),
            EvalError::UnderdeterminedSystem => return "Underdetermined system of equations!".to_string(),
            EvalError::InfiniteSolutions => return "Infinite Solutions!".to_string(),
            EvalError::NaNOrInf => return "NaN or Inf".to_string(),
            EvalError::ExpressionCheckFailed => return "Expression Check Failed!".to_string(),
            EvalError::SearchVarsInVars => return "The given solve variables already exist in the context!".to_string(),
            EvalError::NoVariable(s) => return format!("Could not find variable {}!", s),
            EvalError::NoFunction(s) => return format!("Could not find function {}!", s),
            EvalError::WrongNumberOfArgs((e, g)) => return format!("Wrong number of arguments! Expected {} arguments, {} were given!", e, g),
            EvalError::MathError(s) => return s.to_string(),
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

#[derive(Debug, PartialEq, Clone)]
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

#[cfg(feature = "output")]
#[derive(Debug, PartialEq, Clone)]
pub enum LatexError {
    LatexToPdfError(String),
    LatexToImageError(String),
    LatexToSvgError(String)
}

#[cfg(feature = "output")]
impl LatexError {
    pub fn get_reason(&self) -> String {
        match self {
            LatexError::LatexToPdfError(s) => return format!("Could not convert Latex to PDF: {}!", s),
            LatexError::LatexToImageError(s) => return format!("Could not convert Latex to Image: {}!", s),
            LatexError::LatexToSvgError(s) => return format!("Could not convert Latex to SVG: {}!", s)
        }
    }
}

#[cfg(feature = "output")]
impl From<gdk_pixbuf::glib::Error> for LatexError {
    fn from(value: gdk_pixbuf::glib::Error) -> Self {
        LatexError::LatexToImageError(value.to_string())
    }
}

#[cfg(feature = "output")]
impl From<mathjax_svg::Error> for LatexError {
    fn from(value: mathjax_svg::Error) -> Self {
        LatexError::LatexToSvgError(value.to_string())
    }
}

#[cfg(feature = "output")]
impl From<usvg::Error> for LatexError {
    fn from(value: usvg::Error) -> Self {
        LatexError::LatexToImageError(value.to_string())
    }
}

#[cfg(feature = "output")]
impl From<tectonic::Error> for LatexError {
    fn from(value: tectonic::Error) -> Self {
        LatexError::LatexToPdfError(value.to_string())
    }
}
