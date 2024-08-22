use crate::helpers::{center_in_string, round_and_format};

#[doc(hidden)]
const VAR_SYMBOLS: [(&str, &str); 48] = [("\\alpha", "𝛼"), ("\\Alpha", "𝛢"), ("\\beta", "𝛽"), ("\\Beta", "𝛣"), ("\\gamma", "𝛾"), ("\\Gamma", "𝚪"),
("\\delta", "𝛿"), ("\\Delta", "𝛥"), ("\\epsilon", "𝜺"), ("\\Epsilon", "𝛦"), ("\\zeta", "𝜁"), ("\\Zeta", "𝛧"), ("\\eta", "𝜂"), ("\\Eta", "𝛨"),
("\\theta", "𝜃"), ("\\Theta", "𝛩"), ("\\iota", "𝜄"), ("\\Iota", "𝛪"), ("\\kappa", "𝜅"), ("\\Kappa", "𝛫"), ("\\lambda", "𝜆"), ("\\Lambda", "𝛬"),
("\\mu", "𝜇"), ("\\Mu", "𝛭"), ("\\nu", "𝜈"), ("\\Nu", "𝛮"), ("\\xi", "𝜉"), ("\\Xi", "𝛯"), ("\\omicron", "𝜊"), ("\\Omicron", "𝛰"), ("pi", "𝜋"),
("\\Pi", "𝛱"), ("\\rho", "𝜌"), ("\\Rho", "𝛲"), ("\\sigma", "𝜎"), ("\\Sigma", "𝛴"), ("\\tau", "𝜏"), ("\\Tau", "𝛵"), ("\\upsilon", "𝜐"),
("\\Upsilon", "𝛶"), ("\\phi", "𝜑"), ("\\Phi", "𝛷"), ("\\xi", "𝜒"), ("\\Xi", "𝛸"), ("\\psi", "𝜓"), ("\\Psi", "𝛹"), ("\\omega", "𝜔"), ("\\Omega", "𝛺")];

///specifies a Variable that can be used in the context of an evaluation or equation.
///
///Variable Names following the LaTeX format for greek letters (e.g \sigma) (except pi which is not
///\pi but just pi) will get replaced with their unicode counterparts when pretty printing.
///
///Variable Names are not allowed to contain numbers outside of LaTeX style subscript. Additionally
///they must start with an alphabetical letter or a \\.
///
///# Example
///
///```
///let context: Vec<Variable> = vec![
///     Variable::new("pi".to_string(), Value::Scalar(3.14159)),
///];
///```
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: Value
}

impl Variable {
    /// creates a new [Variable] from a [Value].
    pub fn new<S: Into<String>>(name: S, value: Value) -> Variable {
        Variable { name: name.into(), value}
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub ast: AST,
    pub inputs: Vec<String>
}

impl Function {
    pub fn new<S: Into<String>>(name: S, ast: AST, inputs: Vec<S>) -> Function {
        Function { name: name.into(), ast, inputs: inputs.into_iter().map(|s| s.into()).collect() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub vars: Vec<Variable>,
    pub funs: Vec<Function>
}

impl Store {
    pub fn default() -> Self {
        Store::from_vars(vec![
            Variable::new("pi", Value::Scalar(std::f64::consts::PI)),
            Variable::new("e", Value::Scalar(std::f64::consts::E))
        ])
    }
    pub fn new<V: AsRef<[Variable]>, F: AsRef<[Function]>>(vars: V, funs: F) -> Store {
        Store {vars: vars.as_ref().to_vec(), funs: funs.as_ref().to_vec()}
    }
    pub fn empty() -> Store {
        Store { vars: vec![], funs: vec![] }
    }
    pub fn from_vars<V: AsRef<[Variable]>>(vars: V) -> Store {
        Store { vars: vars.as_ref().to_vec(), funs: vec![] }
    }
    pub fn from_funs<F: AsRef<[Function]>>(funs: F) -> Store {
        Store { vars: vec![], funs: funs.as_ref().to_vec() }
    }
    pub fn add_var(&mut self, var: &Variable) {
        self.vars = self.vars.iter()
            .filter(|v| v.name != var.name)
            .map(|v| v.to_owned())
            .collect();

        self.vars.push(var.to_owned());
    }
    pub fn add_fun(&mut self, fun: Function) {
        self.funs = self.funs.iter()
            .filter(|f| f.name != fun.name)
            .map(|f| f.to_owned())
            .collect();

        self.funs.push(fun);
    }
    pub fn remove_var<S: Into<String> + Clone>(&mut self, var_name: S) {
        self.vars = self.vars.iter()
            .filter(|v| v.name != var_name.clone().into())
            .map(|v| v.to_owned())
            .collect();
    }
    pub fn remove_fun<S: Into<String> + Clone>(&mut self, fun_name: S) {
        self.funs = self.funs.iter()
            .filter(|f| f.name != fun_name.clone().into())
            .map(|f| f.to_owned())
            .collect()
    }
}

///specifies a Value that can be a Matrix, Vector or a Scalar.
///
///# Example
///
///```
///let x: Value = Value::Scalar(3.5);
///```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Matrix(Vec<Vec<f64>>),
    Vector(Vec<f64>),
    Scalar(f64)
}

impl Value {
    ///returns the scalar if the Value is a scalar and None if it is a matrix or a
    ///vector.
    pub fn get_scalar(&self) -> Option<f64> {
        match self {
            Value::Scalar(a) => return Some(*a),
            Value::Matrix(_) => return None,
            Value::Vector(_) => return None
        }
    }
    ///returns the vector if the Value is a vector and None if it is a matrix or a
    ///scalar.
    pub fn get_vector(&self) -> Option<Vec<f64>> {
        match self {
            Value::Vector(a) => return Some(a.to_vec()),
            Value::Matrix(_) => return None,
            Value::Scalar(_) => return None
        }
    }
    ///returns the matrix if the Value is a matrix and None if it is a scalar or a
    ///vector.
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
    pub fn round(&self, prec: i32) -> Value {
        match self {
            Value::Scalar(a) => return Value::Scalar((a*10f64.powi(prec)).round()/10f64.powi(prec)),
            Value::Vector(v) => {
                let mut new_vec = vec![];
                for i in v {
                    new_vec.push((i*10f64.powi(prec)).round()/10f64.powi(prec));
                }
                return Value::Vector(new_vec);
            },
            Value::Matrix(m) => {
                let mut new_matrix = vec![];
                for i in m {
                    let mut row = vec![];
                    for j in i {
                        row.push((j*10f64.powi(prec)).round()/10f64.powi(prec));
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
    ///provides a crude method to convert a Value to a string, using square brackets
    ///for matrices and vectors.
    pub fn to_string(&self) -> String {
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
    pub fn to_unicode(&self) -> String {
        self.pretty_print(None)
    }
    pub fn to_unicode_at_var<S: Into<String>>(&self, var_name: S) -> String {
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
    pub fn to_latex(&self) -> String {
        self.latex_print()
    }
    pub fn to_latex_at_var<S: Into<String>>(&self, var_name: S, add_aligner: bool) -> String{
        let mut var_name = var_name.into();
        if var_name == "pi" {
            var_name = "\\pi".to_string();
        }
        let aligner;
        if add_aligner {
            aligner = "&".to_string();
        } else {
            aligner = String::new();
        }
        format!("{} {}= {}", var_name, aligner, self.latex_print())
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

#[derive(Debug, Clone, PartialEq)]
pub struct Values(Vec<Value>);

impl Values {

}

///used to construct a AST Tree which is recursively evaluated by the [eval()] function.
///
///AST can be a:
///
///- Value
///- Variable
///- Operation
#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Scalar(f64),
    Vector(Box<Vec<AST>>),
    Matrix(Box<Vec<Vec<AST>>>),
    Variable(String),
    Function {
        name: String,
        inputs: Box<Vec<AST>>
    },
    Operation(Box<Operation>),
}

impl AST {
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
    pub fn from_variable_name<S: Into<String>>(val: S) -> AST {
        return AST::Variable(val.into());
    }
    pub fn from_operation(val: Operation) -> AST {
        return AST::Operation(Box::new(val));
    }
    pub fn to_string(&self) -> String {
        match self {
            AST::Scalar(s) => return round_and_format(*s, false),
            AST::Vector(v) => return format!("[{}]", v.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ")),
            AST::Matrix(m) => return format!("[{}]", m.iter().map(|v| "[".to_string() + &v.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ") + "]").collect::<Vec<String>>().join(", ")),
            AST::Variable(v) => return v.to_string(),
            AST::Function { name, inputs } => return format!("{}({})", name, inputs.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")),
            AST::Operation(o) => {
                match &**o  {
                    Operation::SimpleOperation {op_type, left, right} => {
                        let lv = &left.to_string();
                        let rv = &right.to_string(); 
                        match op_type {
                            SimpleOpType::Get => return format!("{}_{}", lv, rv),
                            SimpleOpType::Add => return format!("{} + {}", lv, rv),
                            SimpleOpType::Sub => return format!("{} - {}", lv, rv),
                            SimpleOpType::Mult => return format!("{} * {}", lv, rv),
                            SimpleOpType::Neg => return format!("-{}", lv),
                            SimpleOpType::Div => return format!("{} / {}", lv, rv),
                            SimpleOpType::HiddenMult => return format!("{}{}", lv, rv),
                            SimpleOpType::Pow => return format!("{}^({})", lv, rv),
                            SimpleOpType::Cross => return format!("{}x{}", lv, rv),
                            SimpleOpType::Abs => return format!("|{}|", lv),
                            SimpleOpType::Sin => return format!("sin({})", lv),
                            SimpleOpType::Cos => return format!("cos({})", lv),
                            SimpleOpType::Tan => return format!("tan({})", lv),
                            SimpleOpType::Sqrt => return format!("sqrt({})", lv),
                            SimpleOpType::Ln => return format!("ln({})", lv),
                            SimpleOpType::Arcsin => return format!("arcsin({})", lv),
                            SimpleOpType::Arccos => return format!("arccos({})", lv),
                            SimpleOpType::Arctan => return format!("arctan({})", lv),
                            SimpleOpType::Parenths => return format!("({})", lv),
                        }
                    },
                    Operation::AdvancedOperation(a) => {
                        match a {
                            AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                                let eexpr = &expr.to_string();
                                let elower_b = &lower_bound.to_string();
                                let eupper_b = &upper_bound.to_string();
                                return format!("I({}, {}, {}, {})", eexpr, in_terms_of, elower_b, eupper_b);
                            },
                            AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                                let eexpr = &expr.to_string();
                                let eat = &at.to_string();
                                return format!("D({}, {}, {})", eexpr, in_terms_of, eat);
                            },
                            AdvancedOperation::Equation { equations } => {
                                let eqs: Vec<String> = equations.iter().map(|e| format!("{}={}", e.0.to_string(), e.1.to_string())).collect();
                                return format!("eq({})", eqs.join(","));
                            }
                        }
                    }
                } 
            }
        }
    }
    pub fn to_latex(&self) -> String {
        self.latex_print()
    }
    pub fn to_latex_at_fun<S: Into<String>>(&self, fun_name: S, fun_inputs: Vec<S>, add_aligner: bool) -> String {
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
                return format!("{}({})", name, inputs_str);
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
                            SimpleOpType::Mult => return format!("{}\\cdot {}", lv, rv),
                            SimpleOpType::Neg => return format!("-{}", lv),
                            SimpleOpType::Div => return format!("\\frac{{{}}}{{{}}}", lv, rv),
                            SimpleOpType::HiddenMult => return format!("{}{}", lv, rv),
                            SimpleOpType::Pow => return format!("{}^{{{}}}", lv, rv),
                            SimpleOpType::Cross => return format!("{}\\times {}", lv, rv),
                            SimpleOpType::Abs => return format!("|{}|", lv),
                            SimpleOpType::Sin => return format!("\\sin{{({})}}", lv),
                            SimpleOpType::Cos => return format!("\\cos{{({})}}", lv),
                            SimpleOpType::Tan => return format!("\\tan{{({})}}", lv),
                            SimpleOpType::Sqrt => return format!("\\sqrt{{{}}}", lv),
                            SimpleOpType::Ln => return format!("\\ln{{({})}}", lv),
                            SimpleOpType::Arcsin => return format!("\\arcsin{{({})}}", lv),
                            SimpleOpType::Arccos => return format!("\\arccos{{({})}}", lv),
                            SimpleOpType::Arctan => return format!("\\arctan{{({})}}", lv),
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
                            AdvancedOperation::Equation { equations } => {
                                let eqs: Vec<String> = equations.iter().map(|e| format!("{}&={}", e.0.to_latex(), e.1.to_latex())).collect();
                                return format!("\\left\\{{ \\begin{{align}}{}\\end{{align}}\\right.", eqs.join("\\"))
                            }
                        }
                    }
                } 
            }
        }
    }
}

///specifies the type of operation for the [SimpleOperation](Operation::SimpleOperation) struct.
///
///This enum only contains simple mathematical operations with a left and right side or a maximum
///of two arguments. For more advanced operations, see [AdvancedOpType].
///
///The order of the enum also represents the reverse order of the operation priority.
#[derive(Debug, PartialEq, Clone)]
pub enum SimpleOpType { 
    ///Add two scalars, vectors, or matrices (a+b)
    Add,
    ///Subtract two scalars, vectors, or matrices (a-b)
    Sub,
    ///Negate a scalar, vector or matrix or expression in parentheses (-(3*4))
    Neg,
    ///Multiply a scalar, vector or matrix with each other (Dotproduct, Matrix multiplication,
    ///Scalar multiplication, ...) (a*b)
    Mult,
    ///Divide two scalars or a vector or matrix with a scalar (a/b)
    Div,
    ///Calculate the cross product using "#" (V1#V2), only works with dim(V) <= 3. When dim(V) < 3
    ///the vector gets augmented with zeros
    Cross,
    ///Hidden multiplication between scalar and variable or parentheses (3a, 5(3+3), (3+5)(2+6))
    HiddenMult,
    ///Take a scalar to the power of another scalar using "^" (a^b)
    Pow,
    ///Index into vector using "?" ([3, 4, 5]?1 = 4)
    Get,
    ///Calculate the sin of a scalar (sin(a))
    Sin,
    ///Calculate the cos of a scalar (cos(a))
    Cos,
    ///Calculate the tan of a scalar (tan(a))
    Tan,
    ///Calculate the absolute value of a scalar or the length of a vector (abs(a))
    Abs,
    ///Calculate the square root of a scalar (sqrt(a))
    Sqrt,
    ///Calculate the natural log of a scalar (ln(a))
    Ln,
    ///Calculate the arcsin of a scalar (arcsin(a))
    Arcsin,
    ///Calculate the arccos of a scalar (arccos(a))
    Arccos,
    ///Calculate the arctan of a scalar (arctan(a))
    Arctan, 
    ///Prioritise expressions in parentheses (3*(5+5))
    Parenths
}

/// specifies the type of operation for the [AdvancedOperation] struct.
///
/// This enum only contains advanced operations with more than 2 arguments. For simple operations,
/// see [SimpleOpType].
#[derive(Clone, Debug, PartialEq)]
pub enum AdvancedOpType {
    ///Calculate the derivative of a function f in respect to n at a value m (D(f, n, m))
    Derivative,
    ///Calculate the integral of a function f in respect to n with the bounds a and b (I(f, n, a, b))
    Integral,
    ///Solve the given equation(s) (eq(e_1, e_2, e_3, ...))
    Equation,
}

///used to specify an operation in a parsed string. It is used together with [AST] to
///construct a AST Tree from a mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    SimpleOperation {
        op_type: SimpleOpType,
        left: AST,
        right: AST,
    },
    AdvancedOperation(AdvancedOperation)
}

/// used to specify an advanced operation for more complex mathematical operatiors, such as
/// functions with more than two inputs.
#[derive(Debug, Clone, PartialEq)]
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
        equations: Vec<(AST, AST)>
    }
}
