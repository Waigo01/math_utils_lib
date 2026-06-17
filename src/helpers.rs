use crate::{Complex, PREC, RealNumber, StandardFunctions, basetypes::{AST, AdvancedOperation, Operation}, maths::num_traits::Number, tokenizer::{Token, TokenStream}};

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
    if (x*N::BASE.powi(PREC as i32-2)).round()/N::BASE.powi(PREC as i32-2) == N::ZERO && !latex && x != N::ZERO {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        return scientific;
    } else if (x*N::BASE.powi(PREC as i32-2)).round()/N::BASE.powi(PREC as i32-2) == N::ZERO && x != N::ZERO {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        let left = scientific.split("e").nth(0).unwrap();
        let right = scientific.split("e").nth(1).unwrap();
        return format!("{}\\cdot 10^{{{}}}", left, right);
    } else {
        let rounded = (x*N::BASE.powi(PREC as i32-2)).round()/N::BASE.powi(PREC as i32-2);
        let rounded_string;
        if rounded == N::ZERO && rounded.to_string().len() > 1 {
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
        y = pi / ((pi * z).sin() * lanczos_approx(Complex::ONE-z));
    } else {
        z = z - Complex::ONE;
        let mut x = Complex::from(P[0]);
        for i in 1..P.len() {
            x = x + Complex::from(P[i]) / (z + Complex::from(i as i32));
        }
        let t = z + Complex::from(G) + Complex::from(0.5);
        y = (Complex::from(2)*pi).sqrt() * t.powf(z+Complex::from(0.5)) * (-t).exp() * x;
    }

    return y;
}
