use crate::{basetypes::{AST, AdvancedOperation, Operation, SimpleOpType, Value}, errors::ParserError, helpers::get_args, tokenizer::{Delimiter, Token, Tokenizer}, value};

fn get_op_symbol(punct: &str) -> Option<SimpleOpType> {
    match punct {
        "=" => Some(SimpleOpType::Assign),
        "&" => Some(SimpleOpType::BoolAnd),
        "|" => Some(SimpleOpType::BoolOr),
        "==" => Some(SimpleOpType::BoolEq),
        "!=" => Some(SimpleOpType::BoolNEq),
        "<" => Some(SimpleOpType::BoolLt),
        ">" => Some(SimpleOpType::BoolGt),
        "<=" => Some(SimpleOpType::BoolLtEq),
        ">=" => Some(SimpleOpType::BoolGtEq),
        "!" => Some(SimpleOpType::BoolNot),
        "+" => Some(SimpleOpType::Add),
        "-" => Some(SimpleOpType::Sub),
        "+-" => Some(SimpleOpType::AddSub),
        "*" => Some(SimpleOpType::Mult),
        "/" => Some(SimpleOpType::Div),
        "#" => Some(SimpleOpType::Cross),
        "^" => Some(SimpleOpType::Pow),
        "@" => Some(SimpleOpType::Get),
        _ => None
    }
}

fn parse_value(s: &Token) -> Result<AST, ParserError> {
    if let Token::Literal(literal) = s {
        let p = match literal.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(ParserError::ParseValue(literal.to_string()))
        };
        return Ok(AST::Scalar(p));
    } else if let Token::Group(group) = s {
        let args = get_args(&group.stream);

        if args.is_empty() || args[0].is_empty() {
            return Err(ParserError::EmptyVec);
        }

        let output_v = args.iter().map(|v| parse_inner(v)).collect::<Result<Vec<AST>, ParserError>>()?;
        
        let mut is_vec = true;
        let mut is_mat = true;

        for ast in &output_v {
            match ast {
                AST::Vector(_) => is_vec = false,
                AST::Matrix(_) => is_mat = false,
                _ => {}
            }
        }

        if is_vec && is_mat {
            return Ok(AST::Vector(output_v))
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
            return Ok(AST::Matrix(col_matrix));
            #[cfg(feature = "row-major")]
            return Ok(AST::Matrix(output_m));
        } else {
            return Err(ParserError::ParseValue(s.to_string()));
        }
    } else {
        return Err(ParserError::ParseValue(s.to_string()));
    }
}

/// used to construct an AST from a string.
pub fn parse<S: Into<String> + Clone>(expr: S) -> Result<AST, ParserError> {
    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(expr)?;

    parse_inner(&tokens)
}

fn parse_inner(tokens: &[Box<Token>]) -> Result<AST, ParserError> {
    if tokens.len() == 0 {
        return Err(ParserError::EmptyExpr);
    }

    if tokens.len() == 1 && let Some(boxed) = tokens.get(0) && let Token::Group(ref group) = **boxed {
        if group.delimiter == Delimiter::Parenthesis {
            return Ok(AST::from_operation(Operation::SimpleOperation{
                op_type: SimpleOpType::Parenths,
                left: parse_inner(&group.stream)?,
                right: AST::from_value(value!(0))
            }))
        } else if group.delimiter == Delimiter::Brace {
            let args = get_args(&group.stream);
            return Ok(AST::List(args.into_iter().map(|s| parse_inner(&s)).collect::<Result<Vec<AST>, ParserError>>()?));
        }
    }

    //is it an operation? 
    let mut ops_in_expr: Vec<(SimpleOpType, usize, usize)> = vec![];
    let mut highest_op = usize::MAX;

    for i in 0..tokens.len() {
        if i != 0 && ((tokens[i-1].is_literal() && (tokens[i].is_group() || tokens[i].is_ident())) || (tokens[i-1].is_group() && (tokens[i].is_group()))) {
            ops_in_expr.push((SimpleOpType::HiddenMult, i, 0));
            if (SimpleOpType::HiddenMult as usize) < highest_op {highest_op = SimpleOpType::HiddenMult as usize}
        } else if let Token::Punct(ref punct) = *tokens[i] {
            let symbol = get_op_symbol(&punct);
            let Some(operation) = symbol else {return Err(ParserError::UnrecognizedPunct)};
            if i == 0 && operation == SimpleOpType::Sub {
                ops_in_expr.push((SimpleOpType::Neg, i, 1));
                if (SimpleOpType::Neg as usize) < highest_op {highest_op = SimpleOpType::Neg as usize}
            } else {
                ops_in_expr.push((operation.clone(), i, 1));
                if (operation.clone() as usize) < highest_op {highest_op = operation as usize}
            }
            
        }
    }

    if highest_op == SimpleOpType::Sub as usize || highest_op == SimpleOpType::Div as usize {
        ops_in_expr.reverse();
    }

    for op in &ops_in_expr {
        if op.0.clone() as usize == highest_op {
            let left_ts = tokens[0..op.1].to_vec();
            let right_ts = tokens[(op.1+op.2)..].to_vec();
            let right_b = parse_inner(&right_ts)?;

            if left_ts.is_empty() && (op.0 == SimpleOpType::AddSub || op.0 == SimpleOpType::Neg || op.0 == SimpleOpType::BoolNot) {
                return Ok(AST::from_operation(Operation::SimpleOperation {
                    op_type: op.0.clone(), 
                    left: AST::Scalar(0.), 
                    right: right_b
                }));
            } else if left_ts.is_empty() {
                return Err(ParserError::OperationNeedsLeftValue);
            }

            let left_b = parse_inner(&left_ts)?;

            if op.0 == SimpleOpType::Assign {
                let mut is_ok = false;
                if let AST::Function { ref inputs, .. } = left_b {
                    is_ok = true;
                    for input in inputs.iter() {
                        if let AST::Variable(_) = input {} else {
                            is_ok = false;
                            break;
                        }
                    }
                } else if let AST::Variable(_) = left_b {is_ok = true}
                else if let AST::List(ref list) = left_b {
                    is_ok = true;
                    for ast in list.iter() {
                        if let AST::Variable(_) = ast {} else {
                            is_ok = false;
                            break;
                        }
                    }
                }

                if !is_ok {
                    return Err(ParserError::LeftSideOfAssignmentIncorrect);
                }
            }

            return Ok(AST::from_operation(Operation::SimpleOperation {
                op_type: op.0.clone(),
                left: left_b,
                right: right_b
            }));
        }
    }

    // is it a function/advanced op?
    
    if tokens.len() == 2 && let Token::Ident(ref ident) = *tokens[0] && let Token::Group(ref group) = *tokens[1] && group.delimiter == Delimiter::Parenthesis {
        match ident.as_str() {
            "D" => {
                let args = get_args(&group.stream);
                
                if args.len() != 3 {
                    return Err(ParserError::WrongNumberOfArgs("derivative".to_string()));
                }
                let parsed_function = parse_inner(&args[0])?;
                let Ok(AST::Variable(in_terms_of)) = parse_inner(&args[1]) else {
                    return Err(ParserError::DerivNotITOVar);
                };
                let parsed_value_at = parse_inner(&args[2])?;
                return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Derivative {
                    expr: parsed_function,
                    in_terms_of,
                    at: parsed_value_at
                })));
            },
            "I" => {
                let args = get_args(&group.stream);
                
                if args.len() != 4 {
                    return Err(ParserError::WrongNumberOfArgs("integral".to_string()));
                }
                let parsed_function = parse_inner(&args[0])?;
                let Ok(AST::Variable(in_terms_of)) = parse_inner(&args[1]) else {
                    return Err(ParserError::DerivNotITOVar);
                };
                let parsed_lower_b = parse_inner(&args[2])?;
                let parsed_upper_b = parse_inner(&args[3])?;
                return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Integral {
                    expr: parsed_function,
                    in_terms_of,
                    lower_bound: parsed_lower_b,
                    upper_bound: parsed_upper_b
                })));
            },
            "eq" => {
                let entries = get_args(&group.stream);

                let mut parsed_equations = vec![];
                let mut search_vars = vec![];

                for i in entries {
                    if i.len() == 1 && let Token::Ident(ref ident) = *i[0] {
                        search_vars.push(ident.to_string());
                        continue;
                    }

                    let left = i.split(|s| if let Token::Punct(ref punct) = **s && punct == "=" {true} else {false}).nth(0).unwrap();
                    let right = i.split(|s| if let Token::Punct(ref punct) = **s && punct == "=" {true} else {false}).nth(1).unwrap();

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
            },
            "if" => {
                let args = get_args(&group.stream);

                if args.len() != 2 && args.len() != 3 {
                    return Err(ParserError::WrongNumberOfArgs("if".to_string()));
                }

                let parsed_condition = parse_inner(&args[0])?;
                let parsed_then = parse_inner(&args[1])?;
                let parsed_else = if args.len() == 3 {Some(parse_inner(&args[2])?)} else {None};

                return Ok(AST::from_operation(Operation::AdvancedOperation(AdvancedOperation::Conditional {
                    condition: parsed_condition,
                    then: parsed_then,
                    r#else: parsed_else
                })));
            }
            _ => {
                let args = get_args(&group.stream);

                let parsed_args: Vec<AST> = args.iter().map(|a| parse_inner(a)).collect::<Result<Vec<AST>, ParserError>>()?;

                return Ok(AST::Function { name: ident.to_string(), inputs: parsed_args })
            }
        }

    }
    
    // is it a variable?

    if tokens.len() == 1 && let Token::Ident(ref ident) = *tokens[0] {
        return Ok(AST::from_variable_name(ident));
    }
    

    if tokens.len() == 1 {
        return Ok(parse_value(&tokens[0])?);
    }
    

    return Err(ParserError::InvalidState);
}
