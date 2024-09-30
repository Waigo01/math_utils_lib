# Math utilities library

[![crates.io](https://img.shields.io/badge/crates.io-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/math_utils_lib)
[![docs.rs](https://img.shields.io/badge/docs.rs-gray?style=for-the-badge&logo=docs.rs)](https://docs.rs/math_utils_lib/latest/math_utils_lib/)

This repo/crate provides a number of math utilities:

- Parsing and evaluating expressions containing a combination of matrices, vectors and scalars.
- Solving equations and system of equations (both linear and non-linear).
- Exporting a LaTeX document from a collection of parsed and evaluated expressions.

:warning: This repo/crate has not hit 1.0.0 yet, breaking changes are bound to happen!

## Major features

- Parsing and evaluating calculations with matrices, vectors and scalars.
- A recursive parsing implementation allowing for calculations withing matrices and vectors.
- An inbuilt equation solver for solving linear and non-linear systems of equations, accessible through a custom "function".
- An evaluator based on combinatorics for combining multiple results from equations or sqrt with other operations.
- Inbuilt quality of life functions for exporting results to latex.

## Usage

*For usage information and more examples, please see [the wiki](https://github.com/Waigo01/math_utils_lib/wiki).*

## Examples
```rust
let res = quick_eval("3*3", &Context::empty())?.to_vec();
    
assert_eq!(res[0], Value::Scalar(9.));
```

```rust
let x = Variable::new("x", vec![Value::Scalar(3.)]);
let res = quick_eval("3x", &Context::from_vars(vec![x]))?.to_vec();

assert_eq!(res[0], Value::Scalar(9.));
```

```rust
let res = quick_eval("[[3, 4, 5], [1, 2, 3], [5, 6, 7]]", &Context::empty())?.to_vec();

assert_eq!(res[0], Value::Matrix(vec![vec![3., 1., 5.], vec![4., 2., 6.], vec![5., 3., 7.]]));
```

```rust
let function = parse("5x^2+2x+x")?;
let function_var = Function::new("f", function, vec!["x"]);

let res = quick_eval("f(5)", &Context::from_funs(vec![function_var]))?.to_vec();

assert_eq!(res[0], Value::Scalar(140.));
```

```rust
let res = quick_eval("eq(x^2=9, x)", &Context::empty())?.round(3).to_vec();
    
assert_eq!(res, vec![Value::Scalar(-3.), Value::Scalar(3.)]);
```

```rust
let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";

let res = quick_eval(equation, &Context::empty())?.round(3).to_vec();

assert_eq!(res, vec![Value::Vector(vec![3., -8., -2.])]);
```

```rust
let parsed_expr = parse("3*3+6^5")?;
let res = eval(&parsed_expr, &Context::empty())?;

let step = Step::Calc { term: parsed_expr, result: res, variable_save: Some("x".to_string()) };

let png = png_from_latex(step.as_latex_inline(), 200, "#FFFFFF")?;
```

Output:

![For proper render visit github](./images/test.png)

## TODO

- [x] Support for vectors and matrices
- [x] Calculations in vectors and matrices
- [x] Equations as operators -> eval can handle multiple values
- [ ] Complex numbers
- [ ] Possible tensor support
- [ ] Stable API that makes everyone happy (very hard)

## Issues and Contributions

When opening an issue, please specify the following:

- The mathematical expression that causes the issue
- The error, be it a MathLibError or any other kind of error
- The expected behavior

When it comes to contributions you are free to fork this repo and open pull requests.
