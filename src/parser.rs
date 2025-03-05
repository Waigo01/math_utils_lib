use crate::{basetypes::{AdvancedOpType, AdvancedOperation, Operation, SimpleOpType, Value, Variable, AST}, errors::{EvalError, ParserError}, helpers::{cart_prod, get_args}, maths, roots::RootFinder, Context, Values};

fn get_op_symbol(c: char) -> Option<SimpleOpType> {
    match c {
        '?' => Some(SimpleOpType::Get),
        '+' => Some(SimpleOpType::Add),
        '-' => Some(SimpleOpType::Sub),
        '&' => Some(SimpleOpType::AddSub),
        '*' => Some(SimpleOpType::Mult),
        '/' => Some(SimpleOpType::Div),
        '^' => Some(SimpleOpType::Pow),
        '#' => Some(SimpleOpType::Cross),
        _ => None
    }
}

/// checks if the given variable name is a valid variable name.
pub fn is_valid_var_name(var: String) -> bool {
    let var_chars: Vec<char> = var.chars().collect();
    if !var_chars[0].is_alphabetic() && var_chars[0] != '\\' {
        return false;
    }
    let mut parenths_open = 0;
    let mut previous_char = '\\';
    for i in var_chars {
        if i == '{' {
            parenths_open += 1;
        }
        if i == '}' {
            parenths_open -= 1;
        }
        if (i == '?'
            || i == '+'
            || i == '-'
            || i == '&'
            || i == '*'
            || i == '/'
            || i == '^'
            || i == '#' 
            || i == '=')
            && parenths_open == 0{
            return false
        }
        if i.is_numeric() && parenths_open == 0 && previous_char != '_' {
            return false;
        }
        previous_char = i;
    }
    return true;
}

fn parse_value(s: String) -> Result<AST, ParserError> {
    if !s.contains(&"[") {
        let p = match s.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(ParserError::ParseValue(s))
        };
        return Ok(AST::Scalar(p));
    } else if s.len() >= 2 {
        if s.chars().nth(0).unwrap() == '[' && s.chars().nth(s.len()-1).unwrap() == ']' {
            let args = get_args(&s.chars().collect::<Vec<char>>()[1..s.len()-1]);
            if args.is_empty() || args[0].is_empty() {
                return Err(ParserError::EmptyVec);
            }
            let output_v = args.iter().map(|v| parse_inner(v)).collect::<Result<Vec<AST>, ParserError>>()?;
            let mut is_vec = true;
            let mut is_mat = true;
            for i in &output_v {
                match i {
                    AST::Vector(_) => is_vec = false,
                    AST::Matrix(_) => is_mat = false,
                    _ => {}
                }
            }
            if is_vec && is_mat {
                return Ok(AST::Vector(Box::new(output_v)));
            } else if is_mat && !is_vec {
                let output_m = output_v.iter().map(|v| {
                    match v {
                        AST::Vector(v) => return Ok(v.to_vec()),
                        _ => return Err(ParserError::NotRectMatrix)
                    }
                }).collect::<Result<Vec<Vec<AST>>, ParserError>>()?;
                let size = output_m[0].len();
                for i in &output_m {
                    if i.len() != size {
                        return Err(ParserError::NotRectMatrix);
                    }
                }
                #[cfg(not(feature = "row-major"))]
                let mut col_matrix = vec![];
                #[cfg(not(feature = "row-major"))]
                for i in 0..output_m[0].len() {
                    let mut row = vec![];
                    for j in 0..output_m.len() {
                        row.push(output_m[j][i].clone());
                    }
                    col_matrix.push(row);
                }
                #[cfg(not(feature = "row-major"))]
                return Ok(AST::Matrix(Box::new(col_matrix)));
                #[cfg(feature = "row-major")]
                return Ok(AST::Matrix(Box::new(output_m)));
            } else {
                return Err(ParserError::ParseValue(s))
            }
        } else {
            return Err(ParserError::MissingBracket)
        }
    } else {
        return Err(ParserError::ParseValue(s));
    }
}

/// used to construct an AST from a string.
pub fn parse<S: Into<String>>(expr: S) -> Result<AST, ParserError> {
    let whitespaced_string: String = expr.into().trim().split(" ").filter(|s| !s.is_empty()).collect();
    parse_inner(&whitespaced_string)
}

fn parse_inner(expr: &str) -> Result<AST, ParserError> {
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
                left: parse_inner(&expr_chars.iter().collect::<String>())?,
                right: AST::from_value(Value::Scalar(0.)) 
            }));
        }
    }

    //is it an operation?
    
    let op_types = vec![SimpleOpType::Add, SimpleOpType::Sub, SimpleOpType::AddSub, SimpleOpType::Mult, SimpleOpType::Neg, SimpleOpType::Div, SimpleOpType::Cross, SimpleOpType::HiddenMult, SimpleOpType::Pow, SimpleOpType::Get];
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
        if parenths_open == 0 && brackets_open == 0 && curly_brackets_open == 0 && i != expr_chars.len()-1 && symbol.is_some() {
            let operation = symbol.unwrap();
            if i == 0 && operation == SimpleOpType::Sub {
                ops_in_expr.push((SimpleOpType::Neg, i, 0, 1));
            } else {
                ops_in_expr.push((operation, i, 0, 1));
            }
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
                let left_s: String = expr_chars[0..(i.1-i.2)].to_vec().iter().collect();
                let right_s: String = expr_chars[(i.1+i.3)..].to_vec().iter().collect();
                let right_b = parse_inner(&right_s)?; 
                if left_s.is_empty() {
                    return Ok(AST::from_operation(Operation::SimpleOperation {
                        op_type: i.0.clone(), 
                        left: AST::Scalar(0.), 
                        right: right_b
                    }));
                }
                let left_b = parse_inner(&expr_chars[0..(i.1-i.2)].to_vec().iter().collect::<String>())?;
                return Ok(AST::from_operation(Operation::SimpleOperation {
                    op_type: i.0.clone(),
                    left: left_b,
                    right: right_b
                }));
            }
        }
    }

    // is it a function?

    let function_look_up = vec![(SimpleOpType::Sin, "sin("), (SimpleOpType::Cos, "cos("), (SimpleOpType::Tan, "tan("), (SimpleOpType::Abs, "abs("), (SimpleOpType::Sqrt, "sqrt("), (SimpleOpType::Root, "root("), (SimpleOpType::Ln, "ln("), (SimpleOpType::Arcsin, "arcsin("), (SimpleOpType::Arccos, "arccos("), (SimpleOpType::Arctan, "arctan("), (SimpleOpType::Det, "det("), (SimpleOpType::Inv, "inv(")];
    
    for i in function_look_up {
        if expr_chars.iter().collect::<String>().starts_with(i.1) {
            if i.0 == SimpleOpType::Root {
                let args = get_args(&expr_chars[i.1.len()..expr_chars.len()-1]);

                if args.len() != 2 {
                    return Err(ParserError::WrongNumberOfArgs("root".to_string()));
                } else {
                    let left_b = parse_inner(&args[0].clone())?;
                    let right_b = parse_inner(&args[1].clone())?;

                    return Ok(AST::from_operation(Operation::SimpleOperation { 
                        op_type: i.0,
                        left: left_b,
                        right: right_b
                    }));
                }
            } else {
                let left_b = parse_inner(&expr_chars[i.1.len()..expr_chars.len()-1].to_vec().iter().collect::<String>())?;
                return Ok(AST::from_operation(Operation::SimpleOperation {
                    op_type: i.0,
                    left: left_b,
                    right: AST::from_value(Value::Scalar(0.))
                }));
            }
        }
    }

    // is it an advanced operation?

    let advanced_op_look_up = vec![(AdvancedOpType::Integral, "I("), (AdvancedOpType::Derivative, "D("), (AdvancedOpType::Equation, "eq(")];

    for i in advanced_op_look_up {
        if expr_chars.iter().collect::<String>().starts_with(i.1) {
            match i.0 {
                AdvancedOpType::Derivative => {
                    let args = get_args(&expr_chars[i.1.len()..expr_chars.len()-1]);
                    
                    if args.len() != 3 {
                        return Err(ParserError::WrongNumberOfArgs("derivative".to_string()));
                    }
                    let parsed_function = parse_inner(&args[0])?;
                    let parsed_value_at = parse_inner(&args[2])?;
                    return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Derivative {
                        expr: parsed_function,
                        in_terms_of: args[1].clone(),
                        at: parsed_value_at
                    })));
                },
                AdvancedOpType::Integral => {
                    let args = get_args(&expr_chars[i.1.len()..expr_chars.len()-1]);
                    
                    if args.len() != 4 {
                        return Err(ParserError::WrongNumberOfArgs("integral".to_string()));
                    }
                    let parsed_function = parse_inner(&args[0])?;
                    let parsed_lower_b = parse_inner(&args[2])?;
                    let parsed_upper_b = parse_inner(&args[3])?;
                    return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Integral {
                        expr: parsed_function,
                        in_terms_of: args[1].clone(),
                        lower_bound: parsed_lower_b,
                        upper_bound: parsed_upper_b
                    })));
                },
                AdvancedOpType::Equation => {
                    let entries = get_args(&expr_chars[i.1.len()..expr_chars.len()-1]);

                    let mut parsed_equations = vec![];
                    let mut search_vars = vec![];

                    for i in entries {
                        if !i.contains("=") {
                            search_vars.push(i.clone());
                            continue;
                        }

                        let left = i.split("=").nth(0).unwrap().to_string();
                        let right = i.split("=").nth(1).unwrap().to_string();

                        let left_b;
                        let right_b;
                        if left.len() >= right.len() {
                            left_b = parse_inner(&left)?;
                            right_b = parse_inner(&right)?;
                        } else {
                            left_b = parse_inner(&right)?;
                            right_b = parse_inner(&left)?;
                        }

                        parsed_equations.push((left_b, right_b));
                    }

                    return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Equation { equations: parsed_equations, search_vars })));
                }
            }
        }
    }
    
    // is it a custom function?

    if expr.contains("(") && expr.find("(").unwrap() != 0 && *expr_chars.last().unwrap() == ')' {
        let first_parenth = expr.find("(").unwrap();
        let args = get_args(&expr_chars[first_parenth+1..expr_chars.len()-1]);

        let parsed_args: Vec<AST> = args.iter().map(|a| parse_inner(a)).collect::<Result<Vec<AST>, ParserError>>()?;

        let func_name = expr.split("(").nth(0).unwrap().to_string(); 

        if is_valid_var_name(func_name.clone()) == false {
            return Err(ParserError::InvalidFunctionName(func_name));
        }

        return Ok(AST::Function { name: func_name, inputs: Box::new(parsed_args) })
    }
    
    // is it a variable?

    if expr_chars[0].is_alphabetic() || expr_chars[0] == '\\' {
        if is_valid_var_name(expr.to_string()) == false {
            return Err(ParserError::InvalidVariableName(expr.to_string()));
        }

        return Ok(AST::from_variable_name(expr));
    }

    // is it a list of values?
    
    if expr_chars[0] == '{' && expr_chars[expr_chars.len()-1] == '}' {
        return Ok(AST::List(get_args(&expr_chars[1..expr_chars.len()-1]).iter().map(|s| parse_inner(s)).collect::<Result<Vec<AST>, ParserError>>()?));
    }

    let v = parse_value(expr_chars.iter().collect())?;

    return Ok(v);
}

/// used to evaluate an AST with the provided context.
///
/// If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](fn@crate::quick_eval).
pub fn eval(b: &AST, context: &Context) -> Result<Values, EvalError> {
   Ok(Values::from_vec(eval_rec(b, context, "")?))
}

fn eval_rec(b: &AST, context: &Context, last_fn: &str) -> Result<Vec<Value>, EvalError> {
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
            let mut function = None;
            for i in context.funs.iter() {
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
                res.push(eval_rec(&function.ast, &Context::new(&f_vars, &context.funs), name)?);
            }

            return Ok(res.into_iter().flatten().collect());
        },
        AST::Operation(o) => {
            match &**o {
                Operation::SimpleOperation {op_type, left, right} => {
                    let lv = eval_rec(&left, context, last_fn)?;
                    let rv = eval_rec(&right, context, last_fn)?;

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
