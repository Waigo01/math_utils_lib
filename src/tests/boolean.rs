use crate::{Context, MathLibError, quick_eval, value};

#[test]
fn boolean_equals() -> Result<(), MathLibError> {
    let res = quick_eval!("6==6")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_not() -> Result<(), MathLibError> {
    let res = quick_eval!("!(10==6)")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_or() -> Result<(), MathLibError> {
    let res = quick_eval!("6==10 | 10==10")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_and() -> Result<(), MathLibError> {
    let res = quick_eval!("6==6 & 10")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_not_eq() -> Result<(), MathLibError> {
    let res = quick_eval!("6!=10 & 10==10")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general1() -> Result<(), MathLibError> {
    let res = quick_eval!("6==10 | 10!=10")?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general2() -> Result<(), MathLibError> {
    let res = quick_eval!("6==6 & 6!=10")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general3() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval!("a = 6", &mut c)?;
    quick_eval!("b = 10", &mut c)?;

    let res = quick_eval!("!(a!=10 | b!=10)", &mut c)?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general4() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval!("a = 6", &mut c)?;
    quick_eval!("b = 10", &mut c)?;

    let res = quick_eval!("a!=6 & b==10", &mut c)?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn boolean_general5() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval!("a = 6", &mut c)?;
    quick_eval!("b = 10", &mut c)?;

    let res = quick_eval!("!(a != 10 | b != 10) | b != 5", &mut c)?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_equation() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(x^2 = 9, x) == -3")?.to_vec();

    assert_eq!(res, vec![value!(1), value!(0)]);

    Ok(())
}

#[test]
fn boolean_less() -> Result<(), MathLibError> {
    let res = quick_eval!("2<3")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_less_eq() -> Result<(), MathLibError> {
    let res = quick_eval!("(2<3 & 3<2) | 3<=3")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn boolean_general6() -> Result<(), MathLibError> {
    let res = quick_eval!("abs(eq(x^2 = 0, x)) < 10^(-4)")?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}

#[test]
fn if_statement1() -> Result<(), MathLibError> {
    let res = quick_eval!("if([2, 3, 4]==[2, 3, 4], 5, 2)")?.to_vec();

    assert_eq!(res[0], value!(5));

    Ok(())
}

#[test]
fn if_statement2() -> Result<(), MathLibError> {
    let res = quick_eval!("if(eq(x^2 = 9, x) == -3, 5, 2)")?.to_vec();

    assert_eq!(res, vec![value!(5), value!(2)]);

    Ok(())
}

#[test]
fn if_statement3() -> Result<(), MathLibError> {
    let res = quick_eval!("if({1, 2}==1, 2)")?.to_vec();

    assert_eq!(res, vec![value!(2)]);

    Ok(())
}

#[cfg(feature = "output")]
#[test]
fn if_statement_export() -> Result<(), MathLibError> {
    use crate::{eval, png_from_latex, Step, AST};

    let parsed_expr: AST<f64> = parse("f(x) =
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
    quick_eval!("state = 0", &mut c)?;

    quick_eval!("f(x) =
    if(x == 0 & state == 0,
        state = 1,
    if(x == 1 & state == 1,
        state = 2,
    if(x == 2 & state == 2,
        state = 3,
    if(x == 3 & state == 3,
        state = 4,
    state = 0))))", &mut c)?;

    quick_eval!("f({0, 1, 2, 3})", &mut c)?;

    let res = quick_eval!("state", &mut c)?.to_vec();

    assert_eq!(res[0], value!(4));
    
    quick_eval!("state = 0", &mut c)?;

    quick_eval!("f({0, 1, 4, 2})", &mut c)?.to_vec();
    
    let res = quick_eval!("state", &mut c)?.to_vec();

    assert_ne!(res[0], value!(4));

    Ok(())
}

#[test]
fn derivative() -> Result<(), MathLibError> {
    let res = quick_eval!("D(x^2, x, 3)")?.to_vec();

    assert_eq!(res[0].round(6), value!(6));

    Ok(())
}

#[test]
fn integral() -> Result<(), MathLibError> {
    let res = quick_eval!("I(x^2, x, 0, 5)")?.to_vec();

    assert_eq!(res[0].round(4), value!(41.6667));

    Ok(())
}
