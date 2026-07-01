#![cfg_attr(docsrs, feature(doc_cfg))]

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
//! - Generic number implementation for scalars. Allowing for different number types with custom
//! parsers and calculation rules.
//! - A recursive parsing implementation allowing for calculations withing matrices and vectors.
//! - An inbuilt equation solver for solving linear and non-linear systems of equations, accessible through a custom "function".
//! - An evaluator based on combinatorics for combining multiple results from equations or sqrts with other operations.
//! - Assigning values to variables and defining custom functions.
//! - Custom functions with side effects.
//! - Boolean operations (==, <, >, etc.).
//! - Conditional evaluation.
//! - Inbuilt quality of life functions for exporting results to latex.
//!
//! ## Crate features
//!
//! - row-major: parses matrices in a row major format.
//! - output: enables dependencies in order to provide rendered PDFs, PNGs and SVGs.
//! - serde: enables serde::Serialize and serde::Deserialize on most structs and enums.
//! - parallelism: enables multithreading for the equation solver.
//!
//! ## Usage
//!
//! **For usage information concerning the mathematical syntax used by the parser and more examples, please take a look at [the wiki](https://github.com/Waigo01/math_utils_lib/wiki).**
//!
//! ## Error types
//!
//! If you want to use "?", take a look at [MathLibError].
//!
//! ## Examples
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library can do the most basic calculations.
//! let res = quick_eval!("3*3")?.to_vec();
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
//! let res = quick_eval!("3x", &mut Context::from_vars(vec![x]))?.to_vec();
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
//! quick_eval!("x=3", &mut context)?;
//! // And use the variable later on.
//! let res = quick_eval!("3x", &mut context)?.to_vec();
//!
//! assert_eq!(res[0], value!(9));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library also has full matrix and vector support.
//! let res = quick_eval!("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]")?.to_vec();
//!
//! // Notice that the matrix is by default parsed in a column major format,
//! // whereas internally the library uses a row-major format.
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
//! let res = quick_eval!("f(5)", &mut Context::from_funs(vec![function_var]))?.to_vec();
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
//! quick_eval!("f(x)=5x^2+2x+x", &mut context)?;
//! let res = quick_eval!("f(5)", &mut context)?.to_vec();
//!
//! assert_eq!(res[0], value!(140));
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! let mut c = Context::default();
//! 
//! // Defined functions can even have side effects,
//! // like setting a variable in the context, when called.
//! quick_eval!("f(x) = y=x", &mut c)?;
//!
//! quick_eval!("f(5)", &mut c)?;
//!
//! assert_eq!(c.get_var("y".to_string()).unwrap().values.to_vec()[0], value!(5));
//!
//! # Ok::<(), MathLibError>(())
//! ```
//!
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The library also has an inbuilt equation solver, based on newtons method. It can be accessed
//! // using the eq "function"
//! let res = quick_eval!("eq(x^2=9, x)")?.round(3).to_vec();
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
//! let res = quick_eval!(equation)?.round(3).to_vec();
//!
//! assert_eq!(res, vec![value!(3, -8, -2)]);
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! let mut c = Context::empty();
//!
//! quick_eval!("a = 6", &mut c)?;
//! quick_eval!("b = 10", &mut c)?;
//! 
//! // The evaluator also supports boolean expressions where 0 = false and !0 = true.
//! // The and (&) and or (|) operation will always return 1 for true and 0 for false.
//! let res = quick_eval!("!(a != 10 | b != 10) | b != 5", &mut c)?.to_vec();
//!
//! assert_eq!(res[0], value!(1));
//!
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//! // The evaluator can also make case destinctions using an if statement and a boolean expression.
//! let res = quick_eval!("if(eq(x^2 = 9, x) == -3, 5, 2)")?.to_vec();
//!
//! // Since there are two solutions to x^2 = 9
//! // and the first one is indeed -3 but the second one is not, the resulting Values are 5, 2.
//! assert_eq!(res, vec![value!(5), value!(2)]);
//!
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Context, Value};
//!
//! let mut c = Context::empty();
//! quick_eval!("state = 0", &mut c)?;
//!
//! // Using the power of lists, side effects in functions and if statements,
//! // we can even build simple state machines.
//!
//! // This one recognizes the word 'math' with m = 0, a = 1, t = 2 and h = 3.
//!
//! quick_eval!("f(x) =
//! if(x == 0 & state == 0,
//!    state = 1,
//! if(x == 1 & state == 1,
//!    state = 2,
//! if(x == 2 & state == 2,
//!    state = 3,
//! if(x == 3 & state == 3,
//!    state = 4,
//! state = 0))))", &mut c)?;
//!
//! // This is the word 'math'.
//!
//! quick_eval!("f({0, 1, 2, 3})", &mut c)?;
//!
//! let res = quick_eval!("state", &mut c)?.to_vec();
//!
//! assert_eq!(res[0], value!(4));
//! 
//! quick_eval!("state = 0", &mut c)?;
//!
//! // This is NOT the word 'math'.
//!
//! quick_eval!("f({0, 1, 4, 2})", &mut c)?.to_vec();
//! 
//! let res = quick_eval!("state", &mut c)?.to_vec();
//!
//! assert_ne!(res[0], value!(4));
//!
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```rust
//! # use math_utils_lib::{MathLibError, quick_eval, value, Complex};
//!
//! // You can also use other types that implement the Number trait, such as complex numbers.
//! let res = quick_eval!("e^(i*pi)"; Complex<f64>)?.to_vec();
//!
//! assert_eq!(res[0], value!(-1));
//!
//! # Ok::<(), MathLibError>(())
//! ```
//!
//! ```ignore
//! # use math_utils_lib::{parse, eval, Step, png_from_latex, export_history, Context, MathLibError, Value, ExportType, AST};
//! let parsed_expr: AST<f64> = parse("x = 3*3+6^5")?;
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
//! - [x] Function side effects
//! - [x] Boolean operations
//! - [x] Conditional evaluation
//! - [x] Generic numbers
//! - [x] Complex numbers
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

#[doc(hidden)]
pub mod maths;
#[doc(hidden)]
pub mod helpers;
pub mod basetypes;
pub mod output;
pub mod parser;
pub mod errors;
pub mod roots;
pub mod solver;
pub mod evaluator;
pub mod tokenizer;

#[cfg(test)]
mod tests;

pub use basetypes::{Value, Values, Variable, Context, Function, AST, InternalFunction};
pub use output::{export_history, ExportType, Step};
#[cfg(feature = "output")]
pub use output::{png_from_latex, svg_from_latex};
pub use parser::parse;
pub use evaluator::eval;
pub use errors::MathLibError;
pub use maths::num_traits::{Number, StandardFunctions, RealNumber};
pub use maths::num_impls::Complex;

/// evaluates a given expression in the given context. If you just want the AST, have a look at [parse()].
///
/// For more information about the context, take a look at [Context] and for more information about
/// the possible operations, take a look at [SimpleOpType](basetypes::SimpleOpType) and
/// [AdvancedOperation](basetypes::AdvancedOperation).
///
/// When calling this macro with just a string argument, it will internally create a new Context
/// using [Context::default()]. You can also specify your own context as the second argument. With the argument following the semicolon you can specify the number 
/// type that should be used. By default [f64] is used, other number types must implement [Number].
///
/// # Examples
///
/// ```
/// # use math_utils_lib::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, value, Context, Value, Variable};
/// let res = quick_eval!("3*3")?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// # Ok::<(), MathLibError>(())
/// ```
///
/// ```
/// # use math_utils_lib::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, value, Context, Value, Variable};
/// let x = Variable::new("x".to_string(), vec![value!(3.)]);
/// let res = quick_eval!("3x".to_string(), &mut Context::from_vars(vec![x]))?.to_vec();
///
/// assert_eq!(res[0], value!(9.));
/// # Ok::<(), MathLibError>(())
/// ```

#[macro_export]
macro_rules! quick_eval {
    ( $e:expr ) => {
        {
            pub fn quick_eval<S: Into<String>, N: $crate::Number>(expr: S, context: &mut $crate::Context<N>) -> Result<$crate::Values<N>, $crate::errors::QuickEvalError> {
                let expr = expr.into();
                let b_tree = $crate::parse(expr)?; 
                Ok($crate::eval(&b_tree, context)?)
            }

            quick_eval::<_, f64>($e, &mut $crate::Context::default())
        }
    };
    ( $e:expr, $c:expr ) => {
        {
            pub fn quick_eval<S: Into<String>, N: $crate::Number>(expr: S, context: &mut $crate::Context<N>) -> Result<$crate::Values<N>, $crate::errors::QuickEvalError> {
                let expr = expr.into();
                let b_tree = $crate::parse(expr)?; 
                Ok($crate::eval(&b_tree, context)?)
            }

            quick_eval::<_, f64>($e, $c)
        }
    };
    ( $e:expr; $t:ty ) => {
        { 
            pub fn quick_eval<S: Into<String>, N: $crate::Number>(expr: S, context: &mut $crate::Context<N>) -> Result<$crate::Values<N>, $crate::errors::QuickEvalError> {
                let expr = expr.into();
                let b_tree = $crate::parse(expr)?; 
                Ok($crate::eval(&b_tree, context)?)
            }

            quick_eval::<_, $t>($e, &mut $crate::Context::default())
        }
    };
    ( $e:expr, $c:expr; $t:ty ) => {
        { 
            pub fn quick_eval<S: Into<String>, N: $crate::Number>(expr: S, context: &mut $crate::Context<N>) -> Result<$crate::Values<N>, $crate::errors::QuickEvalError> {
                let expr = expr.into();
                let b_tree = $crate::parse(expr)?; 
                Ok($crate::eval(&b_tree, context)?)
            }

            quick_eval::<_, $t>($e, $c)
        }
    }
}
