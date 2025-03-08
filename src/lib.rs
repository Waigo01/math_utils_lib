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
//! - output: enables dependencies in order to provide rendered PDFs, PNGs and SVGs. (currently
//! broken)
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
//! let res = quick_eval("3*3", &Context::empty())?.to_vec();
//!     
//! assert_eq!(res[0], value!(9));
//! ```
//!
//! ```rust
//! let x = Variable::new("x", value!(3)]);
//! let res = quick_eval("3x", &Context::from_vars(vec![x]))?.to_vec();
//!
//! assert_eq!(res[0], value!(9));
//! ```
//!
//! ```rust
//! let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", &Context::empty())?.to_vec();
//!
//! assert_eq!(res[0], value!(3, 1, 5; 4, 2, 6; 5, 3, 7));
//! ```
//!
//! ```rust
//! let function = parse("5x^2+2x+x")?;
//! let function_var = Function::new("f", function, vec!["x"]);
//!
//! let res = quick_eval("f(5)", &Context::from_funs(vec![function_var]))?.to_vec();
//!
//! assert_eq!(res[0], value!(140));
//! ```
//!
//! ```rust
//! let res = quick_eval("eq(x^2=9, x)", &Context::empty())?.round(3).to_vec();
//!     
//! assert_eq!(res, vec![value!(-3), value!(3)]);
//! ```
//!
//! ```rust
//! let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";
//!
//! let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();
//!
//! assert_eq!(res, vec![value!(3, -8, -2)]);
//! ```
//!
//! <div class="warning">Due to dependency issues output is currently broken.</div>
//!
//! ```rust
//! let parsed_expr = parse("3*3+6^5")?;
//! let res = eval(&parsed_expr, &Context::empty())?;
//!
//! let step = Step::Calc { term: parsed_expr, result: res, variable_save: Some("x".to_string()) };
//!
//! let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;
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

#[cfg(test)]
mod tests;

pub use basetypes::{Value, Values, Variable, Context};
pub use latex::Step;
#[cfg(feature = "output")]
pub use latex::{export_history, ExportType, svg_from_latex, png_from_latex};
pub use parser::{parse, eval};
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
/// let res = quick_eval("3*3", Context::default())?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// ```
///
/// ```
/// let x = Variable::new("x".to_string(), value!(3.));
/// let res = quick_eval("3x".to_string(), &Context::from_vars(vec![x]))?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// ```
pub fn quick_eval<S: Into<String>>(expr: S, context: &Context) -> Result<Values, QuickEvalError> {
    let expr = expr.into();
    let b_tree = parse(expr)?; 
    Ok(eval(&b_tree, &context)?)
}
