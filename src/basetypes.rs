use std::usize;

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
///     Variable{
///         name: "pi".to_string(),
///         value: Value::Scalar(3.14159)
///     },
///];
///```
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value
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
    ///returns the scalar if the Value is a scalar and 0 if it is a matrix or a
    ///vector (used in [find_roots()](fn@crate::roots::find_roots)).
    pub fn get_scalar(&self) -> f64 {
        match self {
            Value::Scalar(a) => return *a,
            Value::Matrix(_) => return 0.,
            Value::Vector(_) => return 0.
        }
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
    ///provides a more elegant way of converting a Value to a string using unicode
    ///characters for matrices and vectors.
    ///
    ///This function also takes a var_name option which can be set to print the value with a
    ///"<var_name> = " in front of it.
    pub fn pretty_print(&self, mut var_name: Option<String>) -> String {
        if let Some(ref v) = var_name {
            for i in VAR_SYMBOLS {
                if v == i.0 {
                    var_name = Some(i.1.to_string());
                    break;
                }
            }
        }
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
    ///provides a way to print Values in LaTeX form, using amsmaths' pmatrix and
    ///bmatrix for vectors and matrices.
    pub fn latex_print(&self) -> String {
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
