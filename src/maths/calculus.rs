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
                    if sums.len() <= i {
                        sums.push(e.clone());
                    } else {
                        sums[i] = add(&sums[i], &e)?;
                    }
                }
                mut_vars.remove(mut_vars.len()-1);
                b += dx;
            }
            for i in 0..sums.len() {
                sums[i] = mult(&sums[i], &Value::Scalar(dx))?;
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
pub fn calculate_derivative(expr: &AST, in_terms_of: &str, at: &Value, state: &mut Store) -> Result<Vec<Value>, EvalError> {
    for i in &state.vars {
        if i.name == in_terms_of {
            state.remove_var(i.name.clone());
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            state.add_var(&Variable::new(in_terms_of, at.clone()));
            let fxs = eval(expr, state)?;
            state.remove_var(in_terms_of);
            state.add_var(&Variable::new(in_terms_of, Value::Scalar(s+10f64.powi(-(PREC)))));
            let fxhs = &eval(expr, state)?;
            if fxs.len() != fxhs.len() {
                return Err(EvalError::MathError("Amount of solutions for f(x) and f(x+h) are different!".to_string()));
            }
            let mut res = vec![];
            for i in 0..fxs.len() {
                let h = AST::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Div,
                    left: AST::from_operation(Operation::SimpleOperation {
                        op_type: SimpleOpType::Sub,
                        left: AST::from_value(fxhs[i].clone()),
                        right: AST::from_value(fxs[i].clone())
                    }),
                    right: AST::from_value(Value::Scalar(10f64.powi(-(PREC))))
                });
                res.push(eval(&h, &state)?);
            }

            state.remove_var(in_terms_of);

            return Ok(res.into_iter().flatten().collect()); 
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}

pub fn calculate_derivative_newton(expr: &AST, in_terms_of: &str, at: &Value, mut fx: Option<Value>, state: &mut Store) -> Result<Value, EvalError> {
    for i in &state.vars {
        if i.name == in_terms_of {
            state.remove_var(in_terms_of);
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            if fx.is_none() {
                state.add_var(&Variable::new(in_terms_of, at.clone()));
                fx = Some(eval(expr, state)?[0].clone());
                state.remove_var(in_terms_of);
            }
            state.add_var(&Variable::new(in_terms_of, Value::Scalar(s+10f64.powi(-(PREC)))));
            let fxh = &eval(expr, state)?[0];
            let h = AST::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: AST::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: AST::from_value(fxh.clone()),
                    right: AST::from_value(fx.clone().unwrap().clone())
                }),
                right: AST::from_value(Value::Scalar(10f64.powi(-(PREC))))
            });
            let res = eval(&h, state)?[0].clone();
            state.remove_var(in_terms_of);
            return Ok(res);
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}
