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
/// defines the precision used by the equation solver and the printing precision, which is PREC-2.
pub const PREC: usize = 13;

#[cfg(not(feature = "high-prec"))]
/// defines the precision used by the equation solver and the printing precision, which is PREC - 2.
pub const PREC: usize = 8;

/// evaluates a given expression in the given context. If you just want the AST, have a look at [parse()].
///
/// For more information about the context, take a look at [Context] and for more information about
/// the possible operations, take a look at [basetypes::SimpleOpType] and
/// [basetypes::AdvancedOpType].
///
/// # Examples
///
/// ```
/// let res = quick_eval("3*3", Context::default())?.to_vec();
///
/// assert_eq!(res, vec![Value::Scalar(9.)]);
/// ```
///
/// ```
/// let x = Variable::new("x".to_string(), vec![Value::Scalar(3.)]);
/// let res = quick_eval("3x".to_string(), &Context::from_vars(vec![x]))?.to_vec();
///
/// assert_eq!(res, vec![Value::Scalar(9.)]);
/// ```
pub fn quick_eval<S: Into<String>>(expr: S, context: &Context) -> Result<Values, QuickEvalError> {
    let expr = expr.into();
    let b_tree = parse(expr)?; 
    Ok(eval(&b_tree, &context)?)
}
