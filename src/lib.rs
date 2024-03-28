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
//! - solving equations
//! - exporting a LaTeX document from a collection of parsed expressions or solved equations (see 
//! [StepType](enum@latex_export::StepType))
//!
//! <div class="warning">Important Rules:
//!
//! Vectors and matrices can be written out in an expression (e.g [3, 5, 7] or [[3, 4, 5], [3, 4,
//! 5], [2, 6, 7]]) <br>
//! Variable Names can only start with an alphabetic letter or a \ (see [Variable]) <br>
//! See [OpType](enum@parser::OpType) for all allowed operations and functions.
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
//! let res = quick_eval("3*3".to_string(), None)?;
//!
//! assert_eq!(res, Value::Scalar(9.));
//! ```
//!
//! ```
//! let x = Variable {
//!     name: "x".to_string(),
//!     value: Value::Scalar(3.)
//! };
//! let res = quick_eval("3x".to_string(), Some(vec![x]))?;
//!
//! assert_eq!(res, Value::Scalar(9.));
//! ```
//!
//! ```
//! let a = Variable {
//!     name: "A".to_string(),
//!     value: Value::Vector(vec![3., 5., 8.])
//! };
//! let res = quick_eval("3A".to_string(), Some(vec![a]))?;
//!
//! assert_eq!(res, Value::Vector(vec![9., 15., 24.]));
//! ```
//!
//! ```
//! let a = Variable {
//!     name: "A".to_string(),
//!     value: Value::Vector(vec![3., 5., 8.])
//! };
//! let b = Variable {
//!     name: "B".to_string(),
//!     value: Value::Matrix(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 1.]])
//! };
//! let res = quick_eval("B*A".to_string(), Some(vec![a, b]))?;
//!
//! assert_eq!(res, Value::Vector(vec![6., 10., 8.]));
//! ```
//! ## Equations:
//! ```
//! let equation = "x^2=9".to_string();
//!
//! let res = quick_solve(equation, "x".to_string(), None)?;
//!
//! let res_rounded = res.iter().map(|x| Value::Scalar((x.get_scalar()*1000.).round()/1000.)).collect::<Vec<Value>>();
//!
//! assert_eq!(res_rounded, vec![Value::Scalar(3.), Value::Scalar(-3.)]);
//! ```
//! ## LaTeX:
//! ```
//! let expression = "((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))".to_string();
//! let parsed = parse(expression)?;
//! let vars = vec![Variable {
//!     name: "x".to_string(),
//!     value: Value::Scalar(-0.655639)
//! }];
//! let result = eval(&parsed, &vars)?;
//! let var_assign = StepType::Calc((Binary::Value(Value::Scalar(-0.655639)), Value::Scalar(-0.655639), Some("x".to_string())));
//! let step = StepType::Calc((parsed, result, None));
//! export(vec![var_assign, step], "export".to_string(), ExportType::Png);
//! ```
//!
//! Output (export-1.png):
//!
//! ![LaTeX][latex-export]

use errors::{QuickEvalError, QuickSolveError};
use parser::{Binary, OpType, Operation};

#[doc(hidden)]
pub mod maths;
pub mod basetypes;
#[doc(hidden)]
pub mod helpers;
pub mod latex_export;
pub mod parser;
pub mod errors;
pub mod roots;

#[cfg(test)]
mod tests;

pub use basetypes::{Value, Variable};
pub use latex_export::{export, ExportType, StepType};
pub use parser::{parse, eval};
pub use roots::find_roots;
pub use errors::MathLibError;

/// evaluates a given expression using the given variables (e and pi are provided by
/// the function). If you just want the Binary Tree, have look at [parse()].
///
/// # Examples
/// ```
/// let res = quick_eval("3*3".to_string(), None)?;
///
/// assert_eq!(res, Value::Scalar(9.));
/// ```
///
/// ```
/// let x = Variable {
///     name: "x".to_string(),
///     value: Value::Scalar(3.)
/// };
/// let res = quick_eval("3x".to_string(), Some(vec![x]))?;
///
/// assert_eq!(res, Value::Scalar(9.));
/// ```
pub fn quick_eval(mut expr: String, vars: Option<Vec<Variable>>) -> Result<Value, QuickEvalError> {
    let mut context_vars = vec![
        Variable {
            name: "e".to_string(),
            value: Value::Scalar(std::f64::consts::E)
        },
        Variable {
            name: "pi".to_string(),
            value: Value::Scalar(std::f64::consts::PI)
        }
    ];
    if vars.is_some() {
        let unwraped_vars = vars.unwrap();
        if unwraped_vars.iter().filter(|x| x.name == "e".to_string() || x.name == "pi".to_string()).collect::<Vec<&Variable>>().len() > 0 {
            return Err(QuickEvalError{ code: errors::QuickEvalErrorCode::DuplicateVars, reason: "Can't specify e and pi twice.".to_string()});
        }
        for i in unwraped_vars {
            context_vars.push(i);
        }
    }
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    let b_tree = parse(expr)?;
    
    Ok(eval(&b_tree, &context_vars)?)
}

/// solves a given equation towards a given Variable Name (solve_var). It can additionaly be
/// provided with other variables. If you just want a root finder, have a look at
/// [find_roots()](fn@roots::find_roots).
///
/// # Example
/// ```
/// let equation = "x^2=9".to_string();
///
/// let res = quick_solve(equation, "x".to_string(), None)?;
///
/// let res_rounded = res.iter().map(|x| Value::Scalar((x.get_scalar()*1000.).round()/1000.)).collect::<Vec<Value>>();
///
/// assert_eq!(res_rounded, vec![Value::Scalar(3.), Value::Scalar(-3.)]);
/// ```
pub fn quick_solve(mut expr: String, solve_var: String, vars: Option<Vec<Variable>>) -> Result<Vec<Value>, QuickSolveError> {
    let mut context_vars = vec![
        Variable {
            name: "e".to_string(),
            value: Value::Scalar(std::f64::consts::E)
        },
        Variable {
            name: "pi".to_string(),
            value: Value::Scalar(std::f64::consts::PI)
        }
    ];
    if vars.is_some() {
        let unwraped_vars = vars.unwrap();
        if unwraped_vars.iter().filter(|x| x.name == "e".to_string() || x.name == "pi".to_string()).collect::<Vec<&Variable>>().len() > 0 {
            return Err(QuickSolveError{ code: errors::QuickSolveErrorCode::DuplicateVars, reason: "Can't specify e and pi twice.".to_string()});
        }
        for i in unwraped_vars {
            context_vars.push(i);
        }
    }
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    if !expr.contains("=") {
        return Err(QuickSolveError { code: errors::QuickSolveErrorCode::NoEq, reason: "No = in equation!".to_string() });
    }

    let left = expr.split("=").nth(0).unwrap().to_string();
    let right = expr.split("=").nth(1).unwrap().to_string();

    let left_b;
    let right_b;
    if left.len() >= right.len() {
        left_b = parse(left)?;
        right_b = parse(right)?;
    } else {
        left_b = parse(right)?;
        right_b = parse(left)?;
    }

    let root_b = Binary::Operation(Box::new(Operation {
        op_type: OpType::Sub,
        left: left_b.clone(),
        right: right_b.clone()
    }));

    Ok(find_roots(root_b, context_vars, &solve_var)?)
}
