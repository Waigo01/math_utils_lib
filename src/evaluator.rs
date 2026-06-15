use crate::{Context, Value, Values, Variable, basetypes::{AST, AdvancedOperation, Function, Operation, SimpleOpType}, errors::EvalError, helpers::cart_prod, maths::{self, num_traits::Number}, roots::RootFinder};

/// used to evaluate an AST with the provided context.
///
/// If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](crate::quick_eval!).
pub fn eval<N: Number>(b: &AST<N>, context: &mut Context<N>) -> Result<Values<N>, EvalError> {
   Ok(Values::from_vec(eval_rec(b, context, "")?))
}

fn eval_rec<N: Number>(b: &AST<N>, context: &mut Context<N>, last_fn: &str) -> Result<Vec<Value<N>>, EvalError> {
    match b {
        AST::Scalar(s) => return Ok(vec![Value::Scalar(N::from(*s))]),
        AST::Vector(v) => {
            let mut evaled_fields: Vec<Vec<N>> = vec![];
            for i in &**v {
                let values = eval_rec(i, context, last_fn)?;
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
            let mut permuts_row: Vec<Vec<Vec<N>>> = vec![];
            for i in evaled_rows {
                permuts_row.push(cart_prod(&i));
            }

            let permuts = cart_prod(&permuts_row);
            
            Ok(permuts.iter().map(|m| Value::Matrix(m.to_vec())).collect())
        },
        AST::List(l) => {
            return Ok(l.iter().map(|e| eval_rec(e, context, last_fn)).collect::<Result<Vec<Vec<Value<N>>>, EvalError>>()?.into_iter().flatten().collect());
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

            let mut eval_inputs = vec![];
            for i in inputs.iter() {
                eval_inputs.push(eval_rec(i, context, last_fn)?);
            }

            let permuts = cart_prod(&eval_inputs);

            let mut res = vec![];

            match name.as_str() {
                "sin" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::sin(&p[0])?)},
                "sin" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "cos" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::cos(&p[0])?)},
                "cos" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "tan" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::tan(&p[0])?)},
                "tan" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "abs" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::abs(&p[0])?)},
                "abs" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "sqrt" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::sqrt(&p[0])?)},
                "sqrt" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "root" if eval_inputs.len() == 2 => for p in permuts {res.push(maths::root(&p[0], &p[1])?)},
                "root" => return Err(EvalError::WrongNumberOfArgs((2, eval_inputs.len()))),
                "ln" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::ln(&p[0])?)},
                "ln" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "arcsin" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::arcsin(&p[0])?)},
                "arcsin" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "arccos" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::arccos(&p[0])?)},
                "arccos" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "arctan" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::arctan(&p[0])?)},
                "arctan" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "det" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::det(&p[0])?)},
                "det" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                "inv" if eval_inputs.len() == 1 => for p in permuts {res.push(maths::inv(&p[0])?)},
                "inv" => return Err(EvalError::WrongNumberOfArgs((1, eval_inputs.len()))),
                _ if let Some(function) = context.get_fun(name) => {
                    if inputs.len() != function.inputs.len() {
                        return Err(EvalError::WrongNumberOfArgs((function.inputs.len(), inputs.len())));
                    }

                    for p in permuts {
                        let mut copied_vars: Vec<Variable<N>> = vec![];
                        for i in 0..inputs.len() {
                            let var_name = &function.inputs[i];
                            if let Some(var) = context.get_var(var_name) {
                                copied_vars.push(var);
                            }

                            context.add_var(&Variable::new(var_name, vec![p[i].clone()]));
                        }

                        res.append(&mut eval_rec(&function.ast, context, name)?);

                        for var_name in &function.inputs {
                            context.remove_var(var_name);
                        }

                        for var in copied_vars {
                            context.add_var(&var);
                        }
                    }
                },
                _ => {return Err(EvalError::NoFunction(name.to_string()))}
            }

            return Ok(res);
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

                    for i in &lv {
                        for j in &rv {
                            match op_type {
                                SimpleOpType::Get => res.push(maths::get(i, j)?),
                                SimpleOpType::Add => res.push(maths::add(i, j)?),
                                SimpleOpType::Sub => res.push(maths::sub(i, j)?),
                                SimpleOpType::AddSub => res.append(&mut vec![maths::add(i, j)?, maths::sub(i, j)?]),
                                SimpleOpType::Mult => res.push(maths::mult(i, j)?),
                                SimpleOpType::Neg => res.push(maths::neg(j)?),
                                SimpleOpType::Div => res.push(maths::div(i, j)?),
                                SimpleOpType::Cross => res.push(maths::cross(i, j)?),
                                SimpleOpType::HiddenMult => res.push(maths::mult(i, j)?),
                                SimpleOpType::Pow => res.push(maths::pow(i, j)?),
                                SimpleOpType::Parenths => res.push(i.clone()),
                                SimpleOpType::BoolEq => res.push(maths::bool::eq(i, j)?),
                                SimpleOpType::BoolNEq => res.push(maths::bool::not_eq(i, j)?),
                                SimpleOpType::BoolLt => res.push(maths::bool::lt(i, j)?),
                                SimpleOpType::BoolGt => res.push(maths::bool::gt(i, j)?),
                                SimpleOpType::BoolLtEq => res.push(maths::bool::lteq(i, j)?),
                                SimpleOpType::BoolGtEq => res.push(maths::bool::gteq(i, j)?),
                                SimpleOpType::BoolNot => res.push(maths::bool::not(j)?),
                                SimpleOpType::BoolAnd => res.push(maths::bool::and(i, j)?),
                                SimpleOpType::BoolOr => res.push(maths::bool::or(i, j)?),
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
                        },
                        AdvancedOperation::Conditional { condition, then, r#else } => {
                            let econdition = eval_rec(&condition, context, last_fn)?;

                            let mut res = vec![];

                            for con in econdition {
                                if maths::bool::eq(&con, &Value::Scalar(N::ZERO))? == Value::Scalar(N::ZERO) {
                                    let ethen = eval_rec(&then, context, last_fn)?;
                                    res.push(ethen);
                                } else {
                                    let eelse = if let Some(r#else) = r#else {Some(eval_rec(r#else, context, last_fn)?)} else {None};
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
