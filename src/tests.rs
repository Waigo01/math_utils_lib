use std::fs;

use crate::{errors::{MathLibError, ParserError, QuickEvalError}, eval, export, parse, parser::Binary, quick_eval, quick_solve, ExportType, StepType, Value, Variable};

#[test]
fn easy_eval1() -> Result<(), MathLibError> {
    let res = quick_eval("3*3".to_string(), vec![])?;
    assert_eq!(res, Value::Scalar(9.));

    Ok(())
}

#[test]
fn medium_eval1() -> Result<(), MathLibError> {
    let x = Variable {
        name: "x".to_string(),
        value: Value::Scalar(3.)
    };
    let res = quick_eval("3x".to_string(), vec![x])?;

    assert_eq!(res, Value::Scalar(9.));
    Ok(())
}

#[test]
fn medium_eval2() -> Result<(), MathLibError> {
    let a = Variable {
        name: "A".to_string(),
        value: Value::Vector(vec![3., 5., 8.])
    };
    let res = quick_eval("3A".to_string(), vec![a])?;

    assert_eq!(res, Value::Vector(vec![9., 15., 24.]));

    Ok(())
}

#[test]
fn medium_eval3() -> Result<(), MathLibError> {
    let a = Variable {
        name: "A".to_string(),
        value: Value::Vector(vec![3., 5., 8.])
    };
    let b = Variable {
        name: "B".to_string(),
        value: Value::Matrix(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 1.]])
    };
    let res = quick_eval("B*A".to_string(), vec![a, b])?;

    assert_eq!(res, Value::Vector(vec![6., 10., 8.]));

    Ok(())
}

#[test]
fn medium_eval4() -> Result<(), MathLibError> {
    let a = Variable {
        name: "A".to_string(),
        value: Value::Matrix(vec![vec![3., 5., 7.], vec![4., 8., 2.], vec![1., 9., 2.]])
    };
    let b = Variable {
        name: "B".to_string(),
        value: Value::Matrix(vec![vec![7., 9., 10.], vec![1., 55., 8.], vec![22., 9., 2.]])
    };
    let res = quick_eval("A*B".to_string(), vec![a, b])?;

    assert_eq!(res, Value::Matrix(vec![vec![180., 365., 84.], vec![80., 494., 108.], vec![60., 522., 86.]]));

    Ok(())
}

#[test]
fn medium_eval5() -> Result<(), MathLibError> {
    let res = quick_eval("((3*3))".to_string(), vec![])?;

    assert_eq!(res, Value::Scalar(9.));

    Ok(())
}

#[test]
fn medium_eval6() {
    let res = quick_eval("[[3, 0, 5], [2, 4, 5], [1, 2]]".to_string(), vec![]);

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn medium_eval7() {
    let res = quick_eval("[[], [], []]".to_string(), vec![]);

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn medium_eval8() {
    let res = quick_eval("".to_string(), vec![]);

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyExpr))
}

#[test]
fn medium_eval9() {
    let res = quick_eval("[[3, 0,], [2, 4, 5], [1, 2]]".to_string(), vec![]);

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::EmptyVec))
}

#[test]
fn medium_eval10() {
    let res = quick_eval("[[3, 0, 5], [2, 4], [1, 2,]]".to_string(), vec![]);

    assert_eq!(res.unwrap_err(), QuickEvalError::ParserError(ParserError::NotRectMatrix))
}

#[test]
fn hard_eval1() -> Result<(), MathLibError> {
    let x = Variable {
        name: "x".to_string(),
        value: Value::Scalar(3.)
    };
    let a = Variable {
        name: "A".to_string(),
        value: Value::Vector(vec![3., 2., 1.])
    };
    let b = Variable {
        name: "B".to_string(),
        value: Value::Matrix(vec![vec![2., 3., 4.], vec![5., 1., 7.], vec![2., 3., 6.]])
    };
    let res = quick_eval("x*B*A?1".to_string(), vec![x, a, b])?;

    assert_eq!(res, Value::Scalar(72.));

    Ok(())
}

#[test]
fn easy_solve1() -> Result<(), MathLibError> {
    let equation = "x^2=9".to_string();

    let res = quick_solve(equation, "x".to_string(), vec![])?;
    
    let res_rounded = res.iter().map(|x| Value::Scalar((x.get_scalar()*1000.).round()/1000.)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Scalar(-3.), Value::Scalar(3.)]);

    Ok(())
}

#[test]
fn medium_solve1() -> Result<(), MathLibError> {
    let equation = "3x^2+2x-1=0".to_string();

    let res = quick_solve(equation, "x".to_string(), vec![])?;
    
    let res_rounded = res.iter().map(|x| Value::Scalar((x.get_scalar()*1000.).round()/1000.)).collect::<Vec<Value>>();

    assert_eq!(res_rounded, vec![Value::Scalar(-1.), Value::Scalar(((1./3.) as f64*1000.).round()/1000.)]);

    Ok(())
}

#[test]
fn hard_solve1() -> Result<(), MathLibError> {
    let equation = "((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0".to_string();

    let res = quick_solve(equation, "x".to_string(), vec![])?;

    let res_rounded = res.iter().map(|x| Value::Scalar((x.get_scalar()*1000.).round()/1000.)).collect::<Vec<Value>>();

    println!("{:.?}", res_rounded);

    assert_eq!(res_rounded.contains(&Value::Scalar(-0.656)), true);

    Ok(())
}

#[test]
fn hard_latex1() -> Result<(), MathLibError> {
    let expression = "((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))".to_string();
    let parsed = parse(expression)?;
    let vars = vec![Variable {
        name: "x".to_string(),
        value: Value::Scalar(-0.655639)
    }];
    let result = eval(&parsed, &vars)?;
    let var_assign = StepType::Calc((Binary::Value(Value::Scalar(-0.655639)), Value::Scalar(-0.655639), Some("x".to_string())));
    let step = StepType::Calc((parsed, result, None));

    export(vec![var_assign, step], "export".to_string(), ExportType::Png);

    let _ = fs::remove_file("./export-1.png");

    Ok(())
}
