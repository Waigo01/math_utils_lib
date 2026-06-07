use crate::{Context, Value, Values, Variable, basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError, TokenizerError}, parse, quick_eval, value};

#[test]
fn simple_multiplication() -> Result<(), MathLibError> {
    let res = quick_eval("3*3", &mut Context::empty())?.to_vec();
    
    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn subtraction_order() -> Result<(), MathLibError> {
    let res = quick_eval("3-4-5", &mut Context::empty())?.to_vec();
    
    assert_eq!(res[0], value!(-6));

    Ok(())
}

#[test]
fn power_order() -> Result<(), MathLibError> {
    let res = quick_eval("3^2^4", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(43_046_721));

    Ok(())
}

#[test]
fn simple_matrix() -> Result<(), MathLibError> {
    let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3, 1, 5; 4, 2, 6; 5, 3, 7));

    Ok(())
}

#[test]
fn eval_in_vector1() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3/4, 6]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3, 0.75, 6));

    Ok(())
}

#[test]
fn root_function() -> Result<(), MathLibError> {
    let res = quick_eval("root(8, 3)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(2));

    Ok(())
}

#[test]
fn plus_minus_root() -> Result<(), MathLibError> {
    let res = quick_eval("+-root(9, 2)", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3), value!(-3)]);

    Ok(())
}

#[test]
fn list_eval() -> Result<(), MathLibError> {
    let res = quick_eval("2*(+-sqrt(9))", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(6), value!(-6)]);

    Ok(())
}

#[test]
fn list_in_vector() -> Result<(), MathLibError> {
    let res = quick_eval("[{3, 5}, 0, 0]", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3, 0, 0), value!(5, 0, 0)]);

    Ok(())
}

#[test]
fn matrix_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(6, 1, 13, 8; 4, 3, 9, 10; 4, 0, 14, 8));
    let b = Variable::new("B", value!(3, 0; 1, 1; 0, 6; 2.5, 2));
    let res = quick_eval("A*B", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(39, 95; 40, 77; 32, 100));

    Ok(())
}

#[test]
fn negation() -> Result<(), MathLibError> {
    let parsed_ast = parse("-1")?;

    assert_eq!(parsed_ast.as_latex(), "-1".to_string());

    Ok(())
}

#[test]
fn matrix_inverse1() -> Result<(), MathLibError> {
    let a = Variable::new("A", Value::Matrix(vec![vec![2.]]));

    let res = quick_eval("inv(A)", &mut Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], Value::Matrix(vec![vec![0.5]]));

    Ok(())
}

#[test]
fn hidden_mult() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3.));
    let res = quick_eval("3x", &mut Context::from_vars(vec![x]))?.to_vec();

    assert_eq!(res[0], value!(9));
    Ok(())
}

#[test]
fn hidden_vector_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 8));
    let res = quick_eval("3A", &mut Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(9, 15, 24));

    Ok(())
}

#[test]
fn matrix_vector_mult1() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 8));
    let b = Variable::new("B", value!(2, 0, 0; 0, 2, 0; 0, 0, 1));
    let res = quick_eval("B*A", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(6, 10, 8));

    Ok(())
}

#[test]
fn large_matrix_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 7; 4, 8, 2; 1, 9, 2));
    let b = Variable::new("B", value!(7, 9, 10; 1, 55, 8; 22, 9, 2));
    let res = quick_eval("A*B", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(180, 365, 84; 80, 494, 108; 60, 522, 86));

    Ok(())
}

#[test]
fn double_paranth() -> Result<(), MathLibError> {
    let res = quick_eval("((3*3))", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn matrix_not_rect() {
    let res = quick_eval("[[3, 0, 5], [2, 4, 5], [1, 2]]", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn empty_matrix_vector() {
    let res = quick_eval("[[], [], []]", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn empty_expr() {
    let res = quick_eval("", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn empty_expr_in_matrix1() {
    let res = quick_eval("[[3, 0,], [2, 4, 5], [1, 2]]", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn empty_expr_in_matrix2() {
    let res = quick_eval("[[3, 0, 5], [2, 4], [1, 2,]]", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn negation_in_vector() -> Result<(), MathLibError> {
    let res = quick_eval("[-3 ,-5, -2]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(-3, -5, -2));

    Ok(())
}

#[test]
fn eval_in_vector2() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3*3, -5]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3, 9, -5));

    return Ok(())
}

#[test]
fn weird_variable_name() -> Result<(), MathLibError> {
    let vars = vec![Variable::new("A_{3*6}", Value::Scalar(3.))];

    let res = quick_eval("A_{3*6}*3", &mut Context::from_vars(vars))?.to_vec();

    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn matrix_vector_mult2() -> Result<(), MathLibError> {
    let res = quick_eval("[[1, 0], [0, 6], [0, 0]]*[3, 4, 5]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3, 24));

    Ok(())
}

#[test]
fn hidden_paranth_mult() -> Result<(), MathLibError> {
    let res = quick_eval("3(6+2)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(24));

    Ok(())
}

#[test]
fn hidden_bracket_mult() -> Result<(), MathLibError> {
    let res = quick_eval("3[4, 5, 6]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(12, 15, 18));

    Ok(())
}

#[test]
fn long_mult_order() -> Result<(), MathLibError> {
    let res = quick_eval("3*[[2, 0, 0], [0, 1, 0], [0, 0, 5]]*[[1, 0, 0], [0, 1, 0], [0, 0, 1]]*[3, 4, 5]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(18, 12, 75));

    Ok(())
}

#[test]
fn eval_in_vector3() -> Result<(), MathLibError> {
    let res = quick_eval("[sqrt(25), 2pi, 3]", &mut Context::default())?.to_vec();

    assert_eq!(res[0], value!(5, 2.*std::f64::consts::PI, 3));

    Ok(())
}

#[test]
fn medium_eval19() -> Result<(), MathLibError> {
    let res = quick_eval("[0, 0.5, 0]#[-0.8, 0, 0.6]", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(0.3, 0, 0.4));

    Ok(())
}

#[test]
fn custom_function() -> Result<(), MathLibError> {
    let function = parse("5x^2+2x+x")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval("f(5)", &mut Context::from_funs(vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(140));

    Ok(())
}

#[test]
fn custom_function_vector_input() -> Result<(), MathLibError> {
    let function = parse("x-A")?;
    let function_var = Function::new("f", function, vec!["x"]);
    let a = Variable::new("A", value!(3., 4., 5.));

    let res = quick_eval("f([3, 4, 5])", &mut Context::new(vec![a], vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(0, 0, 0));

    Ok(())
}

#[test]
fn custom_function_recursion() -> Result<(), MathLibError> {
    let function = parse("3*f(x)")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(5)", &mut Context::new(vec![], vec![function_var]));

    assert_eq!(res.err().unwrap(), QuickEvalError::EvalError(EvalError::RecursiveFunction));

    Ok(())
}

#[test]
fn custom_function_of_function() -> Result<(), MathLibError> {
    let function = parse("3*x")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(f(6))", &mut Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(54.));

    Ok(())
}

#[test]
fn cartesian_vector_from_lists() -> Result<(), MathLibError> {
    let res = quick_eval("[+-sqrt(9), +-sqrt(9), 0]", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3, 3, 0), value!(3, -3, 0), value!(-3, 3, 0), value!(-3, -3, 0)]);

    Ok(())
}

#[test]
fn list_in_matrix() -> Result<(), MathLibError> {
    let res = quick_eval("[[+-sqrt(9), 0, 0], [0, 1, 0], [0, 0, 1]]", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3, 0, 0; 0, 1, 0; 0, 0, 1), value!(-3, 0, 0; 0, 1, 0; 0, 0, 1)]);

    Ok(())
}

#[test]
fn custom_function_list_input_output() -> Result<(), MathLibError> {
    let function = parse("+-sqrt(x)+y")?;
    let function_var = Function::new("f", function, vec!["x", "y"]);

    let res = quick_eval("f(+-sqrt(16), +-sqrt(9))", &mut Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0..4].to_vec(), vec![value!(5), value!(1), value!(-1), value!(-5)]);

    Ok(())
}

#[test]
fn list_mult() -> Result<(), MathLibError> {
    let res = quick_eval("{3, 2}*{7, 3, 2}", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(21), value!(9), value!(6), value!(14), value!(6), value!(4)]);

    Ok(())
}

#[test]
fn matrix_det() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(1, 2, 3; 4, 5, 6; 7, 8, 9));
    let res = quick_eval("det(A)", &mut Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn large_matrix_det() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(0, 6, -2, -1, 5; 0, 0, 0, -9, -7; 0, 15, 35, 0, 0; 0, -1, -11, -2, 1; -2, -2, 3, 0., -2));
    let res = quick_eval("det(A)", &mut Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(2480));

    Ok(())
}

#[test]
fn matrix_inverse2() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-1, 3./2.; 1, -1));

    let res = quick_eval("inv(A)", &mut Context::from_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(2, 3; 2, 2));

    Ok(())
}

#[test]
fn matrix_power1() -> Result<(), MathLibError> {
    let m = Variable::new("M", value!(0.7, 0.1, 0.3; 0.1, 0.5, 0.1; 0.2, 0.4, 0.6));

    let res = quick_eval("M^20", &mut Context::from_vars(vec![m]))?.round(3).to_vec();

    assert_eq!(res[0], value!(0.444, 0.444, 0.444; 0.167, 0.167, 0.167; 0.389, 0.389, 0.389));

    Ok(())
}

#[test]
fn matrix_power2() -> Result<(), MathLibError> {
    let m = Variable::new("M", value!(0., 2., 0.; 0.5, 0., 0.; 0., 0.8, 0.));

    let res = quick_eval("M^2", &mut Context::from_vars(vec![m]))?.round(3).to_vec();

    assert_eq!(res[0], value!(1, 0, 0; 0, 1, 0; 0.4, 0, 0));

    Ok(())
}

#[test]
fn unmatched_delimiter() -> Result<(), MathLibError> {
    let res = quick_eval("eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-(32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)", &mut Context::empty());

    assert_eq!(res, Err(QuickEvalError::ParserError(ParserError::TokenizerError(TokenizerError::UnmatchedDelimiter))));

    Ok(())
}

#[test]
fn mutable_context_variable1() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("x = 10", &mut c)?;

    assert_eq!(c.get_var("x"), Some(Variable::new("x", vec![value!(10)])));

    let res = quick_eval("x", &mut c)?.to_vec();

    assert_eq!(res[0], value!(10));

    Ok(())
}

#[test]
fn mutable_context_variable2() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("x = eq(x^2=9, x)", &mut c)?;

    let res = quick_eval("x", &mut c)?.round(3).to_vec();

    assert_eq!(res, vec![value!(-3), value!(3)]);

    Ok(())
}

#[test]
fn mutable_context_function() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("f(x) = x^2", &mut c)?;

    let res = quick_eval("f(5)", &mut c)?.to_vec();

    assert_eq!(res[0], value!(25));

    Ok(())
}

#[test]
fn mutable_context_multi_variable() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("{x, y, z} = {1, 2, 3}", &mut c)?;

    let res = quick_eval("x", &mut c)?.to_vec();
    assert_eq!(res[0], value!(1));

    let res = quick_eval("y", &mut c)?.to_vec();
    assert_eq!(res[0], value!(2));

    let res = quick_eval("z", &mut c)?.to_vec();
    assert_eq!(res[0], value!(3));

    Ok(())
}

#[test]
fn mutable_context_variable_list() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("x = {1, 2, 3}", &mut c)?;

    let res = quick_eval("x", &mut c)?.to_vec();
    assert_eq!(res, vec![value!(1), value!(2), value!(3)]);

    Ok(())
}

#[test]
fn function_side_effects() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("f(x) = y=x", &mut c)?;

    quick_eval("f(5)", &mut c)?;

    assert_eq!(c.get_var("y".to_string()).unwrap().values.to_vec()[0], value!(5));

    Ok(())
}

#[test]
fn function_side_multi_effects() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval("f(x) = z=y=x*2", &mut c)?;

    quick_eval("f(5)", &mut c)?;

    assert_eq!(c.get_var("y".to_string()), Some(Variable { name: "y".to_string(), values: Values::from_vec(vec![value!(10)]) }));
    assert_eq!(c.get_var("z".to_string()), Some(Variable { name: "z".to_string(), values: Values::from_vec(vec![value!(10)]) }));

    Ok(())
}

#[test]
fn boolean_equals() -> Result<(), MathLibError> {
    let res = quick_eval("6==6", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_not() -> Result<(), MathLibError> {
    let res = quick_eval("!(10==6)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_or() -> Result<(), MathLibError> {
    let res = quick_eval("6==10 | 10==10", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_and() -> Result<(), MathLibError> {
    let res = quick_eval("6==6 & 10", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_not_eq() -> Result<(), MathLibError> {
    let res = quick_eval("6!=10 & 10==10", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general1() -> Result<(), MathLibError> {
    let res = quick_eval("6==10 | 10!=10", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general2() -> Result<(), MathLibError> {
    let res = quick_eval("6==6 & 6!=10", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general3() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval("a = 6", &mut c)?;
    quick_eval("b = 10", &mut c)?;

    let res = quick_eval("!(a!=10 | b!=10)", &mut c)?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general4() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval("a = 6", &mut c)?;
    quick_eval("b = 10", &mut c)?;

    let res = quick_eval("a!=6 & b==10", &mut c)?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general5() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval("a = 6", &mut c)?;
    quick_eval("b = 10", &mut c)?;

    let res = quick_eval("!(a != 10 | b != 10) | b != 5", &mut c)?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_equation() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x^2 = 9, x) == -3", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(1), value!(0)]);

    Ok(())
}

#[test]
fn boolean_less() -> Result<(), MathLibError> {
    let res = quick_eval("2<3", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_less_eq() -> Result<(), MathLibError> {
    let res = quick_eval("(2<3 & 3<2) | 3<=3", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general6() -> Result<(), MathLibError> {
    let res = quick_eval("abs(eq(x^2 = 0, x)) < 10^(-4)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn if_statement1() -> Result<(), MathLibError> {
    let res = quick_eval("if([2, 3, 4]==[2, 3, 4], 5, 2)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0], value!(5));

    Ok(())
}

#[test]
fn if_statement2() -> Result<(), MathLibError> {
    let res = quick_eval("if(eq(x^2 = 9, x) == -3, 5, 2)", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(5), value!(2)]);

    Ok(())
}

#[test]
fn if_statement3() -> Result<(), MathLibError> {
    let res = quick_eval("if({1, 2}==1, 2)", &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(2)]);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn if_statement_export() -> Result<(), MathLibError> {
    use crate::{eval, png_from_latex, Step};

    let parsed_expr = parse("f(x) =
    if(x == 0 & state == 0,
        state = 1,
    if(x == 1 & state == 1,
        state = 2,
    if(x == 2 & state == 2,
        state = 3,
    if(x == 3 & state == 3,
        state = 4,
    state = 0))))")?;

    let res = eval(&parsed_expr, &mut Context::empty())?;

    let step = Step::new(parsed_expr, res);

    let png = png_from_latex(step.as_latex_inline(), 100, "#FFFFFF")?;

    let _ = std::fs::write("./images/if_test.png", png);

    Ok(())
}

#[test]
fn state_machine1() -> Result<(), MathLibError> {
    //m = 0,
    //a = 1,
    //t = 2,
    //h = 3

    let mut c = Context::empty();
    quick_eval("state = 0", &mut c)?;

    quick_eval("f(x) =
    if(x == 0 & state == 0,
        state = 1,
    if(x == 1 & state == 1,
        state = 2,
    if(x == 2 & state == 2,
        state = 3,
    if(x == 3 & state == 3,
        state = 4,
    state = 0))))", &mut c)?;

    quick_eval("f({0, 1, 2, 3})", &mut c)?;

    let res = quick_eval("state", &mut c)?.to_vec();

    assert_eq!(res[0], value!(4));
    
    quick_eval("state = 0", &mut c)?;

    quick_eval("f({0, 1, 4, 2})", &mut c)?.to_vec();
    
    let res = quick_eval("state", &mut c)?.to_vec();

    assert_ne!(res[0], value!(4));

    Ok(())
}

#[test]
fn derivative() -> Result<(), MathLibError> {
    let res = quick_eval("D(x^2, x, 3)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0].round(6), value!(6));

    Ok(())
}

#[test]
fn integral() -> Result<(), MathLibError> {
    let res = quick_eval("I(x^2, x, 0, 5)", &mut Context::empty())?.to_vec();

    assert_eq!(res[0].round(4), value!(41.6667));

    Ok(())
}

#[test]
fn hard_integral() -> Result<(), MathLibError> {
    let res = quick_eval("1/sqrt(2*250^2*pi)*I(e^(-(x-4000)^2/(2*250^2)), x, 3500, 4500)", &mut Context::default())?.to_vec();

    assert_eq!(res[0].round(4), value!(0.9545));

    Ok(())
}

#[test]
fn positional_get() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3));
    let a = Variable::new("A", value!(3, 2, 1));
    let b = Variable::new("B", value!(2, 3, 4; 5, 1, 7; 2, 3, 6));
    let res = quick_eval("(x*B*A)@1", &mut Context::from_vars(vec![a, x, b]))?.to_vec();

    assert_eq!(res[0], value!(72));

    Ok(())
}

#[test]
fn matrix_inverse3() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-3, -1, 2, -3; -3, 1, 2, -2; -2, 3, 0, 1; 1, -2, -3, 1));
    let res = quick_eval("inv(A)", &mut Context::from_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(-11./7., 2, -1, 2./7.; -15./7., 3, -1, 4./7.; 2, -3, 1, -1; 23./7., -5, 2, -8./7.).round(3));

    Ok(())
}

#[test]
fn solve_parabolic() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x^2=9, x)", &mut Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-3), value!(3)]);

    Ok(())
}

#[test]
fn solve_polynomial() -> Result<(), MathLibError> {
    let res = quick_eval("eq(3x^2+2x-1=0, x)", &mut Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-1), value!(1./3.).round(3)]);

    Ok(())
}

#[test]
fn solve_linear_system1() -> Result<(), MathLibError> {
    let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(3, -8, -2)]);

    Ok(())
}

#[test]
fn solve_linear_system2() -> Result<(), MathLibError> {
    let equation = "eq(3x-9z=33, 7x-4y-z=-15, 4x+6y+5z=-6, x, y, z)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1, 3, -4)]);

    Ok(())
}

#[test]
fn solve_custom_function() -> Result<(), MathLibError> {
    let function = parse("4x^2-9")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval("eq(f(x)=0, x)", &mut Context::from_funs(vec![function_var]))?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1.5), value!(1.5)]);

    Ok(())
}

#[test]
fn solve_extremum() -> Result<(), MathLibError> {
    let res = quick_eval("eq(D(3x^2+2x-1, x, k)=0, k)", &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1./3.).round(3)]);

    Ok(())
}

#[test]
fn crazy_solve() -> Result<(), MathLibError> {
    let equation = "eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res.contains(&value!(-0.656)), true);

    Ok(())
}

#[test]
fn solve_vector() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x*[3, 4, 5]=[6, 8, 10], x)", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::EvalError(EvalError::VectorInEq));

    Ok(())
}

#[test]
fn solve_overdefined_system1() -> Result<(), MathLibError> {
    let equation = "eq(400-100x=600-100x, -600-100x=-400-100x, 1000-100x=0+100x, x)";

    let res = quick_eval(equation, &mut Context::empty())?.to_vec();

    assert_eq!(res, vec![]);

    Ok(())
}

#[test]
fn solve_overdefined_system2() -> Result<(), MathLibError> {
    let equation = "eq(400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k, g, k)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(4, 6)]);

    Ok(())
}

#[test]
fn solve_nonlinear_system1() -> Result<(), MathLibError> {
    let equation = "eq(y=x^2+6x-8, y=4x+7, x, y)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-5, -13), value!(3, 19)]);

    Ok(())
}

#[test]
fn solve_nonlinear_system2() -> Result<(), MathLibError> {
    let equation = "eq(y=1-3x, x^2/4+y^2=1, x, y)";

    let res = quick_eval(equation, &mut Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(24./37., -35./37.).round(3), value!(0, 1)]);

    Ok(())
}

#[test]
fn variable_as_latex() -> Result<(), MathLibError> {
    let res1 = quick_eval("{2, 3, 4}", &mut Context::empty())?;
    let res2 = quick_eval("f(x) = x^2", &mut Context::empty())?;
    let res3 = quick_eval("4", &mut Context::empty())?;

    let var1 = Variable::new("x", res1);
    let var2 = Variable::new("y", res2);
    let var3 = Variable::new("z", res3);

    assert_eq!(var1.as_latex(true), r"x&≔\left\{2, 3, 4\right\}");
    assert_eq!(var2.as_latex(true), r"y&≔\left\{\right\}");
    assert_eq!(var3.as_latex(true), r"z&≔4");

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output1() -> Result<(), MathLibError> {
    use crate::{eval, export_history, ExportType, Step};
    use std::fs;

    let mut c = Context::empty();

    let parsed_expr = parse("x = 3*3+6^5")?;
    let res = eval(&parsed_expr, &mut c)?;

    let step1 = Step::new(parsed_expr, res);

    let parsed_expr = parse("3x")?;
    let res = eval(&parsed_expr, &mut c)?;

    let step2 = Step::new(parsed_expr, res);

    let pdf = export_history(vec![step1, step2], ExportType::Pdf)?;

    let _ = fs::write("./images/test.pdf", pdf);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output2() -> Result<(), MathLibError> {
    use crate::{eval, png_from_latex, Step};
    use std::fs;

    let parsed_expr = parse("x = 3*3+6^5")?;
    let res = eval(&parsed_expr, &mut Context::empty())?;

    let step = Step::new(parsed_expr, res);

    let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;

    let _ = fs::write("./images/test.png", png);

    Ok(())
}
