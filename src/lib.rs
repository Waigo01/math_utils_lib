//! This repo/crate provides a number of math utilities:
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("latex-export", "images/test.png")))]
#![cfg_attr(
not(feature = "doc-images"),
doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! - Parsing and evaluating expressions containing a combination of matrices, vectors and scalars.
//! - Solving equations and system of equations (both linear and non-linear).
//! - Exporting a LaTeX document from a collection of parsed and evaluated expressions.
//!
//! <div class="warning">This repo/crate has not hit 1.0.0 yet, breaking changes are bound to happen!</div>
//!
//! ## Major features
//!
//! - Parsing and evaluating calculations with matrices, vectors and scalars.
//! - A recursive parsing implementation allowing for calculations withing matrices and vectors.
//! - An inbuilt equation solver for solving linear and non-linear systems of equations, accessible through a custom "function".
//! - An evaluator based on combinatorics for combining multiple results from equations or sqrt with other operations.
//! - Inbuilt quality of life functions for exporting results to latex.
//!
//! ## Crate features
//!
//! - high-prec: uses a precision of 13 instead of 8 (will slow down execution).
//! - row-major: parses matrices in a row major format.
//! - output: enables dependencies in order to provide rendered PDFs, PNGs and SVGs.
//! - serde: enables serde::Serialize and serde::Deserialize on most structs and enums.
//!
//! ## Usage
//!
//! **For usage information concerning the mathematical properties of the evaluator and more examples, please take a look at [the wiki](https://github.com/Waigo01/math_utils_lib/wiki).**
//!
//! ## Error types
//!
//! If you want to use "?", take a look at [MathLibError].
//!
//! ## Examples
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library can do the most basic calculations.
//! let res = quick_eval("3*3", &mut Context::empty())?.to_vec();
//! 
//! // The return of the quick eval method will have multiple values to support things like sqrt(9) = {-3, 3}.
//! // In this case however this vector of values will only have one value (9).
//! assert_eq!(res[0], value!(9));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Variable, Value};
//! // Using the Context we can add variables with their respective values.
//! let x = Variable::new("x", vec![value!(3)]);
//! let res = quick_eval("3x", &mut Context::from_vars(vec![x]))?.to_vec();
//!
//! assert_eq!(res[0], value!(9));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // We can also define a mutable context to be used later.
//! let mut context = Context::empty();
//!
//! // We can now assign 3 to the variable x in the expression itself.
//! quick_eval("x=3", &mut context)?;
//! // And use the variable later on.
//! let res = quick_eval("3x", &mut context)?.to_vec();
//!
//! assert_eq!(res[0], value!(9));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library also has full matrix and vector support.
//! let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", &mut Context::empty())?.to_vec();
//!
//! // Notice that the matrix is by default parsed in a column major format, whereas internally the library uses a row-major format.
//! assert_eq!(res[0], value!(3, 1, 5; 4, 2, 6; 5, 3, 7));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{basetypes::Function, MathLibError, parse, quick_eval, value, Context, Value};
//! // We can also define custom functions based on a parsed expression.
//! let function = parse("5x^2+2x+x")?;
//! // We simply need to define the function name and the input variables, in this case "x".
//! let function_var = Function::new("f", function, vec!["x"]);
//!
//! // Then we can use that function later on.
//! let res = quick_eval("f(5)", &mut Context::from_funs(vec![function_var]))?.to_vec();
//!
//! assert_eq!(res[0], value!(140));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // Like before this assignment can also be done in an expression.
//! let mut context = Context::empty();
//!
//! quick_eval("f(x)=5x^2+2x+x", &mut context)?;
//! let res = quick_eval("f(5)", &mut context)?.to_vec();
//!
//! assert_eq!(res[0], value!(140));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library also has an inbuilt equation solver, based on newtons method. It can be accessed
//! // using the eq "function"
//! let res = quick_eval("eq(x^2=9, x)", &mut Context::empty())?.round(3).to_vec();
//!     
//! assert_eq!(res, vec![value!(-3), value!(3)]);
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The same solver can also solve both linear and non-linear systems of equations.
//! let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";
//!
//! let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();
//!
//! assert_eq!(res, vec![value!(3, -8, -2)]);
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```ignore
//! # use math_utils_lib::{parse, eval, Step, png_from_latex, export_history, Context, MathLibError, Value, ExportType};
//! let parsed_expr = parse("x = 3*3+6^5")?;
//! let res = eval(&parsed_expr, &mut Context::empty())?;
//!
//! // We can also create a "Step" based on a parsed expression and a corresponding result.
//! let step = Step::new(parsed_expr, res);
//!
//! // Which we can the export to a png or svg via latex.
//! let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;
//!
//! // Alternatively we can also export a history of steps to a full pdf document.
//! let pdf = export_history(vec![step], ExportType::Pdf)?;
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! Output (Please turn on dark mode to view the image, as the background is transparent):
//!
//! ![LaTeX][latex-export]
//!
//! ## TODO
//!
//! - [x] Support for vectors and matrices
//! - [x] Calculations in vectors and matrices
//! - [x] Equations as operators -> eval can handle multiple values
//! - [x] Variable/Function assignment as operator -> mutable context for evaluator
//! - [ ] Complex numbers
//! - [ ] Possible tensor support
//! - [ ] Stable API that makes everyone happy (very hard)
//!
//! ## Issues and Contributions
//!
//! When opening an issue on github, please specify the following:
//!
//! - The mathematical expression that causes the issue
//! - The error (or lack of it), be it a MathLibError or any other kind of error
//! - The expected behavior
//!
//! When it comes to contributions, feel free to fork the github repo and open pull requests.

use errors::QuickEvalError;

#[doc(hidden)]
pub mod maths;
#[doc(hidden)]
pub mod helpers;
pub mod basetypes;
pub mod latex;
pub mod parser;
pub mod errors;
pub mod roots;
pub mod solver;
pub mod evaluator;
pub mod tokenizer;

#[cfg(test)]
mod tests;

pub use basetypes::{Value, Values, Variable, Context, Function};
pub use latex::Step;
#[cfg(feature = "output")]
pub use latex::{export_history, ExportType, svg_from_latex, png_from_latex};
pub use parser::parse;
pub use evaluator::eval;
pub use errors::MathLibError;

#[cfg(feature = "high-prec")]
/// defines the precision used by the equation solver. The printing precision is PREC - 2.
pub const PREC: usize = 13;

#[cfg(not(feature = "high-prec"))]
/// defines the precision used by the equation solver. The printing precision is PREC - 2.
pub const PREC: usize = 8;

/// evaluates a given expression in the given context. If you just want the AST, have a look at [parse()].
///
/// For more information about the context, take a look at [Context] and for more information about
/// the possible operations, take a look at [SimpleOpType](basetypes::SimpleOpType) and
/// [AdvancedOpType](basetypes::AdvancedOpType).
///
/// # Examples
///
/// ```
/// # use math_utils_lib::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, value, Context, Value, Variable};
/// let res = quick_eval("3*3", &mut Context::default())?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// # Ok::<(), MathLibError>(())
/// ```
///
/// ```
/// # use math_utils_lib::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, value, Context, Value, Variable};
/// let x = Variable::new("x".to_string(), vec![value!(3.)]);
/// let res = quick_eval("3x".to_string(), &mut Context::from_vars(vec![x]))?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// # Ok::<(), MathLibError>(())
/// ```
pub fn quick_eval<S: Into<String>>(expr: S, context: &mut Context) -> Result<Values, QuickEvalError> {
    let expr = expr.into();
    let b_tree = parse(expr)?; 
    Ok(eval(&b_tree, context)?)
}
