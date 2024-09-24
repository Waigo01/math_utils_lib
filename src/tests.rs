use crate::{basetypes::Function, errors::{EvalError, MathLibError, ParserError, QuickEvalError}, parse, quick_eval, Context, Value, Variable};

#[test]
fn easy_eval1() -> Result<(), MathLibError> {
    let res = quick_eval("3*3", Context::empty())?.to_vec();
    
    assert_eq!(res[0], Value::Scalar(9.));

    Ok(())
}

#[test]
fn easy_eval2() -> Result<(), MathLibError> {
    let res = quick_eval("3-4-5", Context::empty())?.to_vec();
    
    assert_eq!(res[0], Value::Scalar(-6.));

    Ok(())
}

#[test]
fn easy_eval3() -> Result<(), MathLibError> {
    let res = quick_eval("3^2^4", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Scalar(43_046_721.));

    Ok(())
}

#[test]
fn easy_eval4() -> Result<(), MathLibError> {
    let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Matrix(vec![vec![3., 1., 5.], vec![4., 2., 6.], vec![5., 3., 7.]]));

    Ok(())
}

#[test]
fn easy_eval5() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3/4, 6]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![3., 0.75, 6.]));

    Ok(())
}

#[test]
fn easy_eval6() -> Result<(), MathLibError> {
    let res = quick_eval("root(8, 3)", Context::empty())?.to_vec();

    assert_eq!(res, vec![Value::Scalar(2.)]);

    Ok(())
}

#[test]
fn easy_eval7() -> Result<(), MathLibError> {
    let res = quick_eval("root(9, 2)", Context::empty())?.to_vec();

    assert_eq!(res, vec![Value::Scalar(3.), Value::Scalar(-3.)]);

    Ok(())
}

#[test]
fn easy_eval8() -> Result<(), MathLibError> {
    let res = quick_eval("2*sqrt(9)", Context::empty())?.to_vec();

    assert_eq!(res, vec![Value::Scalar(6.), Value::Scalar(-6.)]);

    Ok(())
}

#[test]
fn medium_eval1() -> Result<(), MathLibError> {
    let x = Variable::new("x", vec![Value::Scalar(3.)]);
    let res = quick_eval("3x", Context::from_vars(vec![x]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(9.));
    Ok(())
}

#[test]
fn medium_eval2() -> Result<(), MathLibError> {
    let a = Variable::new("A", vec![Value::Vector(vec![3., 5., 8.])]);
    let res = quick_eval("3A", Context::from_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![9., 15., 24.]));

    Ok(())
}

#[test]
fn medium_eval3() -> Result<(), MathLibError> {
    let a = Variable::new("A", vec![Value::Vector(vec![3., 5., 8.])]);
    let b = Variable::new("B", vec![Value::Matrix(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 1.]])]);
    let res = quick_eval("B*A", Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![6., 10., 8.]));

    Ok(())
}

#[test]
fn medium_eval4() -> Result<(), MathLibError> {
    let a = Variable::new("A", vec![Value::Matrix(vec![vec![3., 5., 7.], vec![4., 8., 2.], vec![1., 9., 2.]])]);
    let b = Variable::new("B", vec![Value::Matrix(vec![vec![7., 9., 10.], vec![1., 55., 8.], vec![22., 9., 2.]])]);
    let res = quick_eval("A*B", Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], Value::Matrix(vec![vec![180., 365., 84.], vec![80., 494., 108.], vec![60., 522., 86.]]));

    Ok(())
}

#[test]
fn medium_eval5() -> Result<(), MathLibError> {
    let res = quick_eval("((3*3))", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Scalar(9.));

    Ok(())
}

#[test]
fn medium_eval6() {
    let res = quick_eval("[[3, 0, 5], [2, 4, 5], [1, 2]]", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn medium_eval7() {
    let res = quick_eval("[[], [], []]", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn medium_eval8() {
    let res = quick_eval("", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn medium_eval9() {
    let res = quick_eval("[[3, 0,], [2, 4, 5], [1, 2]]", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn medium_eval10() {
    let res = quick_eval("[[3, 0, 5], [2, 4], [1, 2,]]", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn medium_eval11() -> Result<(), MathLibError> {
    let res = quick_eval("[-3 ,-5, -2]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![-3., -5., -2.]));

    Ok(())
}

#[test]
fn medium_eval12() -> Result<(), MathLibError> {
    let res = quick_eval("[3, 3*3, -5]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![3., 9., -5.]));

    return Ok(())
}

#[test]
fn medium_eval13() -> Result<(), MathLibError> {
    let vars = vec![Variable::new("A_{3*6}", vec![Value::Scalar(3.)])];

    let res = quick_eval("A_{3*6}*3", Context::from_vars(vars))?.to_vec();

    assert_eq!(res[0], Value::Scalar(9.));

    Ok(())
}

#[test]
fn medium_eval14() -> Result<(), MathLibError> {
    let res = quick_eval("[[1, 0], [0, 6], [0, 0]]*[3, 4, 5]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![3., 24.]));

    Ok(())
}

#[test]
fn medium_eval15() -> Result<(), MathLibError> {
    let res = quick_eval("3(6+2)", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Scalar(24.));

    Ok(())
}

#[test]
fn medium_eval16() -> Result<(), MathLibError> {
    let res = quick_eval("3[4, 5, 6]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![12., 15., 18.]));

    Ok(())
}

#[test]
fn medium_eval17() -> Result<(), MathLibError> {
    let res = quick_eval("3*[[2, 0, 0], [0, 1, 0], [0, 0, 5]]*[[1, 0, 0], [0, 1, 0], [0, 0, 1]]*[3, 4, 5]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![18., 12., 75.]));

    Ok(())
}

#[test]
fn medium_eval18() -> Result<(), MathLibError> {
    let res = quick_eval("[sqrt(25), 2pi, 3]", Context::default())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![5., 2.*std::f64::consts::PI, 3.]));

    Ok(())
}

#[test]
fn medium_eval19() -> Result<(), MathLibError> {
    let res = quick_eval("[0, 0.5, 0]#[-0.8, 0, 0.6]", Context::empty())?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![0.3, 0., 0.4]));

    Ok(())
}

#[test]
fn medium_eval20() -> Result<(), MathLibError> {
    let function = parse("5x^2+2x+x")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval("f(5)", Context::from_funs(vec![function_var]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(140.));

    Ok(())
}

#[test]
fn medium_eval21() -> Result<(), MathLibError> {
    let function = parse("x-A")?;
    let function_var = Function::new("f", function, vec!["x"]);
    let a = Variable::new("A", vec![Value::Vector(vec![3., 4., 5.])]);

    let res = quick_eval("f([3, 4, 5])", Context::new(vec![a], vec![function_var]))?.to_vec();

    assert_eq!(res[0], Value::Vector(vec![0., 0., 0.]));

    Ok(())
}

#[test]
fn medium_eval22() -> Result<(), MathLibError> {
    let function = parse("3*f(x)")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(5)", Context::new(vec![], vec![function_var]));

    assert_eq!(res.err().unwrap(), QuickEvalError::EvalError(EvalError::RecursiveFunction));

    Ok(())
}

#[test]
fn medium_eval23() -> Result<(), MathLibError> {
    let function = parse("3*x")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval("f(f(6))", Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(54.));

    Ok(())
}

#[test]
fn medium_eval24() -> Result<(), MathLibError> {
    let res = quick_eval("[sqrt(9), sqrt(9), 0]", Context::empty())?.to_vec();

    assert_eq!(res, vec![Value::Vector(vec![3., 3., 0.]), Value::Vector(vec![3., -3., 0.]), Value::Vector(vec![-3., 3., 0.]), Value::Vector(vec![-3., -3., 0.])]);

    Ok(())
}

#[test]
fn medium_eval25() -> Result<(), MathLibError> {
    let res = quick_eval("[[sqrt(9), 0, 0], [0, 1, 0], [0, 0, 1]]", Context::empty())?.to_vec();

    assert_eq!(res, vec![Value::Matrix(vec![vec![3., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]), Value::Matrix(vec![vec![-3., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]])]);

    Ok(())
}

#[test]
fn medium_eval26() -> Result<(), MathLibError> {
    let function = parse("sqrt(x)+y")?;
    let function_var = Function::new("f", function, vec!["x", "y"]);

    let res = quick_eval("f(sqrt(16), sqrt(9))", Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0..4].to_vec(), vec![Value::Scalar(5.), Value::Scalar(1.), Value::Scalar(-1.), Value::Scalar(-5.)]);

    Ok(())
}

#[test]
fn calculus_eval1() -> Result<(), MathLibError> {
    let res = quick_eval("D(x^2, x, 3)", Context::empty())?.to_vec();

    assert_eq!(res[0].round(6), Value::Scalar(6.));

    Ok(())
}

#[test]
fn calculus_eval2() -> Result<(), MathLibError> {
    let res = quick_eval("I(x^2, x, 0, 5)", Context::empty())?.to_vec();

    assert_eq!(res[0].round(4), Value::Scalar(41.6667));

    Ok(())
}

#[test]
fn hard_eval1() -> Result<(), MathLibError> {
    let x = Variable::new("x", vec![Value::Scalar(3.)]);
    let a = Variable::new("A", vec![Value::Vector(vec![3., 2., 1.])]);
    let b = Variable::new("B", vec![Value::Matrix(vec![vec![2., 3., 4.], vec![5., 1., 7.], vec![2., 3., 6.]])]);
    let res = quick_eval("(x*B*A)?1", Context::from_vars(vec![a, x, b]))?.to_vec();

    assert_eq!(res[0], Value::Scalar(72.));

    Ok(())
}

#[test]
fn easy_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x^2=9, x)", Context::empty())?.to_vec();
    
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Scalar(-3.), Value::Scalar(3.)]);

    Ok(())
}

#[test]
fn medium_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(3x^2+2x-1=0, x)", Context::empty())?.to_vec();
    
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Scalar(-1.), Value::Scalar(((1./3.) as f64*1000.).round()/1000.)]);

    Ok(())
}

#[test]
fn medium_solve2() -> Result<(), MathLibError> {
    let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";

    let res = quick_eval(equation, Context::empty())?.to_vec();
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Vector(vec![3., -8., -2.])]);

    Ok(())
}

#[test]
fn medium_solve3() -> Result<(), MathLibError> {
    let equation = "eq(3x-9z=33, 7x-4y-z=-15, 4x+6y+5z=-6, x, y, z)";

    let res = quick_eval(equation, Context::empty())?.to_vec();
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Vector(vec![-1., 3., -4.])]);

    Ok(())
}

#[test]
fn medium_solve4() -> Result<(), MathLibError> {
    let function = parse("4x^2-9")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let mut res = quick_eval("eq(f(x)=0, x)", Context::from_funs(vec![function_var]))?.to_vec();

    res = res.iter().map(|r| r.round(3)).collect();

    assert_eq!(res, vec![Value::Scalar(-1.5), Value::Scalar(1.5)]);

    Ok(())
}

#[test]
fn calculus_solve1() -> Result<(), MathLibError> {
    let res = quick_eval("eq(D(3x^2+2x-1, x, k)=0, k)", Context::empty())?.to_vec();

    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Scalar(-(((1./3.) as f64*1000.).round()/1000.))]);

    Ok(())
}

#[test]
fn hard_solve1() -> Result<(), MathLibError> {
    let equation = "eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)";

    let res = quick_eval(equation, Context::empty())?.to_vec();

    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded.contains(&Value::Scalar(-0.656)), true);

    Ok(())
}

#[test]
fn hard_solve2() -> Result<(), MathLibError> {
    let res = quick_eval("eq(x*[3, 4, 5]=[6, 8, 10], x)", Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::EvalError(EvalError::VectorInEq));

    Ok(())
}

#[test]
fn hard_solve3() -> Result<(), MathLibError> {
    let equation = "eq(400-100x=600-100x, -600-100x=-400-100x, 1000-100x=0+100x, x)";

    let res = quick_eval(equation, Context::empty())?.to_vec();

    assert_eq!(res, vec![]);

    Ok(())
}

#[test]
fn hard_solve4() -> Result<(), MathLibError> {
    let equation = "eq(400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k, g, k)";

    let res = quick_eval(equation, Context::empty())?.to_vec();
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();
    
    assert_eq!(res_rounded, vec![Value::Vector(vec![4., 6.])]);

    Ok(())
}

#[test]
fn hard_solve5() -> Result<(), MathLibError> {
    let equation = "eq(y=x^2+6x-8, y=4x+7, x, y)";

    let res = quick_eval(equation, Context::empty())?.to_vec();
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Vector(vec![-5., -13.]), Value::Vector(vec![3., 19.])]);

    Ok(())
}

#[test]
fn hard_solve6() -> Result<(), MathLibError> {
    let equation = "eq(y=1-3x, x^2/4+y^2=1, x, y)";

    let res = quick_eval(equation, Context::empty())?.to_vec();
    let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Vector(vec![24./37., -35./37.]).round(3), Value::Vector(vec![0., 1.])]);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output1() -> Result<(), MathLibError> {
    use crate::svg_from_latex;

    let res = quick_eval("3*3+6^5", Context::empty())?;

    let svg = svg_from_latex(res.to_latex_at_var("H", false), "#FFFFFF")?;

    println!("{}", svg);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn output2() -> Result<(), MathLibError> {
    use crate::png_from_latex;
    use std::fs;

    let res = quick_eval("3*3+6^5", Context::empty())?;

    let png = png_from_latex(res.to_latex_at_var("H", false), 2.0, "#FFFFFF")?;

    let _ = fs::write("./test.png", png);

    Ok(())
}
