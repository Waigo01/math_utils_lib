use crate::{basetypes::{Operation, SimpleOpType, Store, AST}, errors::EvalError, eval, Value, Variable, PREC};

use super::{add, mult};

/// calculates the integral of an expression in terms of a variable with a lower and a upper bound.
///
/// Only scalars are supported as lower and upper bounds.
pub fn calculate_integral(expr: &AST, in_terms_of: String, lower_bound: Value, upper_bound: Value, state: &Store) -> Result<Vec<Value>, EvalError> {
    let mut mut_vars = state.vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match (lower_bound, upper_bound) {
        (Value::Scalar(mut lb), Value::Scalar(mut ub)) => {
            if lb == ub {
                return Ok(vec![Value::Scalar(0.)])
            }
            if ub < lb {
                let temp = ub;
                ub = lb;
                lb = temp;
            }
            let dx = (ub-lb)/10f64.powi(PREC-2);
            let mut sums = vec![];
            let mut b = lb;
            while b < ub {
                mut_vars.push(Variable::new(&in_terms_of, Value::Scalar(b)));
                let evals = eval(expr, &Store::new(&mut_vars, &state.funs))?;
                for (i, e) in evals.iter().enumerate() {
                    if sums.len() < i {
                        sums.push(e.clone());
                    } else {
                        sums[i] = add(sums[i].clone(), e.clone())?;
                    }
                }
                mut_vars.remove(mut_vars.len()-1);
                b += dx;
            }
            for i in 0..sums.len() {
                sums[i] = mult(sums[i].clone(), Value::Scalar(dx))?;
            }

            return Ok(sums)
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
pub fn calculate_derivative(expr: &AST, in_terms_of: &str, at: &Value, mut fxs: Option<Vec<Value>>, state: &Store) -> Result<Vec<Value>, EvalError> {
    let mut mut_vars = state.vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            if fxs.is_none() {
                mut_vars.push(Variable::new(in_terms_of, at.clone()));
                fxs = Some(eval(expr, &Store::new(&mut_vars, &state.funs))?);
                mut_vars.remove(mut_vars.len()-1);
            }
            let fxs_unwraped = fxs.unwrap();
            mut_vars.push(Variable::new(in_terms_of, Value::Scalar(s+10f64.powi(-(PREC)))));
            let fxhs = &eval(expr, &Store::new(&mut_vars, &state.funs))?;
            if fxs_unwraped.len() != fxhs.len() {
                return Err(EvalError::MathError("Amount of solutions for f(x) and f(x+h) are different!".to_string()));
            }
            let mut res = vec![];
            for i in 0..fxs_unwraped.len() {
                let h = AST::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Div,
                    left: AST::from_operation(Operation::SimpleOperation {
                        op_type: SimpleOpType::Sub,
                        left: AST::from_value(fxhs[i].clone()),
                        right: AST::from_value(fxs_unwraped[i].clone())
                    }),
                    right: AST::from_value(Value::Scalar(10f64.powi(-(PREC))))
                });
                res.push(eval(&h, &Store::new(&mut_vars, &state.funs))?);
            }

            return Ok(res.into_iter().flatten().collect()); 
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}
