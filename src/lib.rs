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
//! [StepType](enum@latex_export::StepType) and [export()])
//!
//! Precision ([PREC]):
//!
//! Use feature high-prec for a precision of 13, standard precision is 8. The precision is used to
//! solve equations at the set precision. The actual output precision for printing is the defined precision - 2.
//!
//! <div class="warning">Important Rules:
//!
//! Vectors and matrices can be written out in an expression (e.g [3, 5, 7] or [[3, 4, 5], [3, 4,
//! 5], [2, 6, 7]]) <br>
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
//! let var_assign = StepType::Calc((Binary::Value(Value::Scalar(-0.655639)), Value::Scalar(-0.655639), Some("x".to_string())));
//! let step = StepType::Calc((parsed, result, None));
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
pub mod latex_export;
pub mod parser;
pub mod errors;
pub mod roots;
pub mod solver;

#[cfg(test)]
mod tests;

pub use basetypes::{Value, Variable};
pub use latex_export::{export, ExportType, StepType};
pub use parser::{parse, eval};
pub use solver::solve;
pub use errors::MathLibError;

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
pub fn quick_eval(mut expr: String, vars: Vec<Variable>) -> Result<Value, QuickEvalError> {
    let mut context_vars = vec![
        Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E)),
        Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI))
    ];
    if !vars.is_empty() {
        if vars.iter().filter(|x| x.name == "e".to_string() || x.name == "pi".to_string()).collect::<Vec<&Variable>>().len() > 0 {
            return Err(QuickEvalError::DuplicateVars);
        }
        for i in vars {
            context_vars.push(i);
        }
    }
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    let b_tree = parse(expr)?;
    
    Ok(eval(&b_tree, &context_vars)?)
}

/// solves an equation or a system of equations towards the variables not yet specified in vars. It can additionaly be
/// provided with other variables. If you just want to solve equations with parsed left and right
/// hand side binaries, have a look at [solve()](fn@solver::solve).
///
/// # Example
/// ```
/// let equation = "x^2=9".to_string();
///
/// let res = quick_solve(equation, "x".to_string(), vec![])?;
/// let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();
///
/// assert_eq!(res_rounded, vec![Value::Scalar(3.), Value::Scalar(-3.)]);
/// ```
/// ```
/// let equation = "400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k".to_string();
///
/// let res = quick_solve(equation, vec![])?;
/// let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();
///
/// assert_eq!(res_rounded, vec![Value::Vector(vec![4., 6.])]);
/// ```
pub fn quick_solve(mut expr: String, vars: Vec<Variable>) -> Result<Vec<Value>, QuickSolveError> {
    let mut context_vars = vec![
        Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E)),
        Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI))
    ];
    if !vars.is_empty() {
        if vars.iter().filter(|x| x.name == "e".to_string() || x.name == "pi".to_string()).collect::<Vec<&Variable>>().len() > 0 {
            return Err(QuickSolveError::DuplicateVars);
        }
        for i in &vars {
            context_vars.push(i.clone());
        }
    }
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    let mut equations = vec![];
    let mut parenths_open = 0;
    let mut buffer = String::new();

    for i in expr.chars() {
        if parenths_open == 0 && i == ',' {
            equations.push(buffer.clone());
            buffer.clear();
        } else {
            buffer.push(i);
        }

        if i == '(' || i == '[' || i == '{' {
            parenths_open += 1;
        } else if i == ')' || i == ']' || i == '}' {
            parenths_open -= 1;
        }
    }
    equations.push(buffer);

    let mut parsed_equations = vec![];

    for i in equations {
        if !expr.contains("=") {
            return Err(QuickSolveError::NoEq);
        }

        let left = i.split("=").nth(0).unwrap().to_string();
        let right = i.split("=").nth(1).unwrap().to_string();

        let left_b;
        let right_b;
        if left.len() >= right.len() {
            left_b = parse(left)?;
            right_b = parse(right)?;
        } else {
            left_b = parse(right)?;
            right_b = parse(left)?;
        }

        parsed_equations.push((left_b, right_b));
    }

    let roots = solve(parsed_equations, &vars)?;

    Ok(roots)
}
