#[cfg(feature = "async")]
use async_macro::function_async;

use async_macro::async_call;

use crate::{Complex, RealNumber, StandardFunctions, basetypes::{AST, AdvancedOperation, Operation}, maths::num_traits::Number, tokenizer::{Token, TokenStream}};

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
pub fn round_and_format<N>(x: N, latex: bool) -> String where N: Number {
    if x.is_infinite() && x < N::zero() && latex {
        return r"-\infty".to_string();
    } else if x.is_infinite() && latex {
        return r"\infty".to_string();
    } else if (x/N::display_epsilon()).round()*N::display_epsilon() == N::zero() && !latex && x != N::zero() {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        return scientific;
    } else if (x/N::display_epsilon()).round()*N::display_epsilon() == N::zero() && x != N::zero() {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        let left = scientific.split("e").nth(0).unwrap();
        let right = scientific.split("e").nth(1).unwrap();
        return format!("{}\\cdot 10^{{{}}}", left, right);
    } else {
        let rounded = (x/N::display_epsilon()).round()*N::display_epsilon();
        let rounded_string;
        if rounded == N::zero() && rounded.to_string().len() > 1 {
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
        if let Token::Punct(ref s) = **token && *s == ',' {
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
pub fn flatten_conditional<N: Number>(condition: &AST<N>, then: &AST<N>, mut else_outer: &Option<AST<N>>) -> (Vec<(AST<N>, AST<N>)>, Option<AST<N>>) {
    let mut conditionals = vec![(condition.clone(), then.clone())];

    while let Some(AST::Operation(op)) = r#else_outer
    && let Operation::AdvancedOperation(ref aop) = **op
    && let AdvancedOperation::Conditional { condition, then, r#else } = aop {
        conditionals.push((condition.clone(), then.clone()));
        else_outer = r#else;
    }

    return (conditionals, else_outer.clone())
}

const G: i32 = 7;
const P: [f64; 9] = [
    0.99999999999980993,
    676.5203681218851,
    -1259.1392167224028,
    771.32342877765313,
    -176.61502916214059,
    12.507343278686905,
    -0.13857109526572012,
    9.9843695780195716e-6,
    1.5056327351493116e-7
];


#[doc(hidden)]
pub fn lanczos_approx<N: Number + RealNumber + StandardFunctions>(mut z: Complex<N>) -> Complex<N> {
    let y;
    let pi = Complex::from(std::f64::consts::PI);
    if z.re() < N::from(0.5) {
        y = pi / ((pi * z).sin() * lanczos_approx(Complex::one()-z));
    } else {
        z = z - Complex::one();
        let mut x = Complex::from(P[0]);
        for i in 1..P.len() {
            x = x + Complex::from(P[i]) / (z + Complex::from(i as i32));
        }
        let t = z + Complex::from(G) + Complex::from(0.5);
        y = (Complex::from(2)*pi).sqrt() * t.powf(z+Complex::from(0.5)) * (-t).exp() * x;
    }

    return y;
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn replace_in_ast<N: Number>(ast: AST<N>, search: &AST<N>, replace: &AST<N>) -> AST<N> {
    if ast == *search {
        return replace.clone();
    } else {
        match ast {
            AST::Operation(op) => {
                match *op {
                    Operation::SimpleOperation { op_type, left, right } => {
                        AST::Operation(Box::new(Operation::SimpleOperation { op_type, left: async_call!(replace_in_ast(left, search, replace)), right: async_call!(replace_in_ast(right, search, replace)) }))
                    },
                    Operation::AdvancedOperation(aop) => {
                        AST::Operation(Box::new(Operation::AdvancedOperation(
                            match aop {
                                AdvancedOperation::Sum { expr, lower_bound, upper_bound, in_terms_of } => {
                                    AdvancedOperation::Sum { expr: async_call!(replace_in_ast(expr, search, replace)), in_terms_of: in_terms_of, lower_bound: async_call!(replace_in_ast(lower_bound, search, replace)), upper_bound: async_call!(replace_in_ast(upper_bound, search, replace)) }
                                },
                                AdvancedOperation::Integral { expr, in_terms_of, lower_bound, upper_bound } => {
                                    AdvancedOperation::Integral { expr: async_call!(replace_in_ast(expr, search, replace)), in_terms_of: in_terms_of, lower_bound: async_call!(replace_in_ast(lower_bound, search, replace)), upper_bound: async_call!(replace_in_ast(upper_bound, search, replace)) }
                                },
                                AdvancedOperation::Equation { mut equations, search_vars } => {
                                    for equation in equations.iter_mut() {
                                        *equation = (async_call!(replace_in_ast(equation.0.clone(), search, replace)), async_call!(replace_in_ast(equation.1.clone(), search, replace)));
                                    }
                                    AdvancedOperation::Equation { equations, search_vars }
                                },
                                AdvancedOperation::Derivative { expr, in_terms_of, at } => {
                                    AdvancedOperation::Derivative { expr: async_call!(replace_in_ast(expr, search, replace)), in_terms_of, at: async_call!(replace_in_ast(at, search, replace)) }
                                },
                                AdvancedOperation::Conditional { condition, then, r#else } => {
                                    AdvancedOperation::Conditional { condition: async_call!(replace_in_ast(condition, search, replace)), then: async_call!(replace_in_ast(then, search, replace)), r#else: if let Some(else_ast) = r#else {Some(async_call!(replace_in_ast(else_ast, search, replace)))} else {None} }
                                }
                            }
                        )))
                    }
                }
            },
            AST::List(mut asts) => {
                for ast in asts.iter_mut() {
                    *ast = async_call!(replace_in_ast(ast.clone(), search, replace));
                }
                AST::List(asts)
            },
            AST::Function { name, mut inputs } => {
                for input in inputs.iter_mut() {
                    *input = async_call!(replace_in_ast(input.clone(), search, replace));
                }
                AST::Function { name, inputs }
            },
            AST::Vector(mut elements) => {
                for element in elements.iter_mut() {
                    *element = async_call!(replace_in_ast(element.clone(), search, replace));
                }
                AST::Vector(elements)
            },
            AST::Matrix(mut rows) => {
                for row in rows.iter_mut() {
                    for element in row.iter_mut() {
                        *element = async_call!(replace_in_ast(element.clone(), search, replace));
                    }
                }
                AST::Matrix(rows)
            },
            AST::Scalar(s) => {
                AST::Scalar(s.clone())
            },
            AST::Variable(v) => {
                AST::Variable(v.clone())
            }
        }
    }
}
