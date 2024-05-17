use crate::{basetypes::{Value, Variable}, errors::{EvalError, ParserError}, maths};

///specifies the type of operation for the [SimpleOperation](Operation::SimpleOperation) struct.
///
///This enum only contains simple mathematical operations with a left and right side or a maximum
///of two arguments. For more advanced operations, see [AdvancedOpType].
///
///The order of the enum also represents the reverse order of the operation priority.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SimpleOpType { 
    ///Add two scalars, vectors, or matrices (a+b)
    Add,
    ///Subtract two scalars, vectors, or matrices (a-b)
    Sub,
    ///Negate a scalar, vector or matrix or expression in parentheses (-(3*4))
    Neg,
    ///Multiply a scalar, vector or matrix with each other (Dotproduct, Matrix multiplication,
    ///Scalar multiplication, ...) (a*b)
    Mult,
    ///Divide two scalars or a vector or matrix with a scalar (a/b)
    Div,
    ///Calculate the cross product using "#" (V1#V2), only works with dim(V) <= 3. When dim(V) < 3
    ///the vector gets augmented with zeros
    Cross,
    ///Hidden multiplication between scalar and variable or parentheses (3a, 5(3+3), (3+5)(2+6))
    HiddenMult,
    ///Take a scalar to the power of another scalar using "^" (a^b)
    Pow,
    ///Index into vector using "?" ([3, 4, 5]?1 = 4)
    Get,
    ///Calculate the sin of a scalar (sin(a))
    Sin,
    ///Calculate the cos of a scalar (cos(a))
    Cos,
    ///Calculate the tan of a scalar (tan(a))
    Tan,
    ///Calculate the absolute value of a scalar or the length of a vector (abs(a))
    Abs,
    ///Calculate the square root of a scalar (sqrt(a))
    Sqrt,
    ///Calculate the natural log of a scalar (ln(a))
    Ln,
    ///Calculate the arcsin of a scalar (arcsin(a))
    Arcsin,
    ///Calculate the arccos of a scalar (arccos(a))
    Arccos,
    ///Calculate the arctan of a scalar (arctan(a))
    Arctan, 
    ///Prioritise expressions in parentheses (3*(5+5))
    Parenths
}

/// specifies the type of operation for the [AdvancedOperation] struct.
///
/// This enum only contains advanced operations with more than 2 arguments. For simple operations,
/// see [SimpleOpType].
#[derive(Clone, Debug)]
pub enum AdvancedOpType {
    ///Calculate the derivative of a function f in respect to n at a value m (D(f, n, m))
    Derivative,
    ///Calculate the integral of a function f in respect to n with the bounds a and b (I(f, n, a, b))
    Integral 
}

///used to construct a Binary Tree which is recursively evaluated by the [eval()] function.
///
///Binary can be a:
///
///- Value
///- Variable
///- Operation
#[derive(Debug, Clone)]
pub enum Binary {
    Value(Value),
    Variable(String),
    Operation(Box<Operation>),
}

impl Binary {
    pub fn from_value(val: Value) -> Binary {
        return Binary::Value(val);
    }
    pub fn from_variable(val: String) -> Binary {
        return Binary::Variable(val);
    }
    pub fn from_operation(val: Operation) -> Binary {
        return Binary::Operation(Box::new(val));
    } 
}

///used to specify an operation in a parsed string. It is used together with [Binary] to
///construct a Binary Tree from a mathematical expression.
#[derive(Debug, Clone)]
pub enum Operation {
    SimpleOperation {
        op_type: SimpleOpType,
        left: Binary,
        right: Binary,
    },
    AdvancedOperation(AdvancedOperation)
}

/// used to specify an advanced operation for more complex mathematical operatiors, such as
/// functions with more than two inputs.
#[derive(Debug, Clone)]
pub enum AdvancedOperation{
    Integral {
        expr: Binary,
        in_terms_of: String,
        lower_bound: Binary,
        upper_bound: Binary
    },
    Derivative {
        expr: Binary,
        in_terms_of: String,
        at: Binary
    }
}

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

fn parse_value(s: String) -> Result<Value, ParserError> {
    if !s.contains(&"[") {
        let val = match s.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(ParserError::ParseValue(s))
        };
        return Ok(Value::Scalar(val));
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
                        row.push(match n_buffer.parse::<f64>() {
                            Ok(t) => t,
                            Err(_) => return Err(ParserError::ParseValue(s))
                        });
                        n_buffer.clear();
                    } else {
                        n_buffer.push(i);
                    }
                } else if open_parenths == 0 {
                    if i == ',' {
                        if n_buffer.is_empty() {
                            return Err(ParserError::EmptyVec)
                        }
                        row.push(match n_buffer.parse::<f64>() {
                            Ok(t) => t,
                            Err(_) => return Err(ParserError::ParseValue(s))
                        });
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
            row.push(match n_buffer.parse::<f64>() {
                Ok(t) => t,
                Err(_) => return Err(ParserError::ParseValue(s))
            });
            if row_size.is_some() && row.len() != row_size.unwrap() {
                return Err(ParserError::NotRectMatrix)
            }
            output_m.push(row);
            let mut col_matrix = vec![];
            for i in 0..output_m[0].len() {
                let mut row = vec![];
                for j in 0..output_m.len() {
                    row.push(output_m[j][i]);
                }
                col_matrix.push(row);
            }
            return Ok(Value::Matrix(col_matrix));
        } else if s.chars().nth(0).unwrap() == '[' && s.chars().nth(s.len()-1).unwrap() == ']' {
            let mut output_v = vec![];
            let mut n_buffer = String::new();
            for i in s[1..s.len()].chars().collect::<Vec<char>>() {
                if i == ',' {
                    if n_buffer.is_empty() {
                        return Err(ParserError::EmptyVec)
                    }
                    output_v.push(match n_buffer.parse::<f64>() {
                        Ok(t) => t,
                        Err(_) => return Err(ParserError::ParseValue(s))
                    });
                    n_buffer.clear();
                } else {
                    n_buffer.push(i);
                }
            }
            if n_buffer.is_empty() {
                return Err(ParserError::EmptyVec);
            }
            output_v.push(match n_buffer[0..n_buffer.len()-1].parse::<f64>() {
                Ok(t) => t,
                Err(_) => return Err(ParserError::ParseValue(s))
            });
            return Ok(Value::Vector(output_v));
        } else {
            return Err(ParserError::MissingBracket)
        }
    } else {
        return Err(ParserError::EmptyVec);
    }
}

///used to construct a Binary Tree from a mathematical expression.
pub fn parse(expr: String) -> Result<Binary, ParserError> {
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
            return Ok(Binary::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Parenths,
                left: parse(expr_chars.iter().collect::<String>())?,
                right: Binary::from_value(Value::Scalar(0.)) 
            }));
        }
    }


    //is it an operation?
    
    let op_types = vec![SimpleOpType::Add, SimpleOpType::Sub, SimpleOpType::Mult, SimpleOpType::Div, SimpleOpType::Cross, SimpleOpType::HiddenMult, SimpleOpType::Pow, SimpleOpType::Get];
    let mut ops_in_expr: Vec<(SimpleOpType, usize, usize, usize)> = vec![];
    let mut highest_op: SimpleOpType = SimpleOpType::Add;
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
        if parenths_open == 0 && is_hidden_mult {
            highest_op = SimpleOpType::HiddenMult;
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
            highest_op = symbol.clone().unwrap();
            ops_in_expr.push((symbol.clone().unwrap(), i, 0, 1));
        } 
    }

    if highest_op == SimpleOpType::Sub || highest_op == SimpleOpType::Div {
        ops_in_expr.reverse();
    }

    for o in op_types {
        for i in &ops_in_expr {
            if i.0 == o {
                let left_b = parse(expr_chars[0..(i.1-i.2)].to_vec().iter().collect::<String>())?;
                let right_b = parse(expr_chars[(i.1+i.3)..].to_vec().iter().collect::<String>())?; 
                return Ok(Binary::from_operation(Operation::SimpleOperation {
                    op_type: i.0.clone(),
                    left: left_b,
                    right: right_b
                }));
            }
        }
    }

    // is it a negation?

    if expr_chars[0] == '-' {
        return Ok(Binary::from_operation(Operation::SimpleOperation {
            op_type: SimpleOpType::Neg,
            left: parse(expr_chars[1..].to_vec().iter().collect::<String>())?,
            right: Binary::from_value(Value::Scalar(0.))
        }));
    }

    // is it a function?

    let function_look_up = vec![(SimpleOpType::Sin, "sin("), (SimpleOpType::Cos, "cos("), (SimpleOpType::Tan, "tan("), (SimpleOpType::Abs, "abs("), (SimpleOpType::Sqrt, "sqrt("), (SimpleOpType::Ln, "ln("), (SimpleOpType::Arcsin, "arcsin("), (SimpleOpType::Arccos, "arccos("), (SimpleOpType::Arctan, "arctan(")];
    
    for i in function_look_up {
        if expr_chars.iter().collect::<String>().starts_with(i.1) {
            let left_b = parse(expr_chars[i.1.len()..expr_chars.len()-1].to_vec().iter().collect::<String>())?;
            return Ok(Binary::from_operation(Operation::SimpleOperation {
                op_type: i.0,
                left: left_b,
                right: Binary::from_value(Value::Scalar(0.))
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
                    let parsed_function = parse(args[0].clone())?;
                    let parsed_value_at = parse(args[2].clone())?;
                    return Ok(Binary::from_operation(Operation::AdvancedOperation(AdvancedOperation::Derivative {
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
                    let parsed_function = parse(args[0].clone())?;
                    let parsed_lower_b = parse(args[2].clone())?;
                    let parsed_upper_b = parse(args[3].clone())?;
                    return Ok(Binary::from_operation(Operation::AdvancedOperation(AdvancedOperation::Integral {
                        expr: parsed_function,
                        in_terms_of: args[1].clone(),
                        lower_bound: parsed_lower_b,
                        upper_bound: parsed_upper_b
                    })));
                } 
            }
        }
    }
    // is it a variable?

    if expr_chars[0].is_alphabetic() || expr_chars[0] == '\\' {
        return Ok(Binary::from_variable(expr));
    }

    let v = parse_value(expr_chars.iter().collect())?;

    return Ok(Binary::from_value(v));
}

///used to evaluate a given binary tree in the context of the provided variables.
///
///pi and e need to be provided as variables if used.
///
///If you are searching for a quick and easy way to evaluate an expression, have a look at [quick_eval()](fn@crate::quick_eval).
pub fn eval(b: &Binary, vars: &Vec<Variable>) -> Result<Value, EvalError> {
    match b {
        Binary::Value(c) => return Ok(c.clone()),
        Binary::Variable(v) => {
            for i in vars {
                if &i.name == v {
                    return Ok(i.value.clone());
                }
            }

            return Err(EvalError::NoVariable(v.to_string()));
        }
        Binary::Operation(o) => {
            match &**o {
                Operation::SimpleOperation {op_type, left, right} => {
                    let lv = eval(&left, vars)?;
                    let rv = eval(&right, vars)?; 
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
                            let lb = eval(&lower_bound, vars)?;
                            let ub = eval(&upper_bound, vars)?;

                            return Ok(maths::calculus::calculate_integral(&expr, in_terms_of.clone(), lb, ub, vars)?);
                        },
                        AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                            let eat = eval(&at, vars)?;
                            
                            return Ok(maths::calculus::calculate_derivative(&expr, in_terms_of.clone(), eat, None, vars)?);
                        }
                    }
                }
            } 
        }
    }
}
