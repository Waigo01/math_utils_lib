use crate::{Value, Variable, basetypes::{AST, Context, Operation, SimpleOpType}, errors::EvalError, eval, helpers::replace_in_ast, maths::{abs, num_traits::Number, sub}};

use super::{add, mult};

const GAUSS_NODES: [f64; 7] = [0., 0.405845151377397, -0.405845151377397, 0.741531185599394, -0.741531185599394, 0.949107912342759, -0.949107912342759];
const GAUSS_WEIGHTS: [f64; 7] = [0.417959183673469, 0.381830050505119, 0.381830050505119, 0.279705391489277, 0.279705391489277, 0.129484966168870, 0.129484966168870];

const KRONROD_NODES: [f64; 15] = [0., 0.207784955007898, -0.207784955007898, 0.405845151377397, -0.405845151377397, 0.586087235467691, -0.586087235467691, 0.741531185599394, -0.741531185599394, 0.864864423359769, -0.864864423359769, 0.949107912342759, -0.949107912342759, 0.991455371120813, -0.991455371120813];
const KRONROD_WEIGHTS: [f64; 15] = [0.209482141084728, 0.204432940075298, 0.204432940075298, 0.190350578064785, 0.190350578064785, 0.169004726639267, 0.169004726639267, 0.140653259715525, 0.140653259715525, 0.104790010322250, 0.104790010322250, 0.063092092629979, 0.063092092629979, 0.022935322010529, 0.022935322010529];

fn calculate_gauss<N: Number>(scale: N, mean: N, f: &AST<N>, in_terms_of: &str, context: &mut Context<N>) -> Result<Vec<Value<N>>, EvalError> {
    let mut sums = vec![];
    for i in 0..GAUSS_NODES.len() {
        let node = N::from(GAUSS_NODES[i]) * scale + mean;
        let weight = N::from(GAUSS_WEIGHTS[i]) * scale;
        context.add_var(&Variable::new(in_terms_of, Value::Scalar(node)));
        let res = eval(f, context)?.to_vec();
        for j in 0..res.len() {
            if sums.len() <= j {
                sums.push(mult(&Value::Scalar(weight), &res[j])?);
            } else {
                sums[j] = add(&sums[j], &mult(&Value::Scalar(weight), &res[j])?)?;
            }
        }
    }
    Ok(sums)
}

fn calculate_kronrod<N: Number>(scale: N, mean: N, f: &AST<N>, in_terms_of: &str, context: &mut Context<N>) -> Result<Vec<Value<N>>, EvalError> {
    let mut sums = vec![];
    for i in 0..KRONROD_NODES.len() {
        let node = N::from(KRONROD_NODES[i]) * scale + mean;
        let weight = N::from(KRONROD_WEIGHTS[i]) * scale;
        context.add_var(&Variable::new(in_terms_of, Value::Scalar(node)));
        let res = eval(f, context)?.to_vec();
        for j in 0..res.len() {
            if sums.len() <= j {
                sums.push(mult(&Value::Scalar(weight), &res[j])?);
            } else {
                sums[j] = add(&sums[j], &mult(&Value::Scalar(weight), &res[j])?)?;
            }
        }
    }
    Ok(sums)
}

fn gauss_kronrod_recurse<N: Number>(expr: &AST<N>, in_terms_of: &str, lb: N, ub: N, context: &mut Context<N>, tolerance: N, depth: usize, mut recurse_tolerance: N) -> Result<Vec<Value<N>>, EvalError> {
    let mean = ub.mean(lb);
    let scale = ub.mean(-lb);
    let mut gauss = calculate_gauss(scale, mean, expr, &in_terms_of, context)?;
    let kronrod = calculate_kronrod(scale, mean, expr, &in_terms_of, context)?;
    let err = gauss.iter().enumerate().map(|(i, g)| Ok(abs(vec![sub(&kronrod[i], &g)?])?)).collect::<Result<Vec<Value<N>>, EvalError>>()?;


    for (i, e) in err.iter().enumerate() {
        let abs_tol = abs(vec![mult(&gauss[i], &Value::Scalar(tolerance))?])?.get_scalar().unwrap();
        if recurse_tolerance == N::zero() {
            recurse_tolerance = abs_tol;
        }

        if depth != 0 && abs_tol < e.get_scalar().unwrap() && recurse_tolerance < e.get_scalar().unwrap() {
            let m = lb.mean(ub);
            let lower = gauss_kronrod_recurse(expr, in_terms_of, lb, m, context, tolerance, depth-1, N::zero().mean(recurse_tolerance))?;
            let upper = gauss_kronrod_recurse(expr, in_terms_of, m, ub, context, tolerance, depth-1, N::zero().mean(recurse_tolerance))?;
            for i in 0..lower.len() {
                gauss[i] = add(&lower[i], &upper[i])?;
            }
            break;
        }
    }

    Ok(gauss)
}

/// calculates the integral of an expression in terms of a variable with a lower and a upper bound.
///
/// Only scalars are supported as lower and upper bounds.
pub fn calculate_integral<N: Number>(expr: &AST<N>, in_terms_of: String, lower_bound: Value<N>, upper_bound: Value<N>, context: &Context<N>, tolerance: N, depth: usize) -> Result<Vec<Value<N>>, EvalError> {
    let mut mut_context = context.clone();
    match (lower_bound, upper_bound) {
        (Value::Scalar(lb), Value::Scalar(ub)) => {
            if lb == ub {
                return Ok(vec![Value::Scalar(N::zero())])
            } else if ub.is_infinite() && lb.is_infinite() {

                let var_sq = AST::Operation(Box::new(Operation::SimpleOperation { 
                    op_type: SimpleOpType::Mult, 
                    left: AST::Variable(in_terms_of.clone()), 
                    right: AST::Variable(in_terms_of.clone())
                }));
                let inv = AST::Operation(Box::new(Operation::SimpleOperation { 
                    op_type: SimpleOpType::Div, 
                    left: AST::Scalar(N::one()),
                    right: AST::Operation(Box::new(Operation::SimpleOperation { 
                        op_type: SimpleOpType::Sub, 
                        left: AST::Scalar(N::one()), 
                        right: var_sq.clone()
                    }))
                }));
                let w = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Mult,
                    left: AST::Operation(Box::new(Operation::SimpleOperation { 
                        op_type: SimpleOpType::Add, 
                        left: AST::Scalar(N::one()), 
                        right: var_sq
                    })),
                    right: AST::Operation(Box::new(Operation::SimpleOperation { 
                        op_type: SimpleOpType::Mult, 
                        left: inv.clone(), 
                        right: inv.clone()
                    }))
                }));
                let arg = AST::Operation(Box::new(Operation::SimpleOperation { 
                    op_type: SimpleOpType::Mult, 
                    left: AST::Variable(in_terms_of.clone()), 
                    right: inv
                }));
                let final_expr = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Mult,
                    left: replace_in_ast(expr.clone(), &AST::Variable(in_terms_of.clone()), &arg),
                    right: w
                }));

                return gauss_kronrod_recurse(&final_expr, &in_terms_of, -N::one(), N::one(), &mut mut_context, tolerance, depth, N::zero());
            } else if ub.is_infinite() && lb.is_finite() {

                let z = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Div,
                    left: AST::Scalar(N::one()),
                    right: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Add,
                        left: AST::Variable(in_terms_of.clone()),
                        right: AST::Scalar(N::one())
                    }))
                }));
                let arg = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Add,
                    left: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Mult,
                        left: AST::Scalar(N::from(2.)),
                        right: z.clone()
                    })),
                    right: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Sub,
                        left: AST::Scalar(lb),
                        right: AST::Scalar(N::one())
                    }))
                }));
                let final_expr = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Mult,
                    left: replace_in_ast(expr.clone(), &AST::Variable(in_terms_of.clone()), &arg),
                    right: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Mult,
                        left: z.clone(),
                        right: z
                    }))
                }));

                return gauss_kronrod_recurse(&final_expr, &in_terms_of, -N::one(), N::one(), &mut mut_context, tolerance, depth, N::zero())?.into_iter().map(|g| Ok(mult(&Value::Scalar(N::from(2)), &g)?)).collect::<Result<Vec<Value<N>>, EvalError>>();
            } else if ub.is_finite() && lb.is_infinite() {

                let z = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Div,
                    left: AST::Scalar(N::one()),
                    right: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Add,
                        left: AST::Variable(in_terms_of.clone()),
                        right: AST::Scalar(N::one())
                    }))
                }));
                let arg = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Mult,
                        left: AST::Scalar(N::from(2.)),
                        right: z.clone()
                    })),
                    right: AST::Scalar(N::one())
                }));
                let new_x = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: AST::Scalar(ub),
                    right: arg
                }));
                let final_expr = AST::Operation(Box::new(Operation::SimpleOperation {
                    op_type: SimpleOpType::Mult,
                    left: replace_in_ast(expr.clone(), &AST::Variable(in_terms_of.clone()), &new_x),
                    right: AST::Operation(Box::new(Operation::SimpleOperation {
                        op_type: SimpleOpType::Mult,
                        left: z.clone(),
                        right: z
                    }))
                }));

                return gauss_kronrod_recurse(&final_expr, &in_terms_of, -N::one(), N::one(), &mut mut_context, tolerance, depth, N::zero())?.into_iter().map(|g| Ok(mult(&Value::Scalar(N::from(2)), &g)?)).collect::<Result<Vec<Value<N>>, EvalError>>();
            } else {
                if ub < lb {
                    return gauss_kronrod_recurse(expr, &in_terms_of, ub, lb, &mut mut_context, tolerance, depth, N::zero())?.into_iter().map(|g| Ok(mult(&Value::Scalar(-N::one()), &g)?)).collect::<Result<Vec<Value<N>>, EvalError>>();
                } else {
                    return gauss_kronrod_recurse(expr, &in_terms_of, lb, ub, &mut mut_context, tolerance, depth, N::zero());
                }
            }
            
        }
        _ => {return Err(EvalError::MathError("only scalar bounds are allowed".to_string()))}
    }
}
/// calculates the derivative of an expression in terms of a variable at a certain value.
///
/// Only scalars and vectors are supported as values.
///
/// The function also takes an optional fx value, which is the value f(x). This can be used in
/// order to increase performance by not having to calculate f(x) twice for an application such as
/// newtons method.
pub fn calculate_derivative<N: Number>(expr: &AST<N>, in_terms_of: &str, at: &Value<N>, context: &mut Context<N>) -> Result<Vec<Value<N>>, EvalError> {
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
            context.add_var(&Variable::new(in_terms_of, vec![Value::Scalar(*s+N::epsilon())]));
            let fxhs = &eval(expr, context)?.to_vec();
            if fxs.len() != fxhs.len() {
                return Err(EvalError::MathError("amount of solutions for f(x) and f(x+h) are different".to_string()));
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
                    right: AST::from_value(Value::Scalar(N::epsilon()))
                });
                res.push(eval(&h, context)?.to_vec());
            }

            context.remove_var(in_terms_of);

            return Ok(res.into_iter().flatten().collect()); 
        } 
        _ => {return Err(EvalError::MathError("only scalar values are allowed".to_string()))}
    }
}

pub fn calculate_derivative_newton<N: Number>(expr: &AST<N>, in_terms_of: &str, at: &Value<N>, mut fx: Option<Value<N>>, context: &mut Context<N>) -> Result<Value<N>, EvalError> {
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
            context.add_var(&Variable::new(in_terms_of, vec![Value::Scalar(*s+N::epsilon())]));
            let fxh = &eval(expr, context)?.get(0).unwrap().clone();
            let h = AST::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: AST::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: AST::from_value(fxh.clone()),
                    right: AST::from_value(fx.clone().unwrap().clone())
                }),
                right: AST::from_value(Value::Scalar(N::epsilon()))
            });
            let res = eval(&h, context)?.get(0).unwrap().clone();
            context.remove_var(in_terms_of);
            return Ok(res);
        } 
        _ => {return Err(EvalError::MathError("only scalar values are allowed".to_string()))}
    }
}

fn sum<N: Number>(expr: &AST<N>, in_terms_of: &str, lb: i32, ub: i32, context: &mut Context<N>) -> Result<Vec<Value<N>>, EvalError> {
    let mut sums = vec![];
    for b in lb..=ub {
        context.add_var(&Variable::new(in_terms_of, vec![Value::Scalar(b.into())]));
        let evals = eval(expr, context)?;
        for (i, e) in evals.to_vec().iter().enumerate() {
            if sums.len() <= i {
                sums.push(e.clone());
            } else {
                sums[i] = add(&sums[i], &e)?;
            }
        }
    }
    Ok(sums)
}

const MAX_TERMS: i32 = 100_000;

#[doc(hidden)]
pub fn calculate_sum<N: Number>(expr: &AST<N>, in_terms_of: String, lower_bound: Value<N>, upper_bound: Value<N>, context: &Context<N>) -> Result<Vec<Value<N>>, EvalError> {
    let mut mut_context = context.clone();
    match (lower_bound, upper_bound) {
        (Value::Scalar(lb), Value::Scalar(ub)) if let Ok(lb) = lb.as_rounded_int() && let Ok(ub) = ub.as_rounded_int() => {
            if lb == ub {
                return Ok(vec![Value::Scalar(N::zero())])
            } else if ub-lb > MAX_TERMS {
                
                let mut sums = sum(expr, &in_terms_of, lb, lb+MAX_TERMS, &mut mut_context)?;
                let integral;
                if ub == i32::MAX {
                    integral = calculate_integral(expr, in_terms_of.clone(), Value::Scalar(N::from(lb+MAX_TERMS)), Value::Scalar(N::infinity()), context, N::epsilon(), 10)?;
                } else {
                    integral = gauss_kronrod_recurse(expr, &in_terms_of, N::from(lb+MAX_TERMS), N::from(ub), &mut mut_context, N::epsilon(), 10, N::zero())?;
                }
                for i in 0..sums.len() {
                    sums[i] = add(&sums[i], &integral[i])?;
                }

                return Ok(sums);
            } else if ub < lb {
                return sum(expr, &in_terms_of, ub, lb, &mut mut_context)?.into_iter().map(|g| Ok(mult(&Value::Scalar(-N::one()), &g)?)).collect::<Result<Vec<Value<N>>, EvalError>>();
            } else {
                return sum(expr, &in_terms_of, lb, ub, &mut mut_context);
            }
        }
        _ => {return Err(EvalError::MathError("only integer bounds are allowed".to_string()))}
    }
}
