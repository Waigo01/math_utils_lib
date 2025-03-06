use crate::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, value, Context, Value, Variable};

#[test]
fn easy_eval1() -> Result<(), MathLibError> {
    let res = quick_eval("3*3", &Context::empty())?.to_vec();
    
    assert_eq!(res[0], value!(9.));

    Ok(())
}

#[test]
fn easy_eval2() -> Result<(), MathLibError> {
    let res = quick_eval("3-4-5", &Context::empty())?.to_vec();
    
    assert_eq!(res[0], value!(-6.));

    Ok(())
}

#[test]
fn easy_eval3() -> Result<(), MathLibError> {
    let res = quick_eval("3^2^4", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(43_046_721.));

    Ok(())
}

#[test]
fn easy_eval4() -> Result<(), MathLibError> {
    let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3., 1., 5.; 4., 2., 6.; 5., 3., 7.));

    Ok(())
}

#[test]
fn easy_eval5() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3/4, 6]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3., 0.75, 6.));

    Ok(())
}

#[test]
fn easy_eval6() -> Result<(), MathLibError> {
    let res = quick_eval("root(8, 3)", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(2.));

    Ok(())
}

#[test]
fn easy_eval7() -> Result<(), MathLibError> {
    let res = quick_eval("&root(9, 2)", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3.), value!(-3.)]);

    Ok(())
}

#[test]
fn easy_eval8() -> Result<(), MathLibError> {
    let res = quick_eval("2*(&sqrt(9))", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(6.), value!(-6.)]);

    Ok(())
}

#[test]
fn easy_eval9() -> Result<(), MathLibError> {
    let res = quick_eval("[{3, 5}, 0, 0]", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3., 0., 0.), value!(5., 0., 0.)]);

    Ok(())
}

#[test]
fn easy_eval10() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(6., 1., 13., 8.; 4., 3., 9., 10.; 4., 0., 14., 8.));
    let b = Variable::new("B", value!(3., 0.; 1., 1.; 0., 6.; 2.5, 2.));
    let res = quick_eval("A*B", &Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(39., 95.; 40., 77.; 32., 100.));

    Ok(())
}

#[test]
fn easy_eval11() -> Result<(), MathLibError> {
    let parsed_ast = parse("-1")?;

    assert_eq!(parsed_ast.as_latex(), "-1".to_string());

    Ok(())
}

#[test]
fn easy_eval12() -> Result<(), MathLibError> {
    let a = Variable::new("A", Value::Matrix(vec![vec![2.]]));

    let res = quick_eval("inv(A)", &Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], Value::Matrix(vec![vec![0.5]]));

    Ok(())
}

#[test]
fn medium_eval1() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3.));
    let res = quick_eval("3x", &Context::from_vars(vec![x]))?.to_vec();

    assert_eq!(res[0], value!(9.));
    Ok(())
}

#[test]
fn medium_eval2() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3., 5., 8.));
    let res = quick_eval("3A", &Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(9., 15., 24.));

    Ok(())
}

#[test]
fn medium_eval3() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3., 5., 8.));
    let b = Variable::new("B", value!(2., 0., 0.; 0., 2., 0.; 0., 0., 1.));
    let res = quick_eval("B*A", &Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(6., 10., 8.));

    Ok(())
}

#[test]
fn medium_eval4() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3., 5., 7.; 4., 8., 2.;1., 9., 2.));
    let b = Variable::new("B", value!(7., 9., 10.; 1., 55., 8.; 22., 9., 2.));
    let res = quick_eval("A*B", &Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(180., 365., 84.; 80., 494., 108.; 60., 522., 86.));

    Ok(())
}

#[test]
fn medium_eval5() -> Result<(), MathLibError> {
    let res = quick_eval("((3*3))", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(9.));

    Ok(())
}

#[test]
fn medium_eval6() {
    let res = quick_eval("[[3, 0, 5], [2, 4, 5], [1, 2]]", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn medium_eval7() {
    let res = quick_eval("[[], [], []]", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn medium_eval8() {
    let res = quick_eval("", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn medium_eval9() {
    let res = quick_eval("[[3, 0,], [2, 4, 5], [1, 2]]", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn medium_eval10() {
    let res = quick_eval("[[3, 0, 5], [2, 4], [1, 2,]]", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn medium_eval11() -> Result<(), MathLibError> {
    let res = quick_eval("[-3 ,-5, -2]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(-3., -5., -2.));

    Ok(())
}

#[test]
fn medium_eval12() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3*3, -5]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3., 9., -5.));

    return Ok(())
}

#[test]
fn medium_eval13() -> Result<(), MathLibError> {
    let vars = vec![Variable::new("A_{3*6}", Value::Scalar(3.))];

    let res = quick_eval("A_{3*6}*3", &Context::from_vars(vars))?.to_vec();

    assert_eq!(res[0], value!(9.));

    Ok(())
}

#[test]
fn medium_eval14() -> Result<(), MathLibError> {
    let res = quick_eval("[[1, 0], [0, 6], [0, 0]]*[3, 4, 5]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(3., 24.));

    Ok(())
}

#[test]
fn medium_eval15() -> Result<(), MathLibError> {
    let res = quick_eval("3(6+2)", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(24.));

    Ok(())
}

#[test]
fn medium_eval16() -> Result<(), MathLibError> {
    let res = quick_eval("3[4, 5, 6]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(12., 15., 18.));

    Ok(())
}

#[test]
fn medium_eval17() -> Result<(), MathLibError> {
    let res = quick_eval("3*[[2, 0, 0], [0, 1, 0], [0, 0, 5]]*[[1, 0, 0], [0, 1, 0], [0, 0, 1]]*[3, 4, 5]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(18., 12., 75.));

    Ok(())
}

#[test]
fn medium_eval18() -> Result<(), MathLibError> {
    let res = quick_eval("[sqrt(25), 2pi, 3]", &Context::default())?.to_vec();

    assert_eq!(res[0], value!(5., 2.*std::f64::consts::PI, 3.));

    Ok(())
}

#[test]
fn medium_eval19() -> Result<(), MathLibError> {
    let res = quick_eval("[0, 0.5, 0]#[-0.8, 0, 0.6]", &Context::empty())?.to_vec();

    assert_eq!(res[0], value!(0.3, 0., 0.4));

    Ok(())
}

#[test]
fn medium_eval20() -> Result<(), MathLibError> {
    let function = parse("5x^2+2x+x")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval("f(5)", &Context::from_funs(vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(140.));

    Ok(())
}

#[test]
fn medium_eval21() -> Result<(), MathLibError> {
    let function = parse("x-A")?;
    let function_var = Function::new("f", function, vec!["x"]);
    let a = Variable::new("A", value!(3., 4., 5.));

    let res = quick_eval("f([3, 4, 5])", &Context::new(vec![a], vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(0., 0., 0.));

    Ok(())
}

#[test]
fn medium_eval22() -> Result<(), MathLibError> {
    let function = parse("3*f(x)")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(5)", &Context::new(vec![], vec![function_var]));

    assert_eq!(res.err().unwrap(), QuickEvalError::EvalError(EvalError::RecursiveFunction));

    Ok(())
}

#[test]
fn medium_eval23() -> Result<(), MathLibError> {
    let function = parse("3*x")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(f(6))", &Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(54.));

    Ok(())
}

#[test]
fn medium_eval24() -> Result<(), MathLibError> {
    let res = quick_eval("[&sqrt(9), &sqrt(9), 0]", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3., 3., 0.), value!(3., -3., 0.), value!(-3., 3., 0.), value!(-3., -3., 0.)]);

    Ok(())
}

#[test]
fn medium_eval25() -> Result<(), MathLibError> {
    let res = quick_eval("[[&sqrt(9), 0, 0], [0, 1, 0], [0, 0, 1]]", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(3., 0., 0.; 0., 1., 0.; 0., 0., 1.), value!(-3., 0., 0.; 0., 1., 0.; 0., 0., 1.)]);

    Ok(())
}

#[test]
fn medium_eval26() -> Result<(), MathLibError> {
    let function = parse("&sqrt(x)+y")?;
    let function_var = Function::new("f", function, vec!["x", "y"]);

    let res = quick_eval("f(&sqrt(16), &sqrt(9))", &Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0..4].to_vec(), vec![value!(5.), value!(1.), value!(-1.), value!(-5.)]);

    Ok(())
}

#[test]
fn medium_eval27() -> Result<(), MathLibError> {
    let res = quick_eval("{3, 2}*{7, 3, 2}", &Context::empty())?.to_vec();

    assert_eq!(res, vec![value!(21.), value!(9.), value!(6.), value!(14.), value!(6.), value!(4.)]);

    Ok(())
}

#[test]
fn medium_eval28() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(1., 2., 3.; 4., 5., 6.; 7., 8., 9.));
    let res = quick_eval("det(A)", &Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(0.));

    Ok(())
}

#[test]
fn medium_eval29() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(0., 6., -2., -1., 5.; 0., 0., 0., -9., -7.; 0., 15., 35., 0., 0.; 0., -1., -11., -2., 1.; -2., -2., 3., 0., -2.));
    let res = quick_eval("det(A)", &Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(2480.));

    Ok(())
}

#[test]
fn medium_eval30() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-1., 3./2.; 1., -1.));

    let res = quick_eval("inv(A)", &Context::from_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(2., 3.; 2., 2.));

    Ok(())
}

#[test]
fn medium_eval31() -> Result<(), MathLibError> {
    let m = Variable::new("M", value!(0.7, 0.1, 0.3; 0.1, 0.5, 0.1; 0.2, 0.4, 0.6));

    let res = quick_eval("M^20", &Context::from_vars(vec![m]))?.round(3).to_vec();

    assert_eq!(res[0], value!(0.444, 0.444, 0.444; 0.167, 0.167, 0.167; 0.389, 0.389, 0.389));

    Ok(())
}

#[test]
fn calculus_eval1() -> Result<(), MathLibError> {
    let res = quick_eval("D(x^2, x, 3)", &Context::empty())?.to_vec();

    assert_eq!(res[0].round(6), value!(6.));

    Ok(())
}

#[test]
fn calculus_eval2() -> Result<(), MathLibError> {
    let res = quick_eval("I(x^2, x, 0, 5)", &Context::empty())?.to_vec();

    assert_eq!(res[0].round(4), value!(41.6667));

    Ok(())
}

#[test]
fn calculus_eval3() -> Result<(), MathLibError> {
    let res = quick_eval("1/sqrt(2*250^2*pi)*I(e^(-(x-4000)^2/(2*250^2)), x, 3500, 4500)", &Context::default())?.to_vec();

    assert_eq!(res[0].round(4), value!(0.9545));

    Ok(())
}

#[test]
fn hard_eval1() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3.));
    let a = Variable::new("A", value!(3., 2., 1.));
    let b = Variable::new("B", value!(2., 3., 4.; 5., 1., 7.; 2., 3., 6.));
    let res = quick_eval("(x*B*A)?1", &Context::from_vars(vec![a, x, b]))?.to_vec();

    assert_eq!(res[0], value!(72.));

    Ok(())
}

#[test]
fn hard_eval2() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-3., -1., 2., -3.; -3., 1., 2., -2.; -2., 3., 0., 1.; 1., -2., -3., 1.));
    let res = quick_eval("inv(A)", &Context::from_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(-11./7., 2., -1., 2./7.; -15./7., 3., -1., 4./7.; 2., -3., 1., -1.; 23./7., -5., 2., -8./7.).round(3));

    Ok(())
}

#[test]
fn easy_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x^2=9, x)", &Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-3.), value!(3.)]);

    Ok(())
}

#[test]
fn medium_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(3x^2+2x-1=0, x)", &Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-1.), value!(((1./3.) as f64*1000.).round()/1000.)]);

    Ok(())
}

#[test]
fn medium_solve2() -> Result<(), MathLibError> {
    let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(3., -8., -2.)]);

    Ok(())
}

#[test]
fn medium_solve3() -> Result<(), MathLibError> {
    let equation = "eq(3x-9z=33, 7x-4y-z=-15, 4x+6y+5z=-6, x, y, z)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1., 3., -4.)]);

    Ok(())
}

#[test]
fn medium_solve4() -> Result<(), MathLibError> {
    let function = parse("4x^2-9")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval("eq(f(x)=0, x)", &Context::from_funs(vec![function_var]))?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1.5), value!(1.5)]);

    Ok(())
}

#[test]
fn calculus_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(D(3x^2+2x-1, x, k)=0, k)", &Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-(((1./3.) as f64*1000.).round()/1000.))]);

    Ok(())
}

#[test]
fn hard_solve1() -> Result<(), MathLibError> {
    let equation = "eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

    assert_eq!(res.contains(&value!(-0.656)), true);

    Ok(())
}

#[test]
fn hard_solve2() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x*[3, 4, 5]=[6, 8, 10], x)", &Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::EvalError(EvalError::VectorInEq));

    Ok(())
}

#[test]
fn hard_solve3() -> Result<(), MathLibError> {
    let equation = "eq(400-100x=600-100x, -600-100x=-400-100x, 1000-100x=0+100x, x)";

    let res = quick_eval(equation, &Context::empty())?.to_vec();

    assert_eq!(res, vec![]);

    Ok(())
}

#[test]
fn hard_solve4() -> Result<(), MathLibError> {
    let equation = "eq(400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k, g, k)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(4., 6.)]);

    Ok(())
}

#[test]
fn hard_solve5() -> Result<(), MathLibError> {
    let equation = "eq(y=x^2+6x-8, y=4x+7, x, y)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(-5., -13.), value!(3., 19.)]);

    Ok(())
}

#[test]
fn hard_solve6() -> Result<(), MathLibError> {
    let equation = "eq(y=1-3x, x^2/4+y^2=1, x, y)";

    let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

    assert_eq!(res, vec![value!(24./37., -35./37.).round(3), value!(0., 1.)]);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output1() -> Result<(), MathLibError> {
    use crate::{eval, export_history, ExportType, Step};
    use std::fs;

    let parsed_expr = parse("3*3+6^5")?;
    let res = eval(&parsed_expr, &Context::empty())?;

    let step = Step::Calc { term: parsed_expr, result: res, variable_save: Some("x".to_string()) };

    let pdf = export_history(vec![step], ExportType::Pdf)?;

    let _ = fs::write("./images/test.pdf", pdf);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output2() -> Result<(), MathLibError> {
    use crate::{eval, png_from_latex, Step};
    use std::fs;

    let parsed_expr = parse("3*3+6^5")?;
    let res = eval(&parsed_expr, &Context::empty())?;

    let step = Step::Calc { term: parsed_expr, result: res, variable_save: Some("x".to_string()) };

    let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;

    let _ = fs::write("./images/test.png", png);

    Ok(())
}
