use crate::{PREC, basetypes::{AST, AdvancedOperation, Operation, SimpleOpType}, output::AssignmentType, tokenizer::{Token, TokenStream}};

#[doc(hidden)]
pub fn center_in_string(f: String, n: i32) -> String {
    let f_string = f.to_string();
    let f_string_len = f_string.len();

    if f_string_len as i32 > n {
        return f_string;
    }

    let padding = n-f_string_len as i32;
    let l_padding;
    let r_padding;
    if padding % 2 == 0 {
        l_padding = padding/2;
        r_padding = l_padding;
    } else {
        l_padding = (padding as f64/2.).floor() as i32;
        r_padding = (padding as f64/2.).ceil() as i32;
    }
    let mut buffer_string = String::new();
    for _ in 0..l_padding {
        buffer_string += " ";
    }
    buffer_string += &f_string;
    for _ in 0..r_padding {
        buffer_string += " ";
    }

    return buffer_string;
}

#[doc(hidden)]
pub fn round_and_format(x: f64, latex: bool) -> String {
    if (x*10f64.powi(PREC as i32-2)).round()/10f64.powi(PREC as i32-2) == 0. && !latex && x != 0. {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        return scientific;
    } else if (x*10f64.powi(PREC as i32-2)).round()/10f64.powi(PREC as i32-2) == 0. && x != 0. {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        let left = scientific.split("e").nth(0).unwrap();
        let right = scientific.split("e").nth(1).unwrap();
        return format!("{}\\cdot 10^{{{}}}", left, right);
    } else {
        let rounded = (x*10f64.powi(PREC as i32-2)).round()/10f64.powi(PREC as i32-2);
        let rounded_string;
        if rounded == 0. && rounded.to_string().len() > 1 {
            rounded_string = rounded.to_string()[1..].to_string();
        } else {
            rounded_string = rounded.to_string();
        }
        return rounded_string;
    }
}

#[doc(hidden)]
pub fn cart_prod<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut results: Vec<Vec<T>> = vec![vec![]];
    for i in 0..arr.len() {
        let mut temp_res = vec![];
        for res in &results {
            for el in &arr[i] {
                let mut new_res = res.to_vec();
                new_res.push(el.clone());
                temp_res.push(new_res);
            }
        }
        results = temp_res;
    }

    return results;
}

#[doc(hidden)]
pub fn get_args(stream: &[Box<Token>]) -> Vec<TokenStream> {
    let mut args = vec![];
    let mut arg = vec![];
    for token in stream {
        if let Token::Punct(ref s) = **token && s == "," {
            args.push(TokenStream::from_tokens(arg.clone()));
            arg.clear();
            continue;
        }
        arg.push(token.to_owned());
    }
    args.push(TokenStream::from_tokens(arg));
    args
}

#[doc(hidden)]
pub fn flatten_conditional(condition: &AST, then: &AST, mut else_outer: &Option<AST>) -> (Vec<(AST, AST)>, Option<AST>) {
    let mut conditionals = vec![(condition.clone(), then.clone())];

    while let Some(AST::Operation(op)) = r#else_outer
    && let Operation::AdvancedOperation(ref aop) = **op
    && let AdvancedOperation::Conditional { condition, then, r#else } = aop {
        conditionals.push((condition.clone(), then.clone()));
        else_outer = r#else;
    }

    return (conditionals, else_outer.clone())
}

#[doc(hidden)]
pub fn find_assignments_in_ast(ast: &AST) -> Vec<(String, AssignmentType)> {
    match ast {
        AST::List(asts) => asts.into_iter().map(|ast| find_assignments_in_ast(ast)).flatten().collect(),
        AST::Operation(op) if let Operation::SimpleOperation { op_type, left, right } = &**op && *op_type == SimpleOpType::Assign => {
            if let AST::Variable(var_name) = left {
                vec![vec![(var_name.to_string(), AssignmentType::Var)], find_assignments_in_ast(right)].concat()
            } else if let AST::List(asts) = left {
                let mut vars = vec![];
                for ast in asts {
                    if let AST::Variable(var_name) = ast {
                        vars.push((var_name.to_string(), AssignmentType::Var));
                    }
                }
                vec![vars, find_assignments_in_ast(right)].concat()
            } else if let AST::Function{name, ..} = left {
                vec![vec![(name.to_string(), AssignmentType::Fun)], find_assignments_in_ast(right)].concat()
            } else {
                find_assignments_in_ast(right)
            }
        },
        AST::Operation(op) if let Operation::SimpleOperation { left, right, .. } = &**op => {
            let mut assignments = vec![];
            assignments.append(&mut find_assignments_in_ast(left));
            assignments.append(&mut find_assignments_in_ast(right));

            assignments
        },
        AST::Operation(op) if let Operation::AdvancedOperation(a_op) = &**op => {
            match a_op {
                AdvancedOperation::Integral { lower_bound, upper_bound, .. } => vec![find_assignments_in_ast(lower_bound), find_assignments_in_ast(upper_bound)].concat(),
                AdvancedOperation::Equation { .. } => vec![],
                AdvancedOperation::Derivative { at, .. } => vec![find_assignments_in_ast(at)].concat(),
                AdvancedOperation::Conditional { condition, then, r#else } => {
                    let mut assignments = vec![find_assignments_in_ast(condition), find_assignments_in_ast(then)].concat();

                    if let Some(else_ast) = r#else {
                        assignments.append(&mut find_assignments_in_ast(else_ast));
                    }

                    assignments
                }
            }
        },
        AST::Operation(_) => vec![],
        AST::Vector(asts) => asts.into_iter().map(|ast| find_assignments_in_ast(ast)).flatten().collect(),
        AST::Matrix(asts2) => asts2.into_iter().map(|asts| asts.into_iter().map(|ast| find_assignments_in_ast(ast)).flatten()).flatten().collect(),
        AST::Variable(_) => vec![],
        AST::Function { inputs, .. } => inputs.into_iter().map(|ast| find_assignments_in_ast(ast)).flatten().collect(),
        AST::Scalar(_) => vec![]
    }
}
