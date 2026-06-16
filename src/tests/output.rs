use crate::{MathLibError, Variable, quick_eval};

#[test]
fn variable_as_latex() -> Result<(), MathLibError> {
    let res1 = quick_eval!("{2, 3, 4}")?;
    let res2 = quick_eval!("f(x) = x^2")?;
    let res3 = quick_eval!("4")?;

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

    let mut c: Context<f64> = Context::empty();

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
//
// #[cfg(feature = "output")]
// #[test]
// fn output2() -> Result<(), MathLibError> {
//     use crate::{eval, png_from_latex, Step, AST};
//     use std::fs;
//
//     let parsed_expr: AST<f64> = parse("x = 3*3+6^5")?;
//     let res = eval(&parsed_expr, &mut Context::empty())?;
//
//     let step = Step::new(parsed_expr, res);
//
//     let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;
//
//     let _ = fs::write("./images/test.png", png);
//
//     Ok(())
// }
