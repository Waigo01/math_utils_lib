//! This crate provides a number of math utilities:
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("latex-export", "images/export-1.png")))]
#![cfg_attr(
not(feature = "doc-images"),
doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! - parsing and evaluating expressions containing matrices and vectors
//! - solving equations and systems of equations
//! - exporting a LaTeX document from a collection of parsed expressions or solved equations (see 
//! [Step](enum@latex_export::Step) and [export()])
//!
//! Precision ([PREC]):
//!
//! Use feature high-prec for a precision of 13, standard precision is 8. The precision is used to
//! solve equations at the set precision. The actual output precision for printing is the defined precision - 2.
//!
//! <div class="warning">Important Rules:
//!
//! Vectors and matrices can be written out in an expression (e.g [3, 5, 7] or [[3, 4, 5], [3, 4,
//! 5], [2, 6, 7]]). Matrices are in column major order. <br>
//! Variable Names can only start with an alphabetic letter or a \ (see [Variable]) <br>
//! See [SimpleOpType](enum@parser::SimpleOpType) and [AdvancedOpType](enum@parser::AdvancedOpType) for all allowed operations and functions.
//!
//! </div>
//!
//! <div class="warning">This crate has not hit 1.0.0 yet, breaking changes are bound to
//! happen!</div>
//!
//! # Examples
//! (if you want to use "?" for error handling in your application, have a look at
//! [MathLibError])
//! ## Evaluations:
//! ```
//! let res = quick_eval("3*3".to_string(), vec![])?;
//!
//! assert_eq!(res, Value::Scalar(9.));
//! ```
//!
//! ```
//! let x = Variable::new("x".to_string(), Value::Scalar(3.));
//! let res = quick_eval("3x".to_string(), vec![x])?;
//!
//! assert_eq!(res, Value::Scalar(9.));
//! ```
//!
//! ```
//! let a = Variable::new("A".to_string(), Value::Vector(vec![3., 5., 8.]));
//! let res = quick_eval("3A".to_string(), vec![a])?;
//!
//! assert_eq!(res, Value::Vector(vec![9., 15., 24.]));
//! ```
//!
//! ```
//! let a = Variable::new("A".to_string(), Value::Vector(vec![3., 5., 8.]));
//! let b = Variable::new("B".to_string(), Value::Matrix(vec![vec![2., 0., 0.], vec![0., 2., 0.],
//! vec![0., 0., 1.]]));
//! let res = quick_eval("B*A".to_string(), vec![a, b])?;
//!
//! assert_eq!(res, Value::Vector(vec![6., 10., 8.]));
//! ```
//! ## Equations:
//! ```
//! let equation = "x^2=9".to_string();
//!
//! let res = quick_solve(equation, "x".to_string(), vec![])?;
//! let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();
//! 
//! assert_eq!(res_rounded, vec![Value::Scalar(3.), Value::Scalar(-3.)]);
//! ```
//!
//! ```
//! let equation = "400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k".to_string();
//!
//! let res = quick_solve(equation, vec![])?;
//! let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();
//!
//! assert_eq!(res_rounded, vec![Value::Vector(vec![4., 6.])]);
//! ```
//! ## LaTeX:
//! ```
//! let expression = "((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))".to_string();
//! let parsed = parse(expression)?;
//! let vars = vec![Variable::new("x".to_string(), Value::Scalar(-0.655639))];
//! let result = eval(&parsed, &vars)?;
//! let var_assign = Step::Calc((Binary::Scalar(-0.655639), Value::Scalar(-0.655639), Some("x".to_string())));
//! let step = Step::Calc((parsed, result, None));
//! export(vec![var_assign, step], "export".to_string(), ExportType::Png);
//! ```
//!
//! Output (export-1.png):
//!
//! ![LaTeX][latex-export]

use errors::{QuickEvalError, QuickSolveError};

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

pub use basetypes::{Value, Variable};
pub use latex::Step;
#[cfg(feature = "output")]
pub use latex::{export_history, ExportType, png_from_latex, svg_from_latex};
pub use parser::{parse, eval};
pub use solver::solve;
pub use errors::MathLibError;
pub use basetypes::Store;

///defines the precision used by the equation solver and the printing precision, which is PREC-2.
#[cfg(feature = "high-prec")]
pub const PREC: i32 = 13;

///defines the precision used by the equation solver and the printing precision, which is PREC - 2.
#[cfg(not(feature = "high-prec"))]
pub const PREC: i32 = 8;

/// evaluates a given expression using the given variables (e and pi are provided by
/// the function). If you just want the Binary Tree, have a look at [parse()].
///
/// # Examples
/// ```
/// let res = quick_eval("3*3".to_string(), vec![])?;
///
/// assert_eq!(res, Value::Scalar(9.));
/// ```
///
/// ```
/// let x = Variable::new("x".to_string(), Value::Scalar(3.));
/// let res = quick_eval("3x".to_string(), vec![x])?;
///
/// assert_eq!(res, Value::Scalar(9.));
/// ```
pub fn quick_eval<S: Into<String>>(expr: S, state: Store) -> Result<Value, QuickEvalError> {
    let mut expr = expr.into();
    let mut context_vars = vec![
        Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E)),
        Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI))
    ];
    if !state.vars.is_empty() {
        if state.vars.iter().filter(|x| x.name == "e".to_string() || x.name == "pi".to_string()).collect::<Vec<&Variable>>().len() > 0 {
            return Err(QuickEvalError::DuplicateVars);
        }
        for i in state.vars {
            context_vars.push(i.clone());
        }
    }
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    let b_tree = parse(expr)?;
    
    Ok(eval(&b_tree, &Store::new(&context_vars, &state.funs))?)
}
