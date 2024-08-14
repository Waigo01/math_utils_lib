use crate::{basetypes::{AdvancedOpType, AdvancedOperation, Operation, SimpleOpType, Value, Variable, AST}, errors::{EvalError, ParserError}, maths, Store};

fn get_op_symbol(c: char) -> Option<SimpleOpType> {
    match c {
        '?' => Some(SimpleOpType::Get),
        '+' => Some(SimpleOpType::Add),
        '-' => Some(SimpleOpType::Sub),
        '*' => Some(SimpleOpType::Mult),
        '/' => Some(SimpleOpType::Div),
        '^' => Some(SimpleOpType::Pow),
        '#' => Some(SimpleOpType::Cross),
        _ => None
    }
}

fn parse_value(s: String) -> Result<AST, ParserError> {
    if !s.contains(&"[") {
        let p = match s.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(ParserError::ParseValue(s))
        };
        return Ok(AST::Scalar(p));
    } else if s.len() > 2 {
        if s.len() > 4 && s[1..s.len()-1].contains(&"[") && s.chars().nth(1).unwrap() == '[' && s.chars().nth(s.len()-2).unwrap() == ']' {
            let mut output_m = vec![];
            let mut row = vec![];
            let mut row_size = None;
            let mut n_buffer = String::new();
            let mut open_parenths = 0;
            for i in s[1..s.len()-1].chars().collect::<Vec<char>>() {
                if i == '[' {
                    open_parenths += 1;
                    continue;
                }
                if i == ']' {
                    open_parenths -= 1;
                    continue;
                }
                if open_parenths > 0 {
                    if i == ',' {
                        if n_buffer.is_empty() {
                            return Err(ParserError::EmptyVec)
                        }
                        row.push(parse(&n_buffer)?);
                        n_buffer.clear();
                    } else {
                        n_buffer.push(i);
                    }
                } else if open_parenths == 0 {
                    if i == ',' {
                        if n_buffer.is_empty() {
                            return Err(ParserError::EmptyVec)
                        }
                        row.push(parse(&n_buffer)?);
                        if row_size.is_some() && row.len() != row_size.unwrap() {
                            return Err(ParserError::NotRectMatrix)
                        }
                        row_size = Some(row.len());
                        output_m.push(row.clone());
                        n_buffer.clear();
                        row.clear();
                    }
                }
            }
            if open_parenths != 0 {
                return Err(ParserError::MissingBracket)
            }
            if n_buffer.is_empty() {
                return Err(ParserError::EmptyVec)
            }
            row.push(parse(&n_buffer)?);
            if row_size.is_some() && row.len() != row_size.unwrap() {
                return Err(ParserError::NotRectMatrix)
            }
            output_m.push(row);
            #[cfg(feature = "column-major")]
            let mut col_matrix = vec![];
            #[cfg(feature = "column-major")]
            for i in 0..output_m[0].len() {
                let mut row = vec![];
                for j in 0..output_m.len() {
                    row.push(output_m[j][i].clone());
                }
                col_matrix.push(row);
            }
            #[cfg(feature = "column-major")]
            return Ok(AST::Matrix(Box::new(col_matrix)));
            #[cfg(not(feature = "column-major"))]
            return Ok(AST::Matrix(Box::new(output_m)));
        } else if s.chars().nth(0).unwrap() == '[' && s.chars().nth(s.len()-1).unwrap() == ']' {
            let mut output_v = vec![];
            let mut n_buffer = String::new();
            for i in s[1..s.len()].chars().collect::<Vec<char>>() {
                if i == ',' {
                    if n_buffer.is_empty() {
                        return Err(ParserError::EmptyVec)
                    }
                    output_v.push(parse(&n_buffer)?);
                    n_buffer.clear();
                } else {
                    n_buffer.push(i);
                }
            }
            if n_buffer.is_empty() {
                return Err(ParserError::EmptyVec);
            }
            output_v.push(parse(n_buffer[0..n_buffer.len()-1].to_string())?);
            return Ok(AST::Vector(Box::new(output_v)));
        } else {
            return Err(ParserError::MissingBracket)
        }
    } else {
        return Err(ParserError::ParseValue(s));
    }
}

///used to construct a AST Tree from a mathematical expression.
pub fn parse<S: Into<String>>(expr: S) -> Result<AST, ParserError> {
    let expr = expr.into();
    if expr.is_empty() {
        return Err(ParserError::EmptyExpr);
    }
    let mut expr_chars = expr.chars().collect::<Vec<char>>();

    let mut parenths_open = 0;
    let mut check_parenths = true;
    for i in 0..expr_chars.len() {
        if expr_chars[i] == '(' {
            parenths_open += 1;
        }
        if expr_chars[i] == ')' {
            parenths_open -= 1;
            if parenths_open == 0 && i != expr_chars.len()-1 {
                check_parenths = false;
            }
        }
    }

    if parenths_open > 0 {
        return Err(ParserError::UnmatchedOpenDelimiter);
    } else if parenths_open < 0 {
        return Err(ParserError::UnmatchedCloseDelimiter);
    }

    if check_parenths {
        if expr_chars[0] == '(' && expr_chars[expr_chars.len()-1] == ')' {
            expr_chars = expr_chars[1..expr_chars.len()-1].iter().map(|c| *c).collect::<Vec<char>>();
            return Ok(AST::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Parenths,
                left: parse(expr_chars.iter().collect::<String>())?,
                right: AST::from_value(Value::Scalar(0.)) 
            }));
        }
    }


    //is it an operation?
    
    let op_types = vec![SimpleOpType::Add, SimpleOpType::Sub, SimpleOpType::Mult, SimpleOpType::Div, SimpleOpType::Cross, SimpleOpType::HiddenMult, SimpleOpType::Pow, SimpleOpType::Get];
    let mut ops_in_expr: Vec<(SimpleOpType, usize, usize, usize)> = vec![];
    let mut highest_op = 7;
    let mut last_char = '\\';
    let mut brackets_open = 0;
    let mut curly_brackets_open = 0;
    for i in 0..expr_chars.len() {
        let mut is_hidden_mult = false;
        if (last_char.is_digit(10) && (expr_chars[i].is_alphabetic() || expr_chars[i] == '\\' || expr_chars[i] == '(' || expr_chars[i] == '['))||(last_char == ')' && expr_chars[i] == '(') {
            is_hidden_mult = true;
            if i as i32-2 > 0 && expr_chars[i-2] == '_' {
                is_hidden_mult = false;
            }
        }
        if parenths_open == 0 && brackets_open == 0 && curly_brackets_open == 0 && is_hidden_mult {
            ops_in_expr.push((SimpleOpType::HiddenMult, i, 0, 0));
        }
        last_char = expr_chars[i];
        if expr_chars[i] == '(' {
            parenths_open += 1;
            continue;
        }
        if expr_chars[i] == ')' {
            parenths_open -= 1;
            continue;
        }
        if expr_chars[i] == '[' {
            brackets_open += 1;
            continue;
        }
        if expr_chars[i] == ']' {
            brackets_open -= 1;
            continue;
        }
        if expr_chars[i] == '{' {
            curly_brackets_open += 1;
            continue;
        }
        if expr_chars[i] == '}' {
            curly_brackets_open -= 1;
            continue;
        }
        let symbol = get_op_symbol(expr_chars[i]);
        if parenths_open == 0 && brackets_open == 0 && curly_brackets_open == 0 && i != 0 && i != expr_chars.len()-1 && symbol.is_some() {
            ops_in_expr.push((symbol.unwrap(), i, 0, 1));
        } 
    }

    for i in &ops_in_expr {
        for (j, o) in op_types.iter().enumerate() {
            if *o == i.0 && j < highest_op {
                highest_op = j;
                break;
            }
        }
    }

    if highest_op == 1 || highest_op == 3 {
        ops_in_expr.reverse();
    }

    for o in op_types {
        for i in &ops_in_expr {
            if i.0 == o {
                let left_b = parse(expr_chars[0..(i.1-i.2)].to_vec().iter().collect::<String>())?;
                let right_b = parse(expr_chars[(i.1+i.3)..].to_vec().iter().collect::<String>())?; 
                return Ok(AST::from_operation(Operation::SimpleOperation {
                    op_type: i.0.clone(),
                    left: left_b,
                    right: right_b
                }));
            }
        }
    }

    // is it a negation?

    if expr_chars[0] == '-' {
        return Ok(AST::from_operation(Operation::SimpleOperation {
            op_type: SimpleOpType::Neg,
            left: parse(expr_chars[1..].to_vec().iter().collect::<String>())?,
            right: AST::from_value(Value::Scalar(0.))
        }));
    }

    // is it a function?

    let function_look_up = vec![(SimpleOpType::Sin, "sin("), (SimpleOpType::Cos, "cos("), (SimpleOpType::Tan, "tan("), (SimpleOpType::Abs, "abs("), (SimpleOpType::Sqrt, "sqrt("), (SimpleOpType::Ln, "ln("), (SimpleOpType::Arcsin, "arcsin("), (SimpleOpType::Arccos, "arccos("), (SimpleOpType::Arctan, "arctan(")];
    
    for i in function_look_up {
        if expr_chars.iter().collect::<String>().starts_with(i.1) {
            let left_b = parse(expr_chars[i.1.len()..expr_chars.len()-1].to_vec().iter().collect::<String>())?;
            return Ok(AST::from_operation(Operation::SimpleOperation {
                op_type: i.0,
                left: left_b,
                right: AST::from_value(Value::Scalar(0.))
            }));
        }
    }

    // is it an advanced operation?

    let advanced_op_look_up = vec![(AdvancedOpType::Integral, "I("), (AdvancedOpType::Derivative, "D(")];

    for i in advanced_op_look_up {
        if expr_chars.iter().collect::<String>().starts_with(i.1) {
            match i.0 {
                AdvancedOpType::Derivative => {
                    let aop_string = expr_chars[i.1.len()..expr_chars.len()-1].to_vec().iter().collect::<String>();

                    let mut args = vec![];
                    parenths_open = 0;
                    let mut buffer = String::new();
                    for j in aop_string.chars() {
                        if parenths_open == 0 && j == ',' {
                            args.push(buffer.clone());
                            buffer.clear();
                        } else {
                            buffer.push(j);
                        }
                        if j == '(' || j == '[' || j == '{' {
                            parenths_open += 1;
                        } else if j == ')' || j == ']' || j == '}' {
                            parenths_open -= 1;
                        } 
                    }
                    args.push(buffer);
                    if args.len() != 3 {
                        return Err(ParserError::WrongNumberOfArgs("derivative".to_string()));
                    }
                    let parsed_function = parse(&args[0])?;
                    let parsed_value_at = parse(&args[2])?;
                    return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Derivative {
                        expr: parsed_function,
                        in_terms_of: args[1].clone(),
                        at: parsed_value_at
                    })));
                },
                AdvancedOpType::Integral => {
                    let aop_string = expr_chars[i.1.len()..expr_chars.len()-1].to_vec().iter().collect::<String>();

                    let mut args = vec![];
                    parenths_open = 0;
                    let mut buffer = String::new();
                    for j in aop_string.chars() {
                        if parenths_open == 0 && j == ',' {
                            args.push(buffer.clone());
                            buffer.clear();
                        } else {
                            buffer.push(j);
                        }
                        if j == '(' || j == '[' || j == '{' {
                            parenths_open += 1;
                        } else if j == ')' || j == ']' || j == '}' {
                            parenths_open -= 1;
                        }                    }
                    args.push(buffer);
                    if args.len() != 4 {
                        return Err(ParserError::WrongNumberOfArgs("integral".to_string()));
                    }
                    let parsed_function = parse(&args[0])?;
                    let parsed_lower_b = parse(&args[2])?;
                    let parsed_upper_b = parse(&args[3])?;
                    return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Integral {
                        expr: parsed_function,
                        in_terms_of: args[1].clone(),
                        lower_bound: parsed_lower_b,
                        upper_bound: parsed_upper_b
                    })));
                } 
            }
        }
    }
    
    // is it a custom function?

    if expr.contains("(") && expr.find("(").unwrap() != 0 && *expr_chars.last().unwrap() == ')' {
        let first_parenth = expr.find("(").unwrap();
        let mut inputs = vec![];
        let mut buffer = String::new();
        parenths_open = 0;
        for i in expr_chars[first_parenth+1..expr_chars.len()-1].iter() {
            if *i == '(' || *i == '[' || *i == '{' {
                parenths_open += 1;
            }
            if *i == ')' || *i == ']' || *i == '}' {
                parenths_open -= 1;
            } 
            if *i == ',' && parenths_open == 0 {
                inputs.push(parse(&buffer)?);
                buffer.clear();
            } else {
                buffer.push(*i);
            }
        }
        inputs.push(parse(buffer)?);
        return Ok(AST::Function { name: expr.split("(").nth(0).unwrap().to_string(), inputs: Box::new(inputs) })
    }
    
    // is it a variable?

    if expr_chars[0].is_alphabetic() || expr_chars[0] == '\\' {
        return Ok(AST::from_variable(expr));
    }

    let v = parse_value(expr_chars.iter().collect())?;

    return Ok(v);
}

///used to evaluate a given binary tree in the context of the provided variables.
///
///pi and e need to be provided as variables if used.
///
///If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](fn@crate::quick_eval).
pub fn eval(b: &AST, state: &Store) -> Result<Value, EvalError> {
    eval_rec(b, state, "")
}

fn eval_rec(b: &AST, state: &Store, last_fn: &str) -> Result<Value, EvalError> {
    match b {
        AST::Scalar(s) => return Ok(Value::Scalar(*s)),
        AST::Vector(v) => {
            let mut evaled_scalars = vec![];
            for i in &**v {
                evaled_scalars.push(match eval_rec(i, state, last_fn)?.get_scalar() {
                    Some(s) => s,
                    None => return Err(EvalError::NonScalarInVector)
                });
            }
            return Ok(Value::Vector(evaled_scalars))
        },
        AST::Matrix(m) => {
            let mut evaled_rows = vec![];
            for i in &**m {
                let mut row = vec![];
                for j in i {
                    row.push(match eval_rec(j, state, last_fn)?.get_scalar() {
                        Some(s) => s,
                        None => return Err(EvalError::NonScalarInMatrix)
                    });
                }
                evaled_rows.push(row);
            }
            return Ok(Value::Matrix(evaled_rows))
        }, 
        AST::Variable(v) => {
            for i in state.vars.iter() {
                if &i.name == v {
                    return Ok(i.value.clone());
                }
            }

            return Err(EvalError::NoVariable(v.to_string()));
        },
        AST::Function { name, inputs } => {
            if last_fn == name {
                return Err(EvalError::RecursiveFunction);
            }
            let mut function = None;
            for i in state.funs.iter() {
                if i.name == name.to_string() {
                    function = Some(i);
                    break;
                } 
            }
            if function.is_none() {
                return Err(EvalError::NoFunction(name.to_string()));
            }

            let function = function.unwrap();
            
            if inputs.len() != function.inputs.len() {
                return Err(EvalError::WrongNumberOfArgs((function.inputs.len(), inputs.len())));
            }

            let mut eval_inputs = vec![];
            for i in inputs.iter() {
                eval_inputs.push(eval_rec(i, state, last_fn)?);
            }

            let mut f_vars = vec![];
            for i in 0..inputs.len() {
                f_vars.push(Variable::new(&function.inputs[i], eval_inputs[i].clone()));
            }

            for i in state.vars.iter() {
                if !f_vars.iter().map(|v| v.name.to_string()).collect::<Vec<String>>().contains(&i.name) {
                    f_vars.push(i.clone());
                }
            }

            return eval_rec(&function.binary, &Store::new(&f_vars, &state.funs), name);
        },
        AST::Operation(o) => {
            match &**o {
                Operation::SimpleOperation {op_type, left, right} => {
                    let lv = eval_rec(&left, state, last_fn)?;
                    let rv = eval_rec(&right, state, last_fn)?; 
                    match op_type {
                        SimpleOpType::Get => return Ok(maths::get(lv, rv)?),
                        SimpleOpType::Add => return Ok(maths::add(lv, rv)?),
                        SimpleOpType::Sub => return Ok(maths::sub(lv, rv)?),
                        SimpleOpType::Mult => return Ok(maths::mult(lv, rv)?),
                        SimpleOpType::Neg => return Ok(maths::neg(lv)?),
                        SimpleOpType::Div => return Ok(maths::div(lv, rv)?),
                        SimpleOpType::Cross => return Ok(maths::cross(lv, rv)?),
                        SimpleOpType::HiddenMult => return Ok(maths::mult(lv, rv)?),
                        SimpleOpType::Pow => return Ok(maths::pow(lv, rv)?),
                        SimpleOpType::Sin => return Ok(maths::sin(lv)?),
                        SimpleOpType::Cos => return Ok(maths::cos(lv)?),
                        SimpleOpType::Tan => return Ok(maths::tan(lv)?),
                        SimpleOpType::Abs => return Ok(maths::abs(lv)?),
                        SimpleOpType::Sqrt => return Ok(maths::sqrt(lv)?),
                        SimpleOpType::Ln => return Ok(maths::ln(lv)?),
                        SimpleOpType::Arcsin => return Ok(maths::arcsin(lv)?),
                        SimpleOpType::Arccos => return Ok(maths::arccos(lv)?),
                        SimpleOpType::Arctan => return Ok(maths::arctan(lv)?),
                        SimpleOpType::Parenths => return Ok(lv),
                    }
                },
                Operation::AdvancedOperation(a) => {
                    match a {
                        AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                            let lb = eval_rec(&lower_bound, state, last_fn)?;
                            let ub = eval_rec(&upper_bound, state, last_fn)?;

                            return Ok(maths::calculus::calculate_integral(&expr, in_terms_of.clone(), lb, ub, state)?);
                        },
                        AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                            let eat = eval_rec(&at, state, last_fn)?;
                            
                            return Ok(maths::calculus::calculate_derivative(&expr, &in_terms_of, &eat, None, state)?);
                        }
                    }
                }
            } 
        }
    }
}
