use std::time::Duration;

use crate::{Context, MathLibError, quick_eval_async};

#[tokio::test]
async fn async_timeout() -> Result<(), MathLibError> {
    let mut c = Context::empty();

    quick_eval_async!("fib(x) = if(x==0, 0, if(x==1, 1, fib(x-1) + fib(x-2)))", &mut c).await?;

    let res = tokio::time::timeout(Duration::from_secs(1), quick_eval_async!("fib(100)", &mut c)).await;

    assert_eq!(res.is_err(), true);

    Ok(())
}
