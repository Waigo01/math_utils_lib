use crate::{helpers::{center_in_string, flatten_conditional, round_and_format}, maths::num_traits::Number};

#[doc(hidden)]
const VAR_SYMBOLS: [(&str, &str); 48] = [("\\alpha", "𝛼"), ("\\Alpha", "𝛢"), ("\\beta", "𝛽"), ("\\Beta", "𝛣"), ("\\gamma", "𝛾"), ("\\Gamma", "𝚪"),
("\\delta", "𝛿"), ("\\Delta", "𝛥"), ("\\epsilon", "𝜺"), ("\\Epsilon", "𝛦"), ("\\zeta", "𝜁"), ("\\Zeta", "𝛧"), ("\\eta", "𝜂"), ("\\Eta", "𝛨"),
("\\theta", "𝜃"), ("\\Theta", "𝛩"), ("\\iota", "𝜄"), ("\\Iota", "𝛪"), ("\\kappa", "𝜅"), ("\\Kappa", "𝛫"), ("\\lambda", "𝜆"), ("\\Lambda", "𝛬"),
("\\mu", "𝜇"), ("\\Mu", "𝛭"), ("\\nu", "𝜈"), ("\\Nu", "𝛮"), ("\\xi", "𝜉"), ("\\Xi", "𝛯"), ("\\omicron", "𝜊"), ("\\Omicron", "𝛰"), ("pi", "𝜋"),
("\\Pi", "𝛱"), ("\\rho", "𝜌"), ("\\Rho", "𝛲"), ("\\sigma", "𝜎"), ("\\Sigma", "𝛴"), ("\\tau", "𝜏"), ("\\Tau", "𝛵"), ("\\upsilon", "𝜐"),
("\\Upsilon", "𝛶"), ("\\phi", "𝜑"), ("\\Phi", "𝛷"), ("\\xi", "𝜒"), ("\\Xi", "𝛸"), ("\\psi", "𝜓"), ("\\Psi", "𝛹"), ("\\omega", "𝜔"), ("\\Omega", "𝛺")];

/// describes a Variable that can be used in the context of an evaluation. 
/// 
/// Variables in this implementation can contain multiple values, in order to make the storage of
/// results from equations easier.
/// 
/// Variable names are not allowed to contain numbers outside of LaTeX style subscript. Additionally
/// they must start with an alphabetical letter or a "\\".
/// 
/// # Example
/// 
/// ```
/// # use math_utils_lib::{Variable, value, Value};
/// let variable: Variable<f64> = Variable::new("x", vec![value!(3.)]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable<N: Number> {
    pub name: String,
    pub values: Values<N>
}

impl<N: Number> Variable<N> {
    /// creates a new variable from a name and associated values.
    pub fn new<S: Into<String>, V: Into<Values<N>>>(name: S, values: V) -> Self {
        Variable { name: name.into(), values: values.into()}
    }
    /// converts the variable to latex. The function also provides the option to add a "&" aligner before the
    /// ":=".
    pub fn as_latex(&self, add_aligner: bool) -> String {
        let right = AST::from_values(self.values.clone());

        let ast = AST::from_operation(Operation::SimpleOperation { op_type: SimpleOpType::Assign, left: AST::Variable(self.name.clone()), right});
        return if add_aligner {ast.as_latex()} else {ast.as_latex_inline()};
    }
    /// converts the variable to a string using basic string formatting.
    pub fn as_string(&self) -> String {
        format!("{} = {}", self.name, self.values.as_string())
    }
    /// converts the variable to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Variable<B> {
        Variable { name: self.name, values: self.values.into() }
    }
}

/// describes a function that can be used in the context of an evaluation.
///
/// Function names must follow the same criteria as [Variable] names.
///
/// # Example
///
/// ```
/// # use math_utils_lib::{parse, Function, MathLibError};
/// let parsed_expr = parse::<_, f64>("x^2")?;
/// let function = Function::new("f", parsed_expr, vec!["x"]);
/// # Ok::<(), MathLibError>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function<N: Number> {
    pub name: String,
    pub ast: AST<N>,
    pub inputs: Vec<String>
}

impl<N: Number> Function<N> {
    /// creates a new function from an [AST] (a parsed expression) and a Vec of input variable
    /// names.
    pub fn new<S: Into<String>>(name: S, ast: AST<N>, inputs: Vec<S>) -> Self {
        Function { name: name.into(), ast, inputs: inputs.into_iter().map(|s| s.into()).collect() }
    }
    /// converts the function to latex. The function also provides the option to add a "&" aligner before
    /// the "=".
    pub fn as_latex(&self, add_aligner: bool) -> String {
        let ast = AST::from_operation(Operation::SimpleOperation { op_type: SimpleOpType::Assign, left: AST::Function { name: self.name.clone(), inputs: self.inputs.iter().map(|i| AST::Variable(i.to_string())).collect() }, right: self.ast.clone() });
        return if add_aligner {ast.as_latex()} else {ast.as_latex_inline()};
    }
    /// converts the function to a string using basic string formatting.
    pub fn as_string(&self) -> String {
        format!("{}({}) = {}", self.name, self.inputs.join(", "), self.ast.as_string())
    }
    /// converts the function to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Function<B> {
        Function { name: self.name, ast: self.ast.into(), inputs: self.inputs }
    }
}

/// Provides a way to call an internal rust function from the evaluation context.
#[derive(Debug, Clone)]
pub struct InternalFunction<N: Number> {
    pub name: String,
    pub n_arguments: usize,
    pub function: fn(Vec<Value<N>>) -> Result<Value<N>, String>
}

impl<N: Number> InternalFunction<N> {
    /// creates a new InternalFunction based on the name of the function, the number of arguments of
    /// the function and the actual function that should be called.
    pub fn new<S: Into<String>>(name: S, n_args: usize, function: fn(Vec<Value<N>>) -> Result<Value<N>, String>) -> Self {
        InternalFunction { name: name.into(), n_arguments: n_args, function }
    }
}

/// combines [Variable]s, [Function]s and [InternalFunction]s into a convenient struct, which then gets passed to the
/// evaluator.
///
/// # Example
///
/// ```
/// # use math_utils_lib::Context;
/// let context: Context<f64> = Context::default();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Context<N: Number> {
    pub vars: Vec<Variable<N>>,
    pub funs: Vec<Function<N>>,
    #[cfg_attr(feature = "serde", serde(skip, default = "N::default_functions"))]
    pub internal_funs: Vec<InternalFunction<N>>
}

impl<N: Number> Context<N> {
    /// creates a new context with variables specified by
    /// [Number::default_vars()](crate::Number::default_vars()). And functions specified by
    /// [Number::default_functions()](crate::Number::default_functions()).
    ///
    /// When using [f64] default vars are pi and e. When using [Complex](crate::Complex) default
    /// vars are pi, e and i.
    ///
    /// Both [f64] and [Complex](crate::Complex) provide the same default functions. Take a look
    /// at [StandardFunctions::default_functions()](crate::StandardFunctions::default_functions()).
    pub fn default() -> Self {
        return Context::new(N::default_vars(), vec![]);
    }
    /// creates a context containing only the given variables and functions.
    pub fn new<V: AsRef<[Variable<N>]>, F: AsRef<[Function<N>]>>(vars: V, funs: F) -> Context<N> {
        Context {vars: vars.as_ref().to_vec(), funs: funs.as_ref().to_vec(), internal_funs: N::default_functions()}
    }
    /// creates a context containing the given variables and functions in addition to the default
    /// variables and functions.
    pub fn with<V: AsRef<[Variable<N>]>, F: AsRef<[Function<N>]>>(vars: V, funs: F) -> Context<N> {
        let mut c = Context::default();
        for var in vars.as_ref() {
            c.add_var(var);
        }
        for fun in funs.as_ref() {
            c.add_fun(fun);
        }
        c
    }
    /// creates an empty context.
    pub fn empty() -> Context<N> {
        Context { vars: vec![], funs: vec![], internal_funs: vec![] }
    }
    /// creates a new context containing only the given variables.
    pub fn from_vars<V: AsRef<[Variable<N>]>>(vars: V) -> Context<N> {
        Context { vars: vars.as_ref().to_vec(), funs: vec![], internal_funs: vec![] }
    }
    /// creates a new context containing only the given functions.
    pub fn from_funs<F: AsRef<[Function<N>]>>(funs: F) -> Context<N> {
        Context { vars: vec![], funs: funs.as_ref().to_vec(), internal_funs: vec![] }
    }
    /// creates a new context containing the given variables in addition to the default variables
    /// and functions.
    pub fn with_vars<V: AsRef<[Variable<N>]>>(vars: V) -> Context<N> {
        let mut c = Context::default();
        for var in vars.as_ref() {
            c.add_var(var);
        }
        c
    }
    /// creates a new context containing the given functions in addition to the default variables
    /// and functions.
    pub fn with_funs<F: AsRef<[Function<N>]>>(funs: F) -> Context<N> {
        let mut c = Context::default();
        for fun in funs.as_ref() {
            c.add_fun(fun);
        }
        c
    }
    /// adds a variable to the context, replacing an already existing variable with the same name.
    pub fn add_var(&mut self, var: &Variable<N>) {
        self.vars = self.vars.iter()
            .filter(|v| v.name != var.name)
            .map(|v| v.to_owned())
            .collect();

        self.vars.push(var.to_owned());
    }
    /// adds a function to the context, replacing an already existing function with the same name.
    pub fn add_fun(&mut self, fun: &Function<N>) {
        self.funs = self.funs.iter()
            .filter(|f| f.name != fun.name)
            .map(|f| f.to_owned())
            .collect();

        self.funs.push(fun.to_owned());
    }
    /// removes all variables with the given variable name.
    pub fn remove_var<S: Into<String> + Clone>(&mut self, var_name: S) {
        self.vars = self.vars.iter()
            .filter(|v| v.name != var_name.clone().into())
            .map(|v| v.to_owned())
            .collect();
    }
    /// removes all functions with the given variable name.
    pub fn remove_fun<S: Into<String> + Clone>(&mut self, fun_name: S) {
        self.funs = self.funs.iter()
            .filter(|f| f.name != fun_name.clone().into())
            .map(|f| f.to_owned())
            .collect()
    }
    /// returns the variable with the given name or None if it does not exist in the context.
    pub fn get_var<S: Into<String> + Clone>(&self, var_name: S) -> Option<Variable<N>> {
        self.vars.iter().filter(|v| v.name == var_name.clone().into()).map(|v| v.to_owned()).nth(0)
    }
    /// returns the function with the given name or None if it does not exist in the context.
    pub fn get_fun<S: Into<String> + Clone>(&self, fun_name: S) -> Option<Function<N>> {
        self.funs.iter().filter(|f| f.name == fun_name.clone().into()).map(|f| f.to_owned()).nth(0)
    }
    /// returns the internal_function with the given name or None if it does not exist in the context.
    pub fn get_internal_fun<S: Into<String> + Clone>(&self, fun_name: S) -> Option<InternalFunction<N>> {
        self.internal_funs.iter().filter(|f| f.name == fun_name.clone().into()).map(|f| f.to_owned()).nth(0)
    }
    /// converts the context to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Context<B> {
        Context::with(self.vars.into_iter().map(|v| v.into()).collect::<Vec<Variable<B>>>(), self.funs.into_iter().map(|f| f.into()).collect::<Vec<Function<B>>>())
    }
}

/// helps to quickly initialize a [Value].
///
/// Matrices are processed in a row-major fashion.
///
/// # Example
///
/// ```
/// # use math_utils_lib::{Value, value};
/// let x: Value<f64> = value!(3.5);
/// let y: Value<f64> = value!(3, 2, 1);
/// let z: Value<f64> = value!(1, 0, 0; 0, 1, 0; 0, 0, 1);
/// ```
#[macro_export]
macro_rules! value {
    ( $x:expr ) => {
        {
            $crate::Value::Scalar($x.into())
        }
    };
    ( $($x:expr),+ ) => {
        {
            let mut vector = Vec::new();
            $(
                vector.push($x.into());
            )*
            $crate::Value::Vector(vector)
        }
    };
    ( $($($x:expr),+);+ ) => {
        {
            let mut vector = Vec::new();
            $(
                let mut row = Vec::new();
                $(
                    row.push($x.into());
                )*
                vector.push(row);
            )*
            $crate::Value::Matrix(vector)
        }
    };
}

/// specifies a Value that can be a Matrix, Vector or a Scalar.
/// 
/// # Example
/// 
/// ```
/// # use math_utils_lib::Value;
/// let x: Value<f64> = Value::Scalar(3.5);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value<N> where N: Number {
    Matrix(Vec<Vec<N>>),
    Vector(Vec<N>),
    Scalar(N)
}

impl<N> Value<N> where N: Number {
    /// returns the scalar if the value is a scalar and None if it is a matrix or a
    /// vector.
    pub fn get_scalar(&self) -> Option<N> {
        match self {
            Value::Scalar(a) => return Some(*a),
            Value::Matrix(_) => return None,
            Value::Vector(_) => return None
        }
    }
    /// returns the vector if the value is a vector and None if it is a matrix or a
    /// scalar.
    pub fn get_vector(&self) -> Option<Vec<N>> {
        match self {
            Value::Vector(a) => return Some(a.to_vec()),
            Value::Matrix(_) => return None,
            Value::Scalar(_) => return None
        }
    }
    /// returns the matrix if the value is a matrix and None if it is a scalar or a
    /// vector.
    pub fn get_matrix(&self) -> Option<Vec<Vec<N>>> {
        match self {
            Value::Matrix(a) => return Some(a.to_vec()),
            Value::Scalar(_) => return None,
            Value::Vector(_) => return None
        }
    }
    /// returns true if the value is a scalar.
    pub fn is_scalar(&self) -> bool {
        match self {
            Value::Scalar(_) => return true,
            _ => return false
        }
    }
    /// returns true if the value is a vector.
    pub fn is_vector(&self) -> bool {
        match self {
            Value::Vector(_) => return true,
            _ => return false
        }
    }
    /// returns true if the value is a matrix.
    pub fn is_matrix(&self) -> bool {
        match self {
            Value::Matrix(_) => return true,
            _ => return false
        }
    }
    /// rounds the value.
    pub fn round(&self, prec: usize) -> Value<N> {
        match self {
            Value::Scalar(a) => return Value::Scalar((*a*N::base().powi(prec as i32)).round()/N::base().powi(prec as i32)),
            Value::Vector(v) => {
                let mut new_vec = vec![];
                for i in v {
                    new_vec.push((*i*N::base().powi(prec as i32)).round()/N::base().powi(prec as i32));
                }
                return Value::Vector(new_vec);
            },
            Value::Matrix(m) => {
                let mut new_matrix = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push((*j*N::base().powi(prec as i32)).round()/N::base().powi(prec as i32));
                    }
                    new_matrix.push(row);
                }
                return Value::Matrix(new_matrix);
            }
        }
    }
    /// rounds the value to the precision of the number type.
    pub fn round_to_precision(&self) -> Value<N> {
        match self {
            Value::Scalar(a) => return Value::Scalar((*a/N::epsilon()).round()*N::epsilon()),
            Value::Vector(v) => {
                let mut new_vec = vec![];
                for i in v {
                    new_vec.push((*i/N::epsilon()).round()*N::epsilon());
                }
                return Value::Vector(new_vec);
            },
            Value::Matrix(m) => {
                let mut new_matrix = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push((*j/N::epsilon()).round()*N::epsilon());
                    }
                    new_matrix.push(row);
                }
                return Value::Matrix(new_matrix);
            }
        }
    }
    /// rounds the value to the display precision of the number type.
    pub fn round_to_display_precision(&self) -> Value<N> {
        match self {
            Value::Scalar(a) => return Value::Scalar((*a/N::display_epsilon()).round()*N::display_epsilon()),
            Value::Vector(v) => {
                let mut new_vec = vec![];
                for i in v {
                    new_vec.push((*i/N::display_epsilon()).round()*N::display_epsilon());
                }
                return Value::Vector(new_vec);
            },
            Value::Matrix(m) => {
                let mut new_matrix = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push((*j/N::display_epsilon()).round()*N::display_epsilon());
                    }
                    new_matrix.push(row);
                }
                return Value::Matrix(new_matrix);
            }
        }
    }
    /// checks if any part of the value is infinite or NaN.
    pub fn is_inf_or_nan(&self) -> bool {
        match self {
            Value::Scalar(s) => {if s.is_infinite() || s.is_nan() {return true}},
            Value::Vector(v) => {
                for i in v {
                    if i.is_infinite() || i.is_nan() {
                        return true;
                    }
                }
            },
            Value::Matrix(m) => {
                for i in m {
                    for j in i {
                        if j.is_infinite() || j.is_nan() {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    /// provides a crude method to convert a value to a string, using square brackets
    /// for matrices and vectors.
    pub fn as_string(&self) -> String {
        let mut replace_string = String::new();
        match &self {
            Value::Matrix(s) => {
                replace_string += "[";
                for k in 0..s.len() {
                    replace_string += "[";
                    for l in 0..s[k].len() {
                        replace_string += &s[k][l].to_string();
                        if l != s[k].len() - 1 {
                            replace_string += ", "
                        }
                    }
                    replace_string += "]";
                    if k != s.len() - 1 {
                        replace_string += ", ";
                    }
                }
                replace_string += "]";
            },
            Value::Vector(s) => {
                replace_string += "[";
                for k in 0..s.len() {
                    replace_string += &s[k].to_string();
                    if k != s.len() - 1 {
                        replace_string += ", ";
                    }    
                }
                replace_string += "]";
            },
            Value::Scalar(s) => {
                replace_string = s.to_string();
            }
        }

        return replace_string
    }
    #[deprecated(since="0.4.0", note="Because of the complexity of Value, Values and ASTs this function can still be used to convert a single Value but will not be implemented for ASTs or Values in the forseeable future.")]
    /// converts the given value to unicode, using unicode symbols for vectors and matrices.
    pub fn as_unicode(&self) -> String {
        self.pretty_print(None)
    }
    #[deprecated(since="0.4.0", note="Because of the complexity of Value, Values and ASTs this function can still be used to convert a single Value but will not be implemented for ASTs or Values in the forseeable future.")]
    /// converts the given value to unicode, same as [as_unicode](Value::as_unicode) but with a variable name in
    /// front of the value.
    pub fn as_unicode_at_var<S: Into<String>>(&self, var_name: S) -> String {
        let mut var_name_string = var_name.into();
        for i in VAR_SYMBOLS {
            if var_name_string == i.0 {
                var_name_string = i.1.to_string();
                break;
            }
        }
        self.pretty_print(Some(var_name_string))
    }
    fn pretty_print(&self, var_name: Option<String>) -> String {
        match self {
            Value::Scalar(s) => {
                let mut output_buffer = String::new();
                if var_name.is_some() {
                    output_buffer += &format!("{} = ", var_name.unwrap())
                }
                output_buffer += &round_and_format(*s, false);
                return output_buffer;
            },
            Value::Vector(v) => {
                let mut rounded_v: Vec<String> = vec![];
                for i in 0..v.len() {
                    rounded_v.push(round_and_format(v[i], false));
                }
                let max_width = rounded_v.iter().map(|x| x.len()).max().unwrap();
                let v_middle = ((rounded_v.len() as f64/2.).ceil()-1.) as i32;
                let mut whitespace_n = 0;
                if var_name.is_some() {
                    whitespace_n = format!("{} = ", var_name.clone().unwrap()).len();
                }
                let mut output_buffer = String::new();
                for i in 0..rounded_v.len() {
                    let mut output_line_buffer = String::new();
                    if i == v_middle as usize && var_name.is_some() {
                        output_line_buffer += &format!("{} = ", var_name.clone().unwrap());
                    } else {
                        for _ in 0..whitespace_n {
                            output_line_buffer += " ";
                        }
                    }

                    if i == 0 {
                        output_line_buffer += "⎛";
                    } else if i == rounded_v.len()-1 {
                        output_line_buffer += "⎝";
                    } else {
                        output_line_buffer += "⎜";
                    }

                    output_line_buffer += &center_in_string(rounded_v[i].clone(), max_width as i32);

                    if i == 0 {
                        output_line_buffer += "⎞";
                    } else if i == rounded_v.len()-1 {
                        output_line_buffer += "⎠";
                    } else {
                        output_line_buffer += "⎟";
                    }

                    if i != rounded_v.len()-1{
                        output_line_buffer += "\n";
                    }
                    output_buffer += &output_line_buffer;
                }

                return output_buffer
            },
            Value::Matrix(m) => {
                let mut rounded_m: Vec<Vec<String>> = vec![];
                for i in 0..m.len() {
                    let mut row = vec![];
                    for j in 0..m[0].len() {
                        row.push(round_and_format(m[i][j], false));
                    }
                    rounded_m.push(row);
                }
                let max_width = rounded_m.iter().map(|r| r.iter().map(|x| x.to_string().len()).max().unwrap()).max().unwrap();
                let v_middle = ((rounded_m.len() as f64/2.).ceil()-1.) as i32;
                let mut whitespace_n = 0;
                if var_name.is_some() {
                    whitespace_n = format!("{} = ", var_name.clone().unwrap()).len();
                }
                let mut output_buffer = String::new();
                for i in 0..rounded_m.len() {
                    let mut output_line_buffer = String::new();
                    if i == v_middle as usize && var_name.is_some() {
                        output_line_buffer += &format!("{} = ", var_name.clone().unwrap());
                    } else {
                        for _ in 0..whitespace_n {
                            output_line_buffer += " ";
                        }
                    }

                    if i == 0 {
                        output_line_buffer += "⎡";
                    } else if i == rounded_m.len()-1 {
                        output_line_buffer += "⎣";
                    } else {
                        output_line_buffer += "⎢";
                    }

                    for j in 0..rounded_m[i].len() {
                        if j != rounded_m[i].len() - 1 {
                            output_line_buffer += &(center_in_string(rounded_m[i][j].clone(), max_width as i32) + " ");
                        } else {
                            output_line_buffer += &center_in_string(rounded_m[i][j].clone(), max_width as i32);
                        }
                    }

                    if i == 0 {
                        output_line_buffer += "⎤";
                    } else if i == rounded_m.len()-1 {
                        output_line_buffer += "⎦";
                    } else {
                        output_line_buffer += "⎥";
                    }

                    if i != rounded_m.len()-1{
                        output_line_buffer += "\n";
                    }
                    output_buffer += &output_line_buffer;
                }

                return output_buffer
            }
        }
    }
    /// converts the value to a latex expression using amsmath's p and bmatrix.
    pub fn as_latex(&self) -> String {
        self.latex_print()
    }
    fn latex_print(&self) -> String {
        match self {
            Value::Scalar(s) => return round_and_format(*s, true),
            Value::Vector(v) => {
                let mut output_string = "\\begin{pmatrix}".to_string();
                for i in 0..v.len() {
                    if i != v.len()-1 {
                        output_string += &format!("{}\\\\ ", round_and_format(v[i], true));
                    } else {
                        output_string += &round_and_format(v[i], true);
                    }
                }
                output_string += "\\end{pmatrix}";
                return output_string
            },
            Value::Matrix(m) => {
                let mut output_string = "\\begin{bmatrix}".to_string();
                for i in 0..m.len(){
                    let mut row_string = "".to_string();
                    for j in 0..m[i].len() {
                        if j != m[i].len()-1 {
                            row_string += &format!("{} & ", round_and_format(m[i][j], true));
                        } else {
                            row_string += &format!("{} \\\\", round_and_format(m[i][j], true));
                        }
                    }
                    output_string += &row_string;
                }
                output_string += "\\end{bmatrix}";
                return output_string;
            }
        }
    }
    /// converts the value to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Value<B> {
        match self {
            Self::Scalar(s) => Value::<B>::Scalar(s.into()),
            Self::Vector(v) => Value::<B>::Vector(v.into_iter().map(|s| s.into()).collect()),
            Self::Matrix(m) => Value::<B>::Matrix(m.into_iter().map(|v| v.into_iter().map(|s| s.into()).collect()).collect())
        }
    }
}

impl<N: Number> Into<Values<N>> for Value<N> {
    fn into(self) -> Values<N> {
        return Values::from_vec(vec![self]);
    }
}

impl<N: Number> Into<Values<N>> for Vec<Value<N>> {
    fn into(self) -> Values<N> {
        return Values::from_vec(self);
    }
}

/// provides a wrapper around Vec of Value with some quality of life implementations.
///
/// # Example
///
/// ```
/// # use math_utils_lib::{Values, Value};
/// let values = Values::from_vec(vec![Value::Scalar(3.)]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Values<N: Number>(Vec<Value<N>>);

impl<N: Number> Values<N> {
    /// creates the values from a Vec of [Value].
    pub fn from_vec<V: AsRef<[Value<N>]>>(values: V) -> Self {
        return Values(values.as_ref().to_vec());
    }
    /// converts the values back to a Vec of [Value].
    pub fn to_vec(self) -> Vec<Value<N>> {
        return self.0;
    }
    /// gets the [Value] at the given index.
    pub fn get(&self, i: usize) -> Option<&Value<N>> {
        self.0.iter().nth(i)
    }
    /// returns the amount of values.
    pub fn len(&self) -> usize {
        return self.0.len()
    }
    /// rounds all values.
    pub fn round(&self, prec: usize) -> Values<N> {
        let rounded_vals = self.0.iter().map(|x| x.round(prec)).collect::<Vec<Value<N>>>();
        Values::from_vec(rounded_vals)
    }
    /// converts the values to a string using "{}" and "," to print multiple Values. This is a crude
    /// way to convert [Values] as it uses [Value::as_string].
    pub fn as_string(&self) -> String {
        if self.len() == 1 {
            return self.get(0).unwrap().as_string();
        } else {
            return format!("{{{}}}", self.0.iter().map(|v| v.as_string()).collect::<Vec<String>>().join(", "));
        }
    }
    /// converts the values to latex using "{}" and "," to print multiple Values.
    pub fn as_latex(&self) -> String {
        if self.len() == 1 {
            return format!("{}", self.0[0].as_latex());
        } else if self.len() <= 0 {
            return "No solutions".to_string();
        } else {
            return format!("\\left\\{{{}\\right\\}}", self.clone().to_vec().iter().map(|v| v.as_latex()).collect::<Vec<String>>().join(", "));
        }
    }
    /// converts the values to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Values<B> {
        self.to_vec().into_iter().map(|v| v.into()).collect::<Vec<Value<B>>>().into()
    }
}

/// used to construct an AST which is recursively evaluated by the [eval](crate::evaluator::eval) function.
/// 
/// Each node of the AST can be a:
/// 
/// - Scalar
/// - Vector
/// - Matrix
/// - List
/// - Variable
/// - Function
/// - Operation
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AST<N: Number> {
    Scalar(N),
    Vector(Vec<AST<N>>),
    Matrix(Vec<Vec<AST<N>>>),
    List(Vec<AST<N>>),
    Variable(String),
    Function {
        name: String,
        inputs: Vec<AST<N>>
    },
    Operation(Box<Operation<N>>),
}

impl<N: Number> AST<N> {
    /// creates an AST node from a [Value].
    pub fn from_value(val: Value<N>) -> AST<N> {
        match val {
            Value::Scalar(s) => return AST::Scalar(s),
            Value::Vector(v) => {
                let mut parsed_values = vec![];
                for i in v {
                    parsed_values.push(AST::Scalar(i))
                }
                return AST::Vector(parsed_values)
            },
            Value::Matrix(m) => {
                let mut parsed_rows = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push(AST::Scalar(j))
                    }
                    parsed_rows.push(row);
                }
                return AST::Matrix(parsed_rows);
            }
        }
    }
    /// creates an AST node from [Values].
    pub fn from_values(vals: Values<N>) -> AST<N> {
        if vals.len() == 1 {
            return AST::from_value(vals.get(0).unwrap().clone());
        } else if vals.len() == 0 {
            return AST::List(vec![]);
        } else {
            let mut ast_vals = vec![];
            for val in vals.to_vec() {
                ast_vals.push(AST::from_value(val));
            }
            return AST::List(ast_vals);
        }
    }
    /// creates an AST node from a variable name.
    pub fn from_variable_name<S: Into<String>>(val: S) -> AST<N> {
        return AST::Variable(val.into());
    }
    /// creates an AST node from an operation.
    pub fn from_operation(val: Operation<N>) -> AST<N> {
        return AST::Operation(Box::new(val));
    }
    /// converts the AST to a string using crude symbols for operations, vectors and matrices.
    pub fn as_string(&self) -> String {
        match self {
            AST::Scalar(s) => return round_and_format(*s, false),
            AST::Vector(v) => return format!("[{}]", v.iter().map(|a| a.as_string()).collect::<Vec<String>>().join(", ")),
            AST::Matrix(m) => return format!("[{}]", m.iter().map(|v| "[".to_string() + &v.iter().map(|v| v.as_string()).collect::<Vec<String>>().join(", ") + "]").collect::<Vec<String>>().join(", ")),
            AST::List(l) => return format!("{{{}}}", l.iter().map(|a| a.as_string()).collect::<Vec<String>>().join(", ")),
            AST::Variable(v) => return v.to_string(),
            AST::Function { name, inputs } => return format!("{}({})", name, inputs.iter().map(|i| i.as_string()).collect::<Vec<String>>().join(", ")),
            AST::Operation(o) => {
                match &**o  {
                    Operation::SimpleOperation {op_type, left, right} => {
                        let lv = &left.as_string();
                        let rv = &right.as_string(); 
                        match op_type {
                            SimpleOpType::Assign => return format!("{} \u{2254} {}", lv, rv),
                            SimpleOpType::BoolAnd => return format!("{} & {}", lv, rv),
                            SimpleOpType::BoolOr => return format!("{} | {}", lv, rv),
                            SimpleOpType::BoolEq => return format!("{} == {}", lv, rv),
                            SimpleOpType::BoolNEq => return format!("{} != {}", lv, rv),
                            SimpleOpType::BoolLt => return format!("{} < {}", lv, rv),
                            SimpleOpType::BoolGt => return format!("{} > {}", lv, rv),
                            SimpleOpType::BoolLtEq => return format!("{} <= {}", lv, rv),
                            SimpleOpType::BoolGtEq => return format!("{} >= {}", lv, rv),
                            SimpleOpType::BoolNot => return format!("!{}", rv),
                            SimpleOpType::Get => return format!("{}_{}", lv, rv),
                            SimpleOpType::Add => return format!("{} + {}", lv, rv),
                            SimpleOpType::Sub => return format!("{} - {}", lv, rv),
                            SimpleOpType::AddSub => return format!("{} +- {}", lv, rv),
                            SimpleOpType::Mult => return format!("{} * {}", lv, rv),
                            SimpleOpType::Neg => return format!("-{}", rv),
                            SimpleOpType::Div => return format!("{} / {}", lv, rv),
                            SimpleOpType::HiddenMult => return format!("{}{}", lv, rv),
                            SimpleOpType::Pow => return format!("{}^({})", lv, rv),
                            SimpleOpType::Cross => return format!("{}x{}", lv, rv),
                            SimpleOpType::Tetration => return format!("{}^^{}", lv, rv),
                            SimpleOpType::Parenths => return format!("({})", lv),
                        }
                    },
                    Operation::AdvancedOperation(a) => {
                        match a {
                            AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                                let eexpr = &expr.as_string();
                                let elower_b = &lower_bound.as_string();
                                let eupper_b = &upper_bound.as_string();
                                return format!("I({}, {}, {}, {})", eexpr, in_terms_of, elower_b, eupper_b);
                            },
                            AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                                let eexpr = &expr.as_string();
                                let eat = &at.as_string();
                                return format!("D({}, {}, {})", eexpr, in_terms_of, eat);
                            },
                            AdvancedOperation::Equation { equations, .. } => {
                                let eqs: Vec<String> = equations.iter().map(|e| format!("{}={}", e.0.as_string(), e.1.as_string())).collect();
                                return format!("eq({})", eqs.join(","));
                            },
                            AdvancedOperation::Conditional { condition, then, r#else } => {
                                let condition = condition.as_string();
                                let then = then.as_string();
                                if let Some(r#else) = r#else {
                                    return format!("if({}, {}, {})", condition, then, r#else.as_string());
                                } else {
                                    return format!("if({}, {})", condition, then);
                                }
                            },
                            AdvancedOperation::Sum { expr, in_terms_of, lower_bound, upper_bound } => {
                                let eexpr = &expr.as_string();
                                let elower_b = &lower_bound.as_string();
                                let eupper_b = &upper_bound.as_string();
                                return format!("Sum({}, {}, {}, {})", eexpr, in_terms_of, elower_b, eupper_b);
                            }
                        }
                    }
                } 
            }
        }
    }
    /// converts the AST to latex.
    pub fn as_latex(&self) -> String {
        self.latex_print(true)
    }
    /// converts the AST to latex but without an aligner if the AST contains an assignment.
    pub fn as_latex_inline(&self) -> String {
        self.latex_print(false)
    }
    fn latex_print(&self, add_aligner: bool) -> String {
        match self {
            AST::Scalar(s) => return round_and_format(*s, true),
            AST::Vector(v) => {
                let mut output_string = "\\begin{pmatrix}".to_string();
                for i in 0..v.len() {
                    let latex_vi = &v[i].latex_print(false);
                    if i != v.len()-1 {
                        output_string += &format!("{}\\\\ ", latex_vi);
                    } else {
                        output_string += &latex_vi;
                    }
                }
                output_string += "\\end{pmatrix}";
                output_string
            },
            AST::Matrix(m) => {
                let mut output_string = "\\begin{bmatrix}".to_string();
                for i in 0..m.len(){
                    let mut row_string = "".to_string();
                    for j in 0..m[i].len() {
                        let matrix_mij = &m[i][j].latex_print(false);
                        if j != m[i].len()-1 {
                            row_string += &format!("{} & ", matrix_mij);
                        } else {
                            row_string += &format!("{} \\\\", matrix_mij);
                        }
                    }
                    output_string += &row_string;
                }
                output_string += "\\end{bmatrix}";
                return output_string;
            },
            AST::List(l) => return format!("\\left\\{{{}\\right\\}}", l.iter().map(|a| a.latex_print(false)).collect::<Vec<String>>().join(", ")),
            AST::Variable(v) => {
                if v == "pi" {
                    return "\\pi".to_string();
                }
                return v.to_string()
            },
            AST::Function { name, inputs } => {
                let mut inputs_str = String::new();
                for (i, inp) in inputs.iter().enumerate() {
                    let recursed = inp.latex_print(false);
                    if i != inputs.len() - 1 {
                        inputs_str += &format!("{}, ", recursed);
                    } else {
                        inputs_str += &format!("{}", recursed);
                    }
                }
                return format!("{}\\left({}\\right)", name, inputs_str);
            }
            AST::Operation(o) => {
                match &**o  {
                    Operation::SimpleOperation {op_type, left, right} => {
                        let lv = &left.latex_print(false);
                        let rv = &right.latex_print(false); 
                        match op_type {
                            SimpleOpType::Assign => return format!("{}{}\\coloneqq {}", lv, if add_aligner {"&"} else {""}, rv),
                            SimpleOpType::BoolAnd => return format!("{}\\land {}", lv, rv),
                            SimpleOpType::BoolOr => return format!("{}\\lor {}", lv, rv),
                            SimpleOpType::BoolEq => return format!("{}={}", lv, rv),
                            SimpleOpType::BoolNEq => return format!("{}\\neq {}", lv, rv),
                            SimpleOpType::BoolLt => return format!("{}<{}", lv, rv),
                            SimpleOpType::BoolGt => return format!("{}>{}", lv, rv),
                            SimpleOpType::BoolLtEq => return format!("{}\\leq {}", lv, rv),
                            SimpleOpType::BoolGtEq => return format!("{}\\geq {}", lv, rv),
                            SimpleOpType::BoolNot => return format!("\\neg {}", rv),
                            SimpleOpType::Get => return format!("{}_{{{}}}", lv, rv),
                            SimpleOpType::Add => return format!("{}+{}", lv, rv),
                            SimpleOpType::Sub => return format!("{}-{}", lv, rv),
                            SimpleOpType::AddSub => return format!("{}\\pm {}", lv, rv),
                            SimpleOpType::Mult => return format!("{}\\cdot {}", lv, rv),
                            SimpleOpType::Neg => return format!("-{}", rv),
                            SimpleOpType::Div => return format!("\\frac{{{}}}{{{}}}", lv, rv),
                            SimpleOpType::HiddenMult => return format!("{}{}", lv, rv),
                            SimpleOpType::Pow => return format!("{}^{{{}}}", lv, rv),
                            SimpleOpType::Cross => return format!("{}\\times {}", lv, rv),
                            SimpleOpType::Tetration => return format!("{{^{{{}}}{}}}", rv, rv),
                            SimpleOpType::Parenths => return format!("\\left({}\\right)", lv),
                        }
                    },
                    Operation::AdvancedOperation(a) => {
                        match a {
                            AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                                let eexpr = &expr.latex_print(false);
                                let elower_b = &lower_bound.latex_print(false);
                                let eupper_b = &upper_bound.latex_print(false);
                                return format!("\\int_{{{}}}^{{{}}}{} d{}", elower_b, eupper_b, eexpr, in_terms_of);
                            },
                            AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                                let eexpr = &expr.latex_print(false);
                                let eat = &at.latex_print(false);
                                return format!("\\frac{{\\partial}}{{\\partial {}}}\\left({}\\right)_{{\\text{{at }}{} = {}}}", in_terms_of, eexpr, in_terms_of, eat);
                            },
                            AdvancedOperation::Equation { equations, .. } => {
                                let eqs: Vec<String> = equations.iter().map(|e| format!("{}&={}", e.0.latex_print(false), e.1.latex_print(false))).collect();
                                return format!("\\left\\{{\\begin{{array}}{{ c l }}{}\\end{{array}}\\right.", eqs.join("\\\\ \n"))
                            },
                            AdvancedOperation::Conditional { condition, then, r#else } => {
                                let (conditionals, r#else) = flatten_conditional(condition, then, r#else);


                                let conditionals: Vec<(String, String)> = conditionals.into_iter()
                                    .map(|c| (c.0.latex_print(false), c.1.latex_print(false))).collect();
                                
                                let r#else = if let Some(else_def) = r#else {Some(else_def.latex_print(false))} else {None};

                                let mut return_string = r"\left\{\begin{array}{ c l }".to_string();

                                for conditional in conditionals {
                                    return_string += &format!("{} & \\quad \\textrm{{if }} {} \\\\", conditional.1, conditional.0);
                                }

                                if let Some(latex_else) = r#else {
                                    return_string += &format!("{} & \\quad \\textrm{{otherwise}}", latex_else);
                                }

                                return_string += r"\end{array}\right.";

                                return return_string;
                            },
                            AdvancedOperation::Sum { expr, in_terms_of, lower_bound, upper_bound } => {
                                let eexpr = &expr.latex_print(false);
                                let elower_b = &lower_bound.latex_print(false);
                                let eupper_b = &upper_bound.latex_print(false);
                                return format!("\\sum_{{{} = {}}}^{{{}}}\\left({}\\right)", in_terms_of, elower_b, eupper_b, eexpr);
                            }
                        }
                    }
                } 
            }
        }
    }
    /// converts the ast to a different number type.
    pub fn into<B: Number + From<N>>(self) -> AST<B> {
        match self {
            AST::Scalar(s) => AST::Scalar(s.into()),
            AST::List(l) => AST::List(l.into_iter().map(|ast| ast.into()).collect()),
            AST::Vector(v) => AST::Vector(v.into_iter().map(|ast| ast.into()).collect()),
            AST::Matrix(m) => AST::Matrix(m.into_iter().map(|v| v.into_iter().map(|ast| ast.into()).collect()).collect()),
            AST::Variable(var_name) => AST::Variable(var_name.clone()),
            AST::Function { name, inputs } => AST::Function { name, inputs: inputs.into_iter().map(|ast| ast.into()).collect() },
            AST::Operation(op) => {
                AST::Operation(Box::new(match *op {
                    Operation::SimpleOperation { op_type, left, right } => Operation::SimpleOperation { op_type, left: left.into(), right: right.into() },
                    Operation::AdvancedOperation(aop) => {
                        Operation::AdvancedOperation(match aop {
                            AdvancedOperation::Integral { expr, in_terms_of, lower_bound, upper_bound } => AdvancedOperation::Integral {
                                expr: expr.into(),
                                in_terms_of,
                                lower_bound: lower_bound.into(),
                                upper_bound: upper_bound.into()
                            },
                            AdvancedOperation::Equation { equations, search_vars } => AdvancedOperation::Equation {
                                equations: equations.into_iter().map(|(e1, e2)| (e1.into(), e2.into())).collect(),
                                search_vars
                            },
                            AdvancedOperation::Conditional { condition, then, r#else } => AdvancedOperation::Conditional {
                                condition: condition.into(),
                                then: then.into(),
                                r#else: if let Some(some_else) = r#else { Some(some_else.into()) } else {None}
                            },
                            AdvancedOperation::Derivative { expr, in_terms_of, at } => AdvancedOperation::Derivative {
                                expr: expr.into(),
                                in_terms_of,
                                at: at.into()
                            },
                            AdvancedOperation::Sum { expr, in_terms_of, lower_bound, upper_bound } => AdvancedOperation::Sum {
                                expr: expr.into(),
                                in_terms_of,
                                lower_bound: lower_bound.into(),
                                upper_bound: upper_bound.into()
                            },
                        })
                    }
                }))
            }
        }
    }
}

/// specifies the type of operation for the [SimpleOperation](Operation::SimpleOperation) enum
/// variant.
/// 
/// This enum only contains simple mathematical operations with a left and right side. For more advanced operations, see [AdvancedOperation].
/// 
/// The order of the enum also represents the reverse order of the operation priority.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SimpleOpType {
    /// Compute the boolean and of two expressions (1==2 & 2==2)
    BoolAnd = 0,
    /// Compute the boolean or of two expressions (1==2 | 2==2)
    BoolOr = 1,
    /// Assign the result of an expression to a variable or assign an expression to a function (x =
    /// 10, f(x) = x^2)
    Assign = 2,
    /// Test for equality, returning 1 if true and 0 if false (3==4)
    BoolEq = 3,
    /// Test for inequality, returning 1 if true and 0 if false (3!=4)
    BoolNEq = 4,
    /// Test for x less than y, returning 1 if true and 0 if false (3<4)
    BoolLt = 5,
    /// Test for x greater than y, returning 1 if true and 0 if false (3>4)
    BoolGt = 6,
    /// Test for x less than or equal y, returning 1 if true and 0 if false (3<=4)
    BoolLtEq = 7,
    /// Test for x greater than or equal y, returning 1 if true and 0 if false (3>=4)
    BoolGtEq = 8,
    /// Calculating the not of a boolean value, turning any number != 0 into a 0 and turning 0 into
    /// 1 (!3)
    BoolNot = 9,
    /// Add two scalars, vectors, or matrices (a+b)
    Add = 10,
    /// Subtract two scalars, vectors, or matrices (a-b)
    Sub = 11,
    /// Add and subtract two scalars, vectors or matrices (a+-b)
    AddSub = 12,
    /// Negate a scalar, vector or matrix or expression in parentheses (-(3*4))
    Neg = 13,
    /// Multiply a scalar, vector or matrix with each other (Dotproduct, Matrix multiplication,
    /// Scalar multiplication, ...) (a*b)
    Mult = 14,
    /// Divide two scalars or a vector or matrix with a scalar (a/b)
    Div = 15,
    /// Calculate the cross product using "#" (V1#V2), only works with dim(V) <= 3. When dim(V) < 3
    /// the vector gets augmented with zeros
    Cross = 16,
    /// Hidden multiplication between scalar and variable or parentheses (3a, 5(3+3), (3+5)(2+6))
    HiddenMult = 17,
    /// Take a scalar or a matrix to the power of a scalar using "^" (a^b)
    Pow = 19,
    /// Calculate the repeated exponential (a^^n), i.e. a^a^a^a.. n times.
    Tetration = 20,
    /// Index into vector using "@" ([3, 4, 5]@1 = 4)
    Get = 21,
    /// Prioritise expressions in parentheses (3*(5+5))
    Parenths = 22
}

/// used to specify an operation in a parsed string. It is used together with [AST] to
/// construct an AST from a mathematical expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Operation<N: Number> {
    SimpleOperation {
        op_type: SimpleOpType,
        left: AST<N>,
        right: AST<N>,
    },
    AdvancedOperation(AdvancedOperation<N>)
}

/// used to specify an advanced operation for more complex mathematical operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdvancedOperation<N: Number>{
    Integral {
        expr: AST<N>,
        in_terms_of: String,
        lower_bound: AST<N>,
        upper_bound: AST<N>
    },
    Derivative {
        expr: AST<N>,
        in_terms_of: String,
        at: AST<N>
    },
    Equation {
        equations: Vec<(AST<N>, AST<N>)>,
        search_vars: Vec<String>
    },
    Conditional {
        condition: AST<N>,
        then: AST<N>,
        r#else: Option<AST<N>>
    },
    Sum {
        expr: AST<N>,
        in_terms_of: String,
        lower_bound: AST<N>,
        upper_bound: AST<N>
    }
}
