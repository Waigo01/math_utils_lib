#[cfg(feature = "async")]
use async_macro::function_async;

use async_macro::async_call;

use crate::{Context, Value, Values, Variable, basetypes::{AST, AdvancedOperation, Function, InternalFunction, Operation, SimpleOpType}, errors::EvalError, helpers::cart_prod, maths::{self, num_traits::Number}, roots::RootFinder};

/// Used to evaluate an AST with the provided context.
///
/// If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](crate::quick_eval!).
#[cfg_attr(feature = "async", function_async)]
pub fn eval<N: Number>(tree: &AST<N>, context: &mut Context<N>) -> Result<Values<N>, EvalError> {
   Ok(Values::from_vec(async_call!(eval_rec(tree, context, "", 0))?))
}

#[cfg(debug_assertions)]
const MAX_RECURSION: i32 = 100;

#[cfg(not(debug_assertions))]
const MAX_RECURSION: i32 = 800;

#[cfg_attr(feature = "async", function_async)]
fn eval_rec<N: Number>(b: &AST<N>, context: &mut Context<N>, last_fn: &str, depth: i32) -> Result<Vec<Value<N>>, EvalError> {
    match b {
        AST::Scalar(s) => return Ok(vec![Value::Scalar(N::from(*s))]),
        AST::Vector(v) => {
            let mut evaled_fields: Vec<Vec<N>> = vec![];
            for i in &**v {
                let values = async_call!(eval_rec(i, context, last_fn, depth + 1))?;
                for i in &values {
                    if i.get_scalar().is_none() {
                        return Err(EvalError::NonScalarInVector);
                    }
                }
                evaled_fields.push(values.iter().map(|v| v.get_scalar().unwrap()).collect());
            }

            let permuts = cart_prod(&evaled_fields);

            return Ok(permuts.iter().map(|p| Value::Vector(p.to_vec())).collect());
        },
        AST::Matrix(m) => {
            let mut evaled_rows: Vec<Vec<Vec<N>>> = vec![];
            for i in &**m {
                let mut row = vec![];
                for j in i {
                    let values = async_call!(eval_rec(j, context, last_fn, depth + 1))?;
                    for i in &values {
                        if i.get_scalar().is_none() {
                            return Err(EvalError::NonScalarInMatrix);
                        }
                    }
                    row.push(values.iter().map(|v| v.get_scalar().unwrap()).collect());
                }
                evaled_rows.push(row);
            }
            let mut permuts_row: Vec<Vec<Vec<N>>> = vec![];
            for i in evaled_rows {
                permuts_row.push(cart_prod(&i));
            }

            let permuts = cart_prod(&permuts_row);
            
            Ok(permuts.iter().map(|m| Value::Matrix(m.to_vec())).collect())
        },
        AST::List(l) => {
            let mut evals = vec![];
            for e in l {
                evals.push(async_call!(eval_rec(e, context, last_fn, depth + 1))?);
            }
            return Ok(evals.into_iter().flatten().collect());
        }
        AST::Variable(v) => {
            for i in context.vars.iter() {
                if &i.name == v {
                    return Ok(i.values.clone().to_vec());
                }
            }

            return Err(EvalError::NoVariable(v.to_string()));
        },
        AST::Function { name, inputs } => {
            if last_fn == name && depth >= MAX_RECURSION {
                return Err(EvalError::RecursiveFunctionDepth);
            }

            let mut eval_inputs = vec![];
            for i in inputs.iter() {
                eval_inputs.push(async_call!(eval_rec(i, context, last_fn, depth + 1))?);
            }

            let permuts = cart_prod(&eval_inputs);

            let mut res = vec![];

            if let Some(InternalFunction{n_arguments, function, ..}) = context.get_internal_fun(name) {
                if inputs.len() != n_arguments {
                    return Err(EvalError::WrongNumberOfArgs((n_arguments, inputs.len())));
                }

                for p in permuts {
                    res.push(function(p)?);
                }
            } else if let Some(Function{ast, inputs: fn_inputs, ..}) = context.get_fun(name) {
                if inputs.len() != fn_inputs.len() {
                    return Err(EvalError::WrongNumberOfArgs((fn_inputs.len(), inputs.len())));
                }

                for p in permuts {
                    let mut copied_vars: Vec<Variable<N>> = vec![];
                    for i in 0..inputs.len() {
                        let var_name = &fn_inputs[i];
                        if let Some(var) = context.get_var(var_name) {
                            copied_vars.push(var);
                        }

                        context.add_var(&Variable::new(var_name, vec![p[i].clone()]));
                    }

                    res.append(&mut async_call!(eval_rec(&ast, context, name, depth+1))?);

                    for var_name in &fn_inputs {
                        context.remove_var(var_name);
                    }

                    for var in copied_vars {
                        context.add_var(&var);
                    }
                }
            } else {
                return Err(EvalError::NoFunction(name.to_string()))
            }

            return Ok(res);
        },
        AST::Operation(o) => {
            match &**o {
                Operation::SimpleOperation {op_type, left, right} => {
                    if *op_type == SimpleOpType::Assign {
                        if let AST::Variable(var_name) = left {
                            let rv = async_call!(eval_rec(&right, context, last_fn, depth + 1))?;
                            context.add_var(&Variable::new(var_name, rv.clone()));
                            return Ok(rv);
                        } else if let AST::Function { name, inputs } = left {
                            let inputs: Vec<String> = inputs.iter().map(|i| if let AST::Variable(name) = i {name.to_string()} else {"".to_string()}).collect();
                            context.add_fun(&Function::new(name.to_string(), right.clone(), inputs));
                            return Ok(vec![]);
                        } else if let AST::List(list) = left {
                            let rv = async_call!(eval_rec(&right, context, last_fn, depth + 1))?;
                            if rv.len() != list.len() {
                                return Err(EvalError::MultiVariableAssignmentItemNumber);
                            } else {
                                for (i, var) in list.iter().enumerate() {
                                    if let AST::Variable(name) = var {
                                        context.add_var(&Variable::new(name, rv[i].clone()));
                                    }
                                }
                            }
                            return Ok(rv);
                        }

                    }
                    
                    let rv = async_call!(eval_rec(&right, context, last_fn, depth + 1))?;
                    let lv = async_call!(eval_rec(&left, context, last_fn, depth + 1))?;

                    let mut res = vec![];

                    for i in &lv {
                        for j in &rv {
                            match op_type {
                                SimpleOpType::Get => res.push(async_call!(maths::get(i, j))?),
                                SimpleOpType::Add => res.push(async_call!(maths::add(i, j))?),
                                SimpleOpType::Sub => res.push(async_call!(maths::sub(i, j))?),
                                SimpleOpType::AddSub => res.append(&mut vec![async_call!(maths::add(i, j))?, async_call!(maths::sub(i, j))?]),
                                SimpleOpType::Mult => res.push(async_call!(maths::mult(i, j))?),
                                SimpleOpType::Neg => res.push(async_call!(maths::neg(j))?),
                                SimpleOpType::Div => res.push(async_call!(maths::div(i, j))?),
                                SimpleOpType::Cross => res.push(async_call!(maths::cross(i, j))?),
                                SimpleOpType::HiddenMult => res.push(async_call!(maths::mult(i, j))?),
                                SimpleOpType::Pow => res.push(async_call!(maths::pow(i, j))?),
                                SimpleOpType::Parenths => res.push(i.clone()),
                                SimpleOpType::BoolEq => res.push(async_call!(maths::bool::eq(i, j))?),
                                SimpleOpType::BoolNEq => res.push(async_call!(maths::bool::not_eq(i, j))?),
                                SimpleOpType::BoolLt => res.push(async_call!(maths::bool::lt(i, j))?),
                                SimpleOpType::BoolGt => res.push(async_call!(maths::bool::gt(i, j))?),
                                SimpleOpType::BoolLtEq => res.push(async_call!(maths::bool::lteq(i, j))?),
                                SimpleOpType::BoolGtEq => res.push(async_call!(maths::bool::gteq(i, j))?),
                                SimpleOpType::BoolNot => res.push(async_call!(maths::bool::not(j))?),
                                SimpleOpType::BoolAnd => res.push(async_call!(maths::bool::and(i, j))?),
                                SimpleOpType::BoolOr => res.push(async_call!(maths::bool::or(i, j))?),
                                SimpleOpType::Tetration => res.push(async_call!(maths::tetration(i, j))?),
                                SimpleOpType::Assign => {},
                            }
                        }
                    }

                    return Ok(res);
                },
                Operation::AdvancedOperation(a) => {
                    match a {
                        AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                            let lb = async_call!(eval_rec(&lower_bound, context, last_fn, depth + 1))?;
                            let ub = async_call!(eval_rec(&upper_bound, context, last_fn, depth + 1))?;

                            let mut res = vec![];

                            for i in lb {
                                for j in &ub {
                                    res.push(async_call!(maths::calculus::calculate_integral(&expr, in_terms_of.clone(), i.clone(), j.clone(), context, N::epsilon(), 15))?);
                                }
                            }

                            return Ok(res.into_iter().flatten().collect());
                        },
                        AdvancedOperation::Sum {expr, in_terms_of, lower_bound, upper_bound} => {
                            let lb = async_call!(eval_rec(&lower_bound, context, last_fn, depth + 1))?;
                            let ub = async_call!(eval_rec(&upper_bound, context, last_fn, depth + 1))?;

                            let mut res = vec![];

                            for i in lb {
                                for j in &ub {
                                    res.push(async_call!(maths::calculus::calculate_sum(&expr, in_terms_of.clone(), i.clone(), j.clone(), context))?);
                                }
                            }

                            return Ok(res.into_iter().flatten().collect());
                        },
                        AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                            let eat = async_call!(eval_rec(&at, context, last_fn, depth + 1))?;

                            let mut res = vec![];

                            for i in eat {
                                let mut new_context = context.to_owned();
                                res.push(async_call!(maths::calculus::calculate_derivative(&expr, &in_terms_of, &i, &mut new_context))?);
                            }

                            return Ok(res.into_iter().flatten().collect());
                        },
                        AdvancedOperation::Equation { equations, search_vars } => {
                            let mut final_expressions = vec![];

                            for i in equations {
                                let root_b = AST::from_operation(Operation::SimpleOperation {
                                    op_type: SimpleOpType::Sub,
                                    left: i.0.clone(),
                                    right: i.1.clone()
                                });

                                final_expressions.push(root_b);
                            }
                            let root_finder = RootFinder::new(final_expressions, context.to_owned(), search_vars.to_vec())?;
                            return async_call!(root_finder.find_roots());
                        },
                        AdvancedOperation::Conditional { condition, then, r#else } => {
                            let econdition = async_call!(eval_rec(&condition, context, last_fn, depth + 1))?;

                            let mut res = vec![];

                            for con in econdition {
                                if maths::bool::eq(&con, &Value::Scalar(N::zero()))? == Value::Scalar(N::zero()) {
                                    let ethen = async_call!(eval_rec(&then, context, last_fn, depth + 1))?;
                                    res.push(ethen);
                                } else {
                                    let eelse = if let Some(r#else) = r#else {Some(async_call!(eval_rec(r#else, context, last_fn, depth + 1))?)} else {None};
                                    if let Some(eelse) = eelse {
                                        res.push(eelse);
                                    }
                                }
                                
                            }

                            return Ok(res.into_iter().flatten().collect());
                        }
                    }
                }
            } 
        }
    }
}
