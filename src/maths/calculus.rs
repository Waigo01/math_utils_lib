use crate::{errors::EvalError, eval, parser::{Binary, Operation, SimpleOpType}, Value, Variable, PREC};

use super::{add, mult};

/// calculates the integral of an expression in terms of a variable with a lower and a upper bound.
///
/// Only scalars are supported as lower and upper bounds.
pub fn calculate_integral(expr: &Binary, in_terms_of: String, lower_bound: Value, upper_bound: Value, vars: &Vec<Variable>) -> Result<Value, EvalError> {
    let mut mut_vars = vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match (lower_bound, upper_bound) {
        (Value::Scalar(mut lb), Value::Scalar(mut ub)) => {
            if lb == ub {
                return Ok(Value::Scalar(0.))
            }
            if ub < lb {
                let temp = ub;
                ub = lb;
                lb = temp;
            }
            let dx = (ub-lb)/10f64.powi(PREC-2);
            let mut sum = Value::Scalar(0.);
            let mut b = lb;
            while b < ub {
                mut_vars.push(Variable::from_value(in_terms_of.clone(), Value::Scalar(b)));
                sum = add(sum, eval(expr, &mut_vars)?)?;
                mut_vars.remove(mut_vars.len()-1);
                b += dx;
            }
            return Ok(mult(sum, Value::Scalar(dx))?);
        }
        _ => {return Err(EvalError::MathError("Only scalar bounds are allowed!".to_string()))}
    }
}
/// calculates the derivative of an expression in terms of a variable at a certain value.
///
/// Only scalars and vectors are supported as values.
///
/// The function also takes an optional fx value, which is the value f(x). This can be used in
/// order to increase performance by not having to calculate f(x) twice for an application such as
/// newtons method.
pub fn calculate_derivative(expr: &Binary, in_terms_of: String, at: Value, mut fx: Option<Value>, vars: &Vec<Variable>) -> Result<Value, EvalError> {
    let mut mut_vars = vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            if fx.is_none() {
                mut_vars.push(Variable::from_value(in_terms_of.clone(), at));
                fx = Some(eval(expr, &mut_vars)?);
                mut_vars.remove(mut_vars.len()-1);
            }
            mut_vars.push(Variable::from_value(in_terms_of.clone(), Value::Scalar(s+10f64.powi(-(PREC)))));
            let fxh = &eval(expr, &mut_vars)?;
            let h = Binary::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: Binary::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: Binary::from_value(fxh.clone()),
                    right: Binary::from_value(fx.clone().unwrap().clone())
                }),
                right: Binary::from_value(Value::Scalar(10f64.powi(-(PREC))))
            });
            return Ok(eval(&h, &mut_vars)?);
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}
