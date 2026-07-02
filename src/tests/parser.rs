use crate::{Context, MathLibError, Value, Variable, errors::{ParserError, QuickEvalError, TokenizerError}, parse, quick_eval, value};

#[test]
fn subtraction_order() -> Result<(), MathLibError> {
    let res = quick_eval!("3-4-5")?.to_vec();
    
    assert_eq!(res[0], value!(-6));

    Ok(())
}

#[test]
fn power_order() -> Result<(), MathLibError> {
    let res = quick_eval!("3^2^4")?.to_vec();

    assert_eq!(res[0], value!(43_046_721));

    Ok(())
}

#[test]
fn simple_matrix() -> Result<(), MathLibError> {
    let res = quick_eval!("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]")?.to_vec();

    assert_eq!(res[0], value!(3, 1, 5; 4, 2, 6; 5, 3, 7));

    Ok(())
}

#[test]
fn eval_in_vector1() -> Result<(), MathLibError> {
    let res = quick_eval!("[3, 3/4, 6]")?.to_vec();

    assert_eq!(res[0], value!(3, 0.75, 6));

    Ok(())
}

#[test]
fn negation() -> Result<(), MathLibError> {
    let parsed_ast = parse::<_, f64>("-1")?;

    assert_eq!(parsed_ast.to_string(), "-1".to_string());

    Ok(())
}

#[test]
fn hidden_mult() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3.));
    let res = quick_eval!("3x", &mut Context::from_vars(vec![x]))?.to_vec();

    assert_eq!(res[0], value!(9));
    Ok(())
}

#[test]
fn hidden_vector_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 8));
    let res = quick_eval!("3A", &mut Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(9, 15, 24));

    Ok(())
}

#[test]
fn double_paranth() -> Result<(), MathLibError> {
    let res = quick_eval!("((3*3))")?.to_vec();

    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn matrix_not_rect() {
    let res = quick_eval!("[[3, 0, 5], [2, 4, 5], [1, 2]]");

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn empty_matrix_vector() {
    let res = quick_eval!("[[], [], []]");

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn empty_expr() {
    let res = quick_eval!("");

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn empty_expr_in_matrix1() {
    let res = quick_eval!("[[3, 0,], [2, 4, 5], [1, 2]]");

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn empty_expr_in_matrix2() {
    let res = quick_eval!("[[3, 0, 5], [2, 4], [1, 2,]]");

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn negation_in_vector() -> Result<(), MathLibError> {
    let res = quick_eval!("[-3 ,-5, -2]")?.to_vec();

    assert_eq!(res[0], value!(-3, -5, -2));

    Ok(())
}

#[test]
fn eval_in_vector2() -> Result<(), MathLibError> {
    let res = quick_eval!("[3, 3*3, -5]")?.to_vec();

    assert_eq!(res[0], value!(3, 9, -5));

    return Ok(())
}

#[test]
fn weird_variable_name() -> Result<(), MathLibError> {
    let vars = vec![Variable::new("A_{3*6}", Value::Scalar(3.))];

    let res = quick_eval!("A_{3*6}*3", &mut Context::from_vars(vars))?.to_vec();

    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn hidden_paranth_mult() -> Result<(), MathLibError> {
    let res = quick_eval!("3(6+2)")?.to_vec();

    assert_eq!(res[0], value!(24));

    Ok(())
}

#[test]
fn hidden_bracket_mult() -> Result<(), MathLibError> {
    let res = quick_eval!("3[4, 5, 6]")?.to_vec();

    assert_eq!(res[0], value!(12, 15, 18));

    Ok(())
}

#[test]
fn long_mult_order() -> Result<(), MathLibError> {
    let res = quick_eval!("3*[[2, 0, 0], [0, 1, 0], [0, 0, 5]]*[[1, 0, 0], [0, 1, 0], [0, 0, 1]]*[3, 4, 5]")?.to_vec();

    assert_eq!(res[0], value!(18, 12, 75));

    Ok(())
}

#[test]
fn eval_in_vector3() -> Result<(), MathLibError> {
    let res = quick_eval!("[sqrt(25), 2pi, 3]")?.to_vec();

    assert_eq!(res[0], value!(5, 2.*std::f64::consts::PI, 3));

    Ok(())
}

#[test]
fn unmatched_delimiter() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-(32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)");

    assert_eq!(res, Err(QuickEvalError::ParserError(ParserError::TokenizerError(TokenizerError::UnmatchedDelimiter))));

    Ok(())
}
