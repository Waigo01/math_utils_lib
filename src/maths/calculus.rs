use crate::{basetypes::{Operation, SimpleOpType, Context, AST}, errors::EvalError, eval, Value, Variable, PREC};

use super::{add, mult};

/// calculates the integral of an expression in terms of a variable with a lower and a upper bound.
///
/// Only scalars are supported as lower and upper bounds.
pub fn calculate_integral(expr: &AST, in_terms_of: String, lower_bound: Value, upper_bound: Value, context: &Context) -> Result<Vec<Value>, EvalError> {
    let mut mut_vars = context.vars.to_owned();
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
            let dx = (ub-lb)/10f64.powi(PREC as i32-3);
            let mut sums = vec![];
            let mut b = lb;
            while b < ub {
                mut_vars.push(Variable::new(&in_terms_of, vec![Value::Scalar((b+(b-dx))/2.0)]));
                let evals = eval(expr, &Context::new(&mut_vars, &context.funs))?;
                for (i, e) in evals.to_vec().iter().enumerate() {
                    if sums.len() <= i {
                        sums.push(e.clone());
                    } else {
                        sums[i] = add(&sums[i], &e)?;
                    }
                }
                mut_vars.remove(mut_vars.len()-1);
                b += dx;
            }
            println!("{:.?}", sums);
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
pub fn calculate_derivative(expr: &AST, in_terms_of: &str, at: &Value, context: &mut Context) -> Result<Vec<Value>, EvalError> {
    for i in &context.vars {
        if i.name == in_terms_of {
            context.remove_var(i.name.clone());
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            context.add_var(&Variable::new(in_terms_of, vec![at.clone()]));
            let fxs = eval(expr, context)?.to_vec();
            context.remove_var(in_terms_of);
            context.add_var(&Variable::new(in_terms_of, vec![Value::Scalar(s+10f64.powi(-(PREC as i32)))]));
            let fxhs = &eval(expr, context)?.to_vec();
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
                    right: AST::from_value(Value::Scalar(10f64.powi(-(PREC as i32))))
                });
                res.push(eval(&h, &context)?.to_vec());
            }

            context.remove_var(in_terms_of);

            return Ok(res.into_iter().flatten().collect()); 
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}

pub fn calculate_derivative_newton(expr: &AST, in_terms_of: &str, at: &Value, mut fx: Option<Value>, context: &mut Context) -> Result<Value, EvalError> {
    for i in &context.vars {
        if i.name == in_terms_of {
            context.remove_var(in_terms_of);
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            if fx.is_none() {
                context.add_var(&Variable::new(in_terms_of, vec![at.clone()]));
                fx = Some(eval(expr, context)?.get(0).unwrap().clone());
                context.remove_var(in_terms_of);
            }
            context.add_var(&Variable::new(in_terms_of, vec![Value::Scalar(s+10f64.powi(-(PREC as i32)))]));
            let fxh = &eval(expr, context)?.get(0).unwrap().clone();
            let h = AST::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: AST::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: AST::from_value(fxh.clone()),
                    right: AST::from_value(fx.clone().unwrap().clone())
                }),
                right: AST::from_value(Value::Scalar(10f64.powi(-(PREC as i32))))
            });
            let res = eval(&h, context)?.get(0).unwrap().clone();
            context.remove_var(in_terms_of);
            return Ok(res);
        } 
        _ => {return Err(EvalError::MathError("Only scalar values are allowed!".to_string()))}
    }
}
