use crate::{Complex, MathLibError, quick_eval, value};

#[test]
fn complex_mult() -> Result<(), MathLibError> {
    let res = quick_eval!("(3+2i)*(4+2i)"; Complex<f64>)?.to_vec();

    assert_eq!(res[0], value!(Complex::new(8., 14.)));

    Ok(())
}

#[test]
fn complex_addition() -> Result<(), MathLibError> {
    let res = quick_eval!("(3+2i)+(4+2i)"; Complex<f64>)?.to_vec();

    assert_eq!(res[0], value!(Complex::new(7., 4.)));

    Ok(())
}

#[test]
fn euler() -> Result<(), MathLibError> {
    let res = quick_eval!("e^(i*pi)"; Complex<f64>)?.to_vec();

    assert_eq!(res[0], value!(-1));

    Ok(())
}

#[test]
fn complex_sqrt() -> Result<(), MathLibError> {
    let res = quick_eval!("sqrt(3+2i)"; Complex<f64>)?.round(4).to_vec();

    assert_eq!(res[0], value!(Complex::new(1.8174, 0.5503)));

    Ok(())
}

#[test]
fn complex_solve1() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(x^2=-9, x)"; Complex<f64>)?.round(4).to_vec();

    assert_eq!(res, vec![value!(Complex::new(0., -3.)), value!(Complex::new(0., 3.))]);

    Ok(())
}

#[test]
fn complex_solve2() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(5x^2-8x+5=0, x)"; Complex<f64>)?.round(4).to_vec();

    assert_eq!(res, vec![value!(Complex::new(0.8, -0.6)), value!(Complex::new(0.8, 0.6))]);

    Ok(())
}
