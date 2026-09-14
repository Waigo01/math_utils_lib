use crate::{Context, MathLibError, quick_eval, value};

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

#[test]
fn hard_integral() -> Result<(), MathLibError> {
    let res = quick_eval!("1/sqrt(2*250^2*pi)*I(e^(-(x-4000)^2/(2*250^2)), x, 3500, 4500)")?.to_vec();

    assert_eq!(res[0].round(8), value!(0.95449974));

    Ok(())
}

#[test]
fn infinite_integral() -> Result<(), MathLibError> {
    let res = quick_eval!("I(1/2^x, x, 0, inf)")?.to_vec();

    assert_eq!(res[0].round(8), value!(1.44269504));

    Ok(())
}

#[test]
fn sin_integral() -> Result<(), MathLibError> {
    let res = quick_eval!("I(sin(x), x, -inf, inf)")?.to_vec();

    assert_eq!(res[0].round(8), value!(0));

    Ok(())
}

#[test]
fn taylor() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("e(x) = S(x^n/fact(n), n, 0, 100)", &mut c)?;

    let res = quick_eval!("e(5) == exp(5)", &mut c)?.to_vec();

    assert_eq!(res[0], value!(1));

    Ok(())
}
