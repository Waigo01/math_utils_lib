[![crates.io](https://img.shields.io/badge/crates.io-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/math_utils_lib)
[![docs.rs](https://img.shields.io/badge/docs.rs-gray?style=for-the-badge&logo=docs.rs)](https://docs.rs/math_utils_lib/latest/math_utils_lib/)

This repo/crate provides a number of math utilities:

- parsing and evaluating expressions containing matrices (column major order) and vectors
- solving equations and systems of equations
- exporting a LaTeX document from a collection of parsed expressions or solved equations

Precision:

Use feature high-prec for a precision of 13, standard precision is 8. The precision is used to solve equations. The actual output precision for printing is the defined precision - 2.

:warning: This repo/crate has not hit 1.0.0 yet, breaking changes are bound to happen!

# Examples
## Evaluations:
```rust
let res = quick_eval("3*3".to_string(), vec![])?;

assert_eq!(res, Value::Scalar(9.));
```

```rust
let x = Variable::new("x".to_string(), Value::Scalar(3.));
let res = quick_eval("3x".to_string(), vec![x])?;

assert_eq!(res, Value::Scalar(9.));
```

```rust
let a = Variable::new("A".to_string(), Value::Vector(vec![3., 5., 8.]));
let res = quick_eval("3A".to_string(), vec![a])?;

assert_eq!(res, Value::Vector(vec![9., 15., 24.]));
```

```rust
let a = Variable::new("A".to_string(), Value::Vector(vec![3., 5., 8.]));
let b = Variable::new("B".to_string(), Value::Matrix(vec![vec![2., 0., 0.], vec![0., 2., 0.],
vec![0., 0., 1.]]));
let res = quick_eval("B*A".to_string(), vec![a, b])?;

assert_eq!(res, Value::Vector(vec![6., 10., 8.]));
```
## Equations:
```rust
let equation = "x^2=9".to_string();

let res = quick_solve(equation, "x".to_string(), vec![])?;
let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

assert_eq!(res_rounded, vec![Value::Scalar(3.), Value::Scalar(-3.)]);
```

```rust
let equation = "400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k".to_string();

let res = quick_solve(equation, vec![])?;
let res_rounded = res.iter().map(|x| x.round(3)).collect::<Vec<Value>>();

assert_eq!(res_rounded, vec![Value::Vector(vec![4., 6.])]);
```
## LaTeX:
```rust
let expression = "((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))".to_string();
let parsed = parse(expression)?;
let vars = vec![Variable::new("x".to_string(), Value::Scalar(-0.655639))];
let result = eval(&parsed, &vars)?;
let var_assign = StepType::Calc((Binary::Value(Value::Scalar(-0.655639)), Value::Scalar(-0.655639), Some("x".to_string())));
let step = StepType::Calc((parsed, result, None));
export(vec![var_assign, step], "export".to_string(), ExportType::Png);
```

Output (export-1.png):

![For proper render visit github](./images/export-1.png)
