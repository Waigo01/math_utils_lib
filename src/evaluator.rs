use crate::{Context, Value, Values, Variable, basetypes::{AST, AdvancedOperation, Function, Operation, SimpleOpType}, errors::EvalError, helpers::cart_prod, maths, roots::RootFinder};

/// used to evaluate an AST with the provided context.
///
/// If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](fn@crate::quick_eval).
pub fn eval(b: &AST, context: &mut Context) -> Result<Values, EvalError> {
   Ok(Values::from_vec(eval_rec(b, context, "")?))
}

fn eval_rec(b: &AST, context: &mut Context, last_fn: &str) -> Result<Vec<Value>, EvalError> {
    match b {
        AST::Scalar(s) => return Ok(vec![Value::Scalar(*s)]),
        AST::Vector(v) => {
            let mut evaled_fields: Vec<Vec<f64>> = vec![];
            for i in &**v {
                let values = eval_rec(i, context, last_fn)?;
                for i in &values {
                    if i.get_scalar().is_none() {
                        return Err(EvalError::NonScalarInVector);
                    }
                }
                evaled_fields.push(values.iter().map(|v| v.get_scalar().unwrap()).collect());
            }

            let permuts: Vec<Vec<f64>> = cart_prod(&evaled_fields);

            return Ok(permuts.iter().map(|p| Value::Vector(p.to_vec())).collect());
        },
        AST::Matrix(m) => {
            let mut evaled_rows: Vec<Vec<Vec<f64>>> = vec![];
            for i in &**m {
                let mut row = vec![];
                for j in i {
                    let values = eval_rec(j, context, last_fn)?;
                    for i in &values {
                        if i.get_scalar().is_none() {
                            return Err(EvalError::NonScalarInMatrix);
                        }
                    }
                    row.push(values.iter().map(|v| v.get_scalar().unwrap()).collect());
                }
                evaled_rows.push(row);
            }
            let mut permuts_row: Vec<Vec<Vec<f64>>> = vec![];
            for i in evaled_rows {
                permuts_row.push(cart_prod(&i));
            }

            let permuts = cart_prod(&permuts_row);
            
            Ok(permuts.iter().map(|m| Value::Matrix(m.to_vec())).collect())
        },
        AST::List(l) => {
            return Ok(l.iter().map(|e| eval_rec(e, context, last_fn)).collect::<Result<Vec<Vec<Value>>, EvalError>>()?.into_iter().flatten().collect());
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
            if last_fn == name {
                return Err(EvalError::RecursiveFunction);
            }
            let function = context.get_fun(name);
            if function.is_none() {
                return Err(EvalError::NoFunction(name.to_string()));
            }

            let function = function.unwrap();
            
            if inputs.len() != function.inputs.len() {
                return Err(EvalError::WrongNumberOfArgs((function.inputs.len(), inputs.len())));
            }

            let mut eval_inputs = vec![];
            for i in inputs.iter() {
                eval_inputs.push(eval_rec(i, context, last_fn)?);
            }

            let permuts = cart_prod(&eval_inputs);

            let mut res = vec![];

            for p in permuts {
                let mut f_vars = vec![];
                for i in 0..inputs.len() {
                    f_vars.push(Variable::new(&function.inputs[i], vec![p[i].clone()]));
                }

                for i in context.vars.iter() {
                    if !f_vars.iter().map(|v| v.name.to_string()).collect::<Vec<String>>().contains(&i.name) {
                        f_vars.push(i.clone());
                    }
                }
                res.push(eval_rec(&function.ast, &mut Context::new(&f_vars, &context.funs), name)?);
            }

            return Ok(res.into_iter().flatten().collect());
        },
        AST::Operation(o) => {
            match &**o {
                Operation::SimpleOperation {op_type, left, right} => {
                    if *op_type == SimpleOpType::Assign {
                        if let AST::Variable(var_name) = left {
                            let rv = eval_rec(&right, context, last_fn)?;
                            context.add_var(&Variable::new(var_name, rv.clone()));
                            return Ok(rv);
                        } else if let AST::Function { name, inputs } = left {
                            let inputs: Vec<String> = inputs.iter().map(|i| if let AST::Variable(name) = i {name.to_string()} else {"".to_string()}).collect();
                            context.add_fun(&Function::new(name.to_string(), right.clone(), inputs));
                            return Ok(vec![]);
                        } else if let AST::List(list) = left {
                            let rv = eval_rec(&right, context, last_fn)?;
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
                    
                    let rv = eval_rec(&right, context, last_fn)?;
                    let lv = eval_rec(&left, context, last_fn)?;

                    let mut res = vec![];

                    for i in lv {
                        for j in &rv {
                            match op_type {
                                SimpleOpType::Get => res.push(maths::get(&i, &j)?),
                                SimpleOpType::Add => res.push(maths::add(&i, &j)?),
                                SimpleOpType::Sub => res.push(maths::sub(&i, &j)?),
                                SimpleOpType::AddSub => res.append(&mut vec![maths::add(&i, &j)?, maths::sub(&i, &j)?]),
                                SimpleOpType::Mult => res.push(maths::mult(&i, &j)?),
                                SimpleOpType::Neg => res.push(maths::neg(&j)?),
                                SimpleOpType::Div => res.push(maths::div(&i, &j)?),
                                SimpleOpType::Cross => res.push(maths::cross(&i, &j)?),
                                SimpleOpType::HiddenMult => res.push(maths::mult(&i, &j)?),
                                SimpleOpType::Pow => res.push(maths::pow(&i, &j)?),
                                SimpleOpType::Sin => res.push(maths::sin(&i)?),
                                SimpleOpType::Cos => res.push(maths::cos(&i)?),
                                SimpleOpType::Tan => res.push(maths::tan(&i)?),
                                SimpleOpType::Abs => res.push(maths::abs(&i)?),
                                SimpleOpType::Sqrt => res.push(maths::sqrt(&i)?),
                                SimpleOpType::Root => res.push(maths::root(&i, &j)?),
                                SimpleOpType::Ln => res.push(maths::ln(&i)?),
                                SimpleOpType::Arcsin => res.push(maths::arcsin(&i)?),
                                SimpleOpType::Arccos => res.push(maths::arccos(&i)?),
                                SimpleOpType::Arctan => res.push(maths::arctan(&i)?),
                                SimpleOpType::Det => res.push(maths::det(&i)?),
                                SimpleOpType::Inv => res.push(maths::inv(&i)?),
                                SimpleOpType::Parenths => res.push(i.clone()),
                                SimpleOpType::Assign => {},
                            }
                        }
                    }

                    return Ok(res);
                },
                Operation::AdvancedOperation(a) => {
                    match a {
                        AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                            let lb = eval_rec(&lower_bound, context, last_fn)?;
                            let ub = eval_rec(&upper_bound, context, last_fn)?;

                            let mut res = vec![];

                            for i in lb {
                                for j in &ub {
                                    res.push(maths::calculus::calculate_integral(&expr, in_terms_of.clone(), i.clone(), j.clone(), context)?);
                                }
                            }

                            return Ok(res.into_iter().flatten().collect());
                        },
                        AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                            let eat = eval_rec(&at, context, last_fn)?;

                            let mut res = vec![];

                            for i in eat {
                                let mut new_context = context.to_owned();
                                res.push(maths::calculus::calculate_derivative(&expr, &in_terms_of, &i, &mut new_context)?);
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
                            return root_finder.find_roots();
                        }
                    }
                }
            } 
        }
    }
}
