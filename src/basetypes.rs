use crate::helpers::{center_in_string, round_and_format};

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
/// Variable Names following the LaTeX format for greek letters (e.g \sigma) (except pi which is not
/// \pi but just pi) will get replaced with their unicode counterparts when pretty printing.
/// 
/// Variable Names are not allowed to contain numbers outside of LaTeX style subscript. Additionally
/// they must start with an alphabetical letter or a "\\".
/// 
/// # Example
/// 
/// ```
/// let variable = Variable::new("x", value!(3.));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable {
    pub name: String,
    pub values: Values
}

impl Variable {
    /// creates a new variable from a Vec of [Value].
    pub fn new<S: Into<String>, V: Into<Values>>(name: S, values: V) -> Self {
        Variable { name: name.into(), values: values.into()}
    }
    /// creates a new variable from [Values].
    pub fn new_from_values<S: Into<String>>(name: S, values: Values) -> Self {
        Variable { name: name.into(), values }
    }
    /// converts the variable to latex. The function also provides the option to add a "&" aligner before the
    /// "=".
    pub fn as_latex(&self, add_aligner: bool) -> String {
        self.values.as_latex_at_var(self.name.clone(), add_aligner)
    }
}

/// describes a function that can be used in the context of an evaluation.
///
/// Function names must follow the same criteria as [Variable] names.
///
/// # Example
///
/// ```
/// let parsed_expr = parse("x^2")?;
/// let function = Function::new("f", parsed_expr, vec!["x"]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    pub name: String,
    pub ast: AST,
    pub inputs: Vec<String>
}

impl Function {
    /// creates a new function from an [AST] (a parsed expression) and a Vec of input variable
    /// names.
    pub fn new<S: Into<String>>(name: S, ast: AST, inputs: Vec<S>) -> Function {
        Function { name: name.into(), ast, inputs: inputs.into_iter().map(|s| s.into()).collect() }
    }
    /// converts the function to latex. The function also provides the option to add a "&" aligner before
    /// the "=".
    pub fn as_latex(&self, add_aligner: bool) -> String {
        self.ast.as_latex_at_fun(self.name.clone(), self.inputs.clone(), add_aligner)
    }
}

/// combines [Variable]s and [Function]s into a convenient struct, which then gets passed to the
/// evaluator.
///
/// # Example
///
/// ```
/// let context = Context::default();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Context {
    pub vars: Vec<Variable>,
    pub funs: Vec<Function>
}

impl Context {
    /// creates a context with the variables pi and e and no functions.
    pub fn default() -> Self {
        Context::from_vars(vec![
            Variable::new("pi", Value::Scalar(std::f64::consts::PI)),
            Variable::new("e", Value::Scalar(std::f64::consts::E))
        ])
    }
    /// creates a context with the given variables and functions.
    pub fn new<V: AsRef<[Variable]>, F: AsRef<[Function]>>(vars: V, funs: F) -> Context {
        Context {vars: vars.as_ref().to_vec(), funs: funs.as_ref().to_vec()}
    }
    /// creates an empty context.
    pub fn empty() -> Context {
        Context { vars: vec![], funs: vec![] }
    }
    /// creates a new context containing only the given variables.
    pub fn from_vars<V: AsRef<[Variable]>>(vars: V) -> Context {
        Context { vars: vars.as_ref().to_vec(), funs: vec![] }
    }
    /// creates a new context containing only the given functions.
    pub fn from_funs<F: AsRef<[Function]>>(funs: F) -> Context {
        Context { vars: vec![], funs: funs.as_ref().to_vec() }
    }
    /// adds a variable to the context, replacing an already existing variable with the same name.
    pub fn add_var(&mut self, var: &Variable) {
        self.vars = self.vars.iter()
            .filter(|v| v.name != var.name)
            .map(|v| v.to_owned())
            .collect();

        self.vars.push(var.to_owned());
    }
    /// adds a function to the context, replacing an already existing function with the same name.
    pub fn add_fun(&mut self, fun: &Function) {
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
}

/// helps to quickly initialize a [Value].
///
/// # Example
///
/// ```
/// let x: Value = value!(3.5);
/// ```
#[macro_export]
macro_rules! value {
    ( $x:expr ) => {
        Value::Scalar($x)
    };
    ( $($x:expr),+ ) => {
        {
            let mut vector = Vec::new();
            $(
                vector.push($x);
            )*
            Value::Vector(vector)
        }
    };
    ( $($($x:expr),+);+ ) => {
        {
            let mut vector = Vec::new();
            $(
                let mut row = Vec::new();
                $(
                    row.push($x);
                )*
                vector.push(row);
            )*
            Value::Matrix(vector)
        }
    };
}

/// specifies a Value that can be a Matrix, Vector or a Scalar.
/// 
/// # Example
/// 
/// ```
/// let x: Value = Value::Scalar(3.5);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Matrix(Vec<Vec<f64>>),
    Vector(Vec<f64>),
    Scalar(f64)
}

impl Value {
    /// returns the scalar if the value is a scalar and None if it is a matrix or a
    /// vector.
    pub fn get_scalar(&self) -> Option<f64> {
        match self {
            Value::Scalar(a) => return Some(*a),
            Value::Matrix(_) => return None,
            Value::Vector(_) => return None
        }
    }
    /// returns the vector if the value is a vector and None if it is a matrix or a
    /// scalar.
    pub fn get_vector(&self) -> Option<Vec<f64>> {
        match self {
            Value::Vector(a) => return Some(a.to_vec()),
            Value::Matrix(_) => return None,
            Value::Scalar(_) => return None
        }
    }
    /// returns the matrix if the value is a matrix and None if it is a scalar or a
    /// vector.
    pub fn get_matrix(&self) -> Option<Vec<Vec<f64>>> {
        match self {
            Value::Matrix(a) => return Some(a.to_vec()),
            Value::Scalar(_) => return None,
            Value::Vector(_) => return None
        }
    }
    /// return true if the value is a scalar.
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
    pub fn round(&self, prec: usize) -> Value {
        match self {
            Value::Scalar(a) => return Value::Scalar((a*10f64.powi(prec as i32)).round()/10f64.powi(prec as i32)),
            Value::Vector(v) => {
                let mut new_vec = vec![];
                for i in v {
                    new_vec.push((i*10f64.powi(prec as i32)).round()/10f64.powi(prec as i32));
                }
                return Value::Vector(new_vec);
            },
            Value::Matrix(m) => {
                let mut new_matrix = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push((j*10f64.powi(prec as i32)).round()/10f64.powi(prec as i32));
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
                            replace_string += ","
                        }
                    }
                    replace_string += "]";
                    if k != s.len() - 1 {
                        replace_string += ",";
                    }
                }
                replace_string += "]";
            },
            Value::Vector(s) => {
                replace_string += "[";
                for k in 0..s.len() {
                    replace_string += &s[k].to_string();
                    if k != s.len() - 1 {
                        replace_string += ",";
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
    /// converts the value to a latex expression, adding a variable name in front of it. The
    /// function also provides the option to add a "&" aligner before the "=".
    pub fn as_latex_at_var<S: Into<String>>(&self, var_name: S, add_aligner: bool) -> String {
        let aligner;
        if add_aligner {
            aligner = "&";
        } else {
            aligner = "";
        }

        let mut var = var_name.into();

        if var == "pi" {
            var = "\\pi".to_string();
        }

        return format!("{} {}= {}", var, aligner, self.as_latex());
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
}

impl Into<Values> for Value {
    fn into(self) -> Values {
        return Values::from_vec(vec![self]);
    }
}

impl Into<Values> for Vec<Value> {
    fn into(self) -> Values {
        return Values::from_vec(self);
    }
}

/// provides a wrapper around Vec of Value with some quality of life implementations.
///
/// # Example
///
/// ```
/// let values = Values::from_vec(vec![Value::Scalar(3.)]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Values(Vec<Value>);

impl Values {
    /// creates the values from a Vec of [Value].
    pub fn from_vec<V: AsRef<[Value]>>(values: V) -> Self {
        return Values(values.as_ref().to_vec());
    }
    /// converts the values back to a Vec of [Value].
    pub fn to_vec(self) -> Vec<Value> {
        return self.0;
    }
    /// gets the [Value] at the given index.
    pub fn get(&self, i: usize) -> Option<&Value> {
        self.0.iter().nth(i)
    }
    /// returns the length of the values.
    pub fn len(&self) -> usize {
        return self.0.len()
    }
    /// rounds all values.
    pub fn round(&self, prec: usize) -> Values {
        let rounded_vals = self.0.iter().map(|x| x.round(prec)).collect::<Vec<Value>>();
        Values::from_vec(rounded_vals)
    }
    /// converts the values to a string using "{}" and "," to print multiple Values. This is a crude
    /// way to convert [Values] as it uses [Value::as_string].
    pub fn as_string(&self) -> String {
        format!("{{{}}}", self.clone().to_vec().iter().map(|v| v.as_string()).collect::<Vec<String>>().join(", "))
    }
    /// converts the values to latex using "{}" and ";" to print multiple Values.
    pub fn as_latex(&self) -> String {
        if self.len() == 1 {
            return format!("{}", self.0[0].as_latex());
        } else if self.len() <= 0 {
            return "No solutions".to_string();
        } else {
            return format!("\\left\\{{{}\\right\\}}", self.clone().to_vec().iter().map(|v| v.as_latex()).collect::<Vec<String>>().join("; "));
        }
    }
    /// converts the values to latex using "{}" and ";" to print multiple Values. This functions
    /// additionally adds a variable name in front of the Values. The function also provides the option to
    /// add a "&" aligner before the "=".
    pub fn as_latex_at_var<S: Into<String>>(&self, var_name: S, add_aligner: bool) -> String {
        let aligner;
        if add_aligner {
            aligner = "&";
        } else {
            aligner = "";
        }

        let mut var = var_name.into();

        if var == "pi" {
            var = "\\pi".to_string();
        }

        if self.len() <= 0 {
            return format!("{}: No solutions", var);
        } else if self.len() == 1 {
            return format!("{} {}= {}", var, aligner, self.0[0].as_latex());
        } else {
            return format!("{} {}= \\left\\{{{}\\right\\}}", var, aligner, self.clone().to_vec().iter().map(|v| v.as_latex()).collect::<Vec<String>>().join("; "));
        }
    }
}

/// used to construct an AST which is recursively evaluated by the [eval](crate::parser::eval) function.
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
pub enum AST {
    Scalar(f64),
    Vector(Box<Vec<AST>>),
    Matrix(Box<Vec<Vec<AST>>>),
    List(Vec<AST>),
    Variable(String),
    Function {
        name: String,
        inputs: Box<Vec<AST>>
    },
    Operation(Box<Operation>),
}

impl AST {
    /// creates an AST node from a [Value].
    pub fn from_value(val: Value) -> AST {
        match val {
            Value::Scalar(s) => return AST::Scalar(s),
            Value::Vector(v) => {
                let mut parsed_values = vec![];
                for i in v {
                    parsed_values.push(AST::Scalar(i))
                }
                return AST::Vector(Box::new(parsed_values))
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
                return AST::Matrix(Box::new(parsed_rows));
            }
        }
    }
    /// creates an AST node from a variable name.
    pub fn from_variable_name<S: Into<String>>(val: S) -> AST {
        return AST::Variable(val.into());
    }
    /// creates an AST node from an operation.
    pub fn from_operation(val: Operation) -> AST {
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
                            SimpleOpType::Abs => return format!("|{}|", lv),
                            SimpleOpType::Sin => return format!("sin({})", lv),
                            SimpleOpType::Cos => return format!("cos({})", lv),
                            SimpleOpType::Tan => return format!("tan({})", lv),
                            SimpleOpType::Sqrt => return format!("sqrt({})", lv),
                            SimpleOpType::Root => return format!("root({}, {})", lv, rv),
                            SimpleOpType::Ln => return format!("ln({})", lv),
                            SimpleOpType::Arcsin => return format!("arcsin({})", lv),
                            SimpleOpType::Arccos => return format!("arccos({})", lv),
                            SimpleOpType::Arctan => return format!("arctan({})", lv),
                            SimpleOpType::Det => return format!("det({})", lv),
                            SimpleOpType::Inv => return format!("inv({})", lv),
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
                            }
                        }
                    }
                } 
            }
        }
    }
    /// converts the AST to latex.
    pub fn as_latex(&self) -> String {
        self.latex_print()
    }
    /// converts the AST to latex, adding a function identifier in front of the term. The function
    /// also provides the option to add a "&" aligner in front of the "=".
    pub fn as_latex_at_fun<S: Into<String>>(&self, fun_name: S, fun_inputs: Vec<S>, add_aligner: bool) -> String {
        let aligner;
        if add_aligner {
            aligner = "&".to_string();
        } else {
            aligner = String::new();
        }
        format!("{}({}) {}= {}", fun_name.into(), fun_inputs.into_iter().map(|s| s.into()).collect::<Vec<String>>().join(", "), aligner, self.latex_print())
    }
    fn latex_print(&self) -> String {
        match self {
            AST::Scalar(s) => return round_and_format(*s, true),
            AST::Vector(v) => {
                let mut output_string = "\\begin{pmatrix}".to_string();
                for i in 0..v.len() {
                    let latex_vi = &v[i].latex_print();
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
                        let matrix_mij = &m[i][j].latex_print();
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
            AST::List(l) => return format!("\\left\\{{{}\\right\\}}", l.iter().map(|a| a.latex_print()).collect::<Vec<String>>().join("; ")),
            AST::Variable(v) => {
                if v == "pi" {
                    return "\\pi".to_string();
                }
                return v.to_string()
            },
            AST::Function { name, inputs } => {
                let mut inputs_str = String::new();
                for (i, inp) in inputs.iter().enumerate() {
                    let recursed = inp.latex_print();
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
                        let lv = &left.latex_print();
                        let rv = &right.latex_print(); 
                        match op_type {
                            SimpleOpType::Get => return format!("{}_{{{}}}", lv, rv),
                            SimpleOpType::Add => return format!("{}+{}", lv, rv),
                            SimpleOpType::Sub => return format!("{}-{}", lv, rv),
                            SimpleOpType::AddSub => return format!("{}\\pm{}", lv, rv),
                            SimpleOpType::Mult => return format!("{}\\cdot {}", lv, rv),
                            SimpleOpType::Neg => return format!("-{}", rv),
                            SimpleOpType::Div => return format!("\\frac{{{}}}{{{}}}", lv, rv),
                            SimpleOpType::HiddenMult => return format!("{}{}", lv, rv),
                            SimpleOpType::Pow => return format!("{}^{{{}}}", lv, rv),
                            SimpleOpType::Cross => return format!("{}\\times {}", lv, rv),
                            SimpleOpType::Abs => return format!("|{}|", lv),
                            SimpleOpType::Sin => return format!("\\sin\\left({}\\right)", lv),
                            SimpleOpType::Cos => return format!("\\cos\\left({}\\right)", lv),
                            SimpleOpType::Tan => return format!("\\tan\\left({}\\right)", lv),
                            SimpleOpType::Sqrt => return format!("\\sqrt{{{}}}", lv),
                            SimpleOpType::Root => return format!("\\sqrt[{}]{{{}}}", rv, lv),
                            SimpleOpType::Ln => return format!("\\ln\\left({}\\right)", lv),
                            SimpleOpType::Arcsin => return format!("\\arcsin\\left({}\\right)", lv),
                            SimpleOpType::Arccos => return format!("\\arccos\\left({}\\right)", lv),
                            SimpleOpType::Arctan => return format!("\\arctan\\left({}\\right)", lv),
                            SimpleOpType::Det => return format!("\\det\\left({}\\right)", lv),
                            SimpleOpType::Inv => return format!("{}^{{-1}}", lv),
                            SimpleOpType::Parenths => return format!("\\left({}\\right)", lv),
                        }
                    },
                    Operation::AdvancedOperation(a) => {
                        match a {
                            AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                                let eexpr = &expr.latex_print();
                                let elower_b = &lower_bound.latex_print();
                                let eupper_b = &upper_bound.latex_print();
                                return format!("\\int_{{{}}}^{{{}}}{} d{}", elower_b, eupper_b, eexpr, in_terms_of);
                            },
                            AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                                let eexpr = &expr.latex_print();
                                let eat = &at.latex_print();
                                return format!("\\frac{{\\partial}}{{\\partial {}}}\\left({}\\right)_{{\\text{{at }}{} = {}}}", in_terms_of, eexpr, in_terms_of, eat);
                            },
                            AdvancedOperation::Equation { equations, .. } => {
                                let eqs: Vec<String> = equations.iter().map(|e| format!("{}&={}", e.0.latex_print(), e.1.latex_print())).collect();
                                return format!("\\left|\\begin{{align}}{}\\end{{align}}\\right|", eqs.join("\\\\ \n "))
                            }
                        }
                    }
                } 
            }
        }
    }
}

/// specifies the type of operation for the [SimpleOperation](Operation::SimpleOperation) struct.
/// 
/// This enum only contains simple mathematical operations with a left and right side or a maximum
/// of two arguments. For more advanced operations, see [AdvancedOpType].
/// 
/// The order of the enum also represents the reverse order of the operation priority.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SimpleOpType { 
    /// Add two scalars, vectors, or matrices (a+b)
    Add,
    /// Subtract two scalars, vectors, or matrices (a-b)
    Sub,
    /// Add and subtract two scalars, vectors or matrices (a&b)
    AddSub,
    /// Negate a scalar, vector or matrix or expression in parentheses (-(3*4))
    Neg,
    /// Multiply a scalar, vector or matrix with each other (Dotproduct, Matrix multiplication,
    /// Scalar multiplication, ...) (a*b)
    Mult,
    /// Divide two scalars or a vector or matrix with a scalar (a/b)
    Div,
    /// Calculate the cross product using "#" (V1#V2), only works with dim(V) <= 3. When dim(V) < 3
    /// the vector gets augmented with zeros
    Cross,
    /// Hidden multiplication between scalar and variable or parentheses (3a, 5(3+3), (3+5)(2+6))
    HiddenMult,
    /// Take a scalar to the power of another scalar using "^" (a^b)
    Pow,
    /// Index into vector using "?" ([3, 4, 5]?1 = 4)
    Get,
    /// Calculate the sin of a scalar (sin(a))
    Sin,
    /// Calculate the cos of a scalar (cos(a))
    Cos,
    /// Calculate the tan of a scalar (tan(a))
    Tan,
    /// Calculate the absolute value of a scalar or the length of a vector (abs(a))
    Abs,
    /// Calculate the square root of a scalar (sqrt(a))
    Sqrt,
    /// Calculate the nth root of a scalar (root(a, n))
    Root,
    /// Calculate the natural log of a scalar (ln(a))
    Ln,
    /// Calculate the arcsin of a scalar (arcsin(a))
    Arcsin,
    /// Calculate the arccos of a scalar (arccos(a))
    Arccos,
    /// Calculate the arctan of a scalar (arctan(a))
    Arctan,
    /// Calculate the determinant of a matrix (det(M))
    Det,
    /// Calculate the inverse of a matrix (inv(M))
    Inv,
    /// Prioritise expressions in parentheses (3*(5+5))
    Parenths
}

/// specifies the type of operation for the [AdvancedOperation] struct.
///
/// This enum only contains advanced operations with more than 2 arguments. For simple operations,
/// see [SimpleOpType].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdvancedOpType {
    /// Calculate the derivative of a function f in respect to n at a value m (D(f, n, m))
    Derivative,
    /// Calculate the integral of a function f in respect to n with the bounds a and b (I(f, n, a, b))
    Integral,
    /// Solve the given equation(s) in terms of the given variable(s) (eq(eq_1, eq_2, eq_3, ..., x, y,
    /// z, ...))
    Equation,
}

/// used to specify an operation in a parsed string. It is used together with [AST] to
/// construct an AST from a mathematical expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Operation {
    SimpleOperation {
        op_type: SimpleOpType,
        left: AST,
        right: AST,
    },
    AdvancedOperation(AdvancedOperation)
}

/// used to specify an advanced operation for more complex mathematical operations, such as
/// functions with more than two inputs and the equation solver.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdvancedOperation{
    Integral {
        expr: AST,
        in_terms_of: String,
        lower_bound: AST,
        upper_bound: AST
    },
    Derivative {
        expr: AST,
        in_terms_of: String,
        at: AST
    },
    Equation {
        equations: Vec<(AST, AST)>,
        search_vars: Vec<String>
    }
}
