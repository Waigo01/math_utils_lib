use crate::{MathLibError, quick_eval, value};

#[test]
fn hard_integral() -> Result<(), MathLibError> {
    let res = quick_eval!("1/sqrt(2*250^2*pi)*I(e^(-(x-4000)^2/(2*250^2)), x, 3500, 4500)")?.to_vec();

    assert_eq!(res[0].round(4), value!(0.9545));

    Ok(())
}
