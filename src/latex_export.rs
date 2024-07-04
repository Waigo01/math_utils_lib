use crate::{basetypes::Value, helpers::round_and_format, parser::{AdvancedOperation, Binary, Operation, SimpleOpType}};
use std::{fs, process, usize};

///provides a way of saving a step. A step can either be a: 
///
///- Calculation, specified by the Binary Tree of the calculation, its result and a possible Variable Name in which it is saved.
///- Equation, specified by both the left (left of the =) and the right (right of the =) Binary
///Trees, its results and a possible Variable Name in which the results are saved. Multiple Tuples
///of Trees specify a system of equations.
///
///# Example
///```
///let steps: Vec<Step> = vec![
///     Step::Calc((BinaryTree, Result, Some("A".to_string())))
///];
///```
#[derive(Debug, Clone)]
pub enum Step {
    Calc{
        term: Binary,
        result: Value,
        variable_save: Option<String>
    },
    Equ{
        eqs: Vec<(Binary, Binary)>,
        results: Vec<Value>,
        variable_save: Option<String>
    },
    Fun{
        term: Binary,
        inputs: Vec<String>,
        name: String
    }
}

enum LatexValue {
    Scalar(f64),
    Vector(Vec<Binary>),
    Matrix(Vec<Vec<Binary>>)
}

fn latex_print(val: LatexValue) -> Result<String, String> {
    match val {
        LatexValue::Scalar(s) => return Ok(round_and_format(s, true)),
        LatexValue::Vector(v) => {
            let mut output_string = "\\begin{pmatrix}".to_string();
            for i in 0..v.len() {
                let latex_vi = latex_recurse(&v[i])?;
                if i != v.len()-1 {
                    output_string += &format!("{}\\\\ ", latex_vi);
                } else {
                    output_string += &latex_vi;
                }
            }
            output_string += "\\end{pmatrix}";
            return Ok(output_string)
        },
        LatexValue::Matrix(m) => {
            let mut output_string = "\\begin{bmatrix}".to_string();
            for i in 0..m.len(){
                let mut row_string = "".to_string();
                for j in 0..m[i].len() {
                    let matrix_mij = latex_recurse(&m[i][j])?;
                    if j != m[i].len()-1 {
                        row_string += &format!("{} & ", matrix_mij);
                    } else {
                        row_string += &format!("{} \\\\", matrix_mij);
                    }
                }
                output_string += &row_string;
            }
            output_string += "\\end{bmatrix}";
            return Ok(output_string);
        }
    }
}

fn latex_recurse(b: &Binary) -> Result<String, String> {
    match b {
        Binary::Scalar(s) => return Ok(latex_print(LatexValue::Scalar(*s))?),
        Binary::Vector(v) => return Ok(latex_print(LatexValue::Vector(*v.clone()))?),
        Binary::Matrix(m) => return Ok(latex_print(LatexValue::Matrix(*m.clone()))?),
        Binary::Variable(v) => {
            if v == "pi" {
                return Ok("\\pi".to_string());
            }
            return Ok(v.to_string())
        },
        Binary::Function { name, inputs } => {
            let mut inputs_str = String::new();
            for (i, inp) in inputs.iter().enumerate() {
                let recursed = latex_recurse(inp)?;
                if i != inputs.len() - 1 {
                    inputs_str += &format!("{}, ", recursed);
                } else {
                    inputs_str += &format!("{}", recursed);
                }
            }
            return Ok(format!("{}({})", name, inputs_str));
        }
        Binary::Operation(o) => {
            match &**o  {
                Operation::SimpleOperation {op_type, left, right} => {
                    let lv = latex_recurse(&left)?;
                    let rv = latex_recurse(&right)?; 
                    match op_type {
                        SimpleOpType::Get => return Ok(format!("{}_{{{}}}", lv, rv)),
                        SimpleOpType::Add => return Ok(format!("{}+{}", lv, rv)),
                        SimpleOpType::Sub => return Ok(format!("{}-{}", lv, rv)),
                        SimpleOpType::Mult => return Ok(format!("{}\\cdot {}", lv, rv)),
                        SimpleOpType::Neg => return Ok(format!("-{}", lv)),
                        SimpleOpType::Div => return Ok(format!("\\frac{{{}}}{{{}}}", lv, rv)),
                        SimpleOpType::HiddenMult => return Ok(format!("{}{}", lv, rv)),
                        SimpleOpType::Pow => return Ok(format!("{}^{{{}}}", lv, rv)),
                        SimpleOpType::Cross => return Ok(format!("{}\\times {}", lv, rv)),
                        SimpleOpType::Abs => return Ok(format!("|{}|", lv)),
                        SimpleOpType::Sin => return Ok(format!("\\sin{{({})}}", lv)),
                        SimpleOpType::Cos => return Ok(format!("\\cos{{({})}}", lv)),
                        SimpleOpType::Tan => return Ok(format!("\\tan{{({})}}", lv)),
                        SimpleOpType::Sqrt => return Ok(format!("\\sqrt{{{}}}", lv)),
                        SimpleOpType::Ln => return Ok(format!("\\ln{{({})}}", lv)),
                        SimpleOpType::Arcsin => return Ok(format!("\\arcsin{{({})}}", lv)),
                        SimpleOpType::Arccos => return Ok(format!("\\arccos{{({})}}", lv)),
                        SimpleOpType::Arctan => return Ok(format!("\\arctan{{({})}}", lv)),
                        SimpleOpType::Parenths => return Ok(format!("\\left({}\\right)", lv)),
                    }
                },
                Operation::AdvancedOperation(a) => {
                    match a {
                        AdvancedOperation::Integral {expr, in_terms_of, lower_bound, upper_bound} => {
                            let eexpr = latex_recurse(&expr)?;
                            let elower_b = latex_recurse(&lower_bound)?;
                            let eupper_b = latex_recurse(&upper_bound)?;
                            return Ok(format!("\\int_{{{}}}^{{{}}}{} d{}", elower_b, eupper_b, eexpr, in_terms_of));
                        },
                        AdvancedOperation::Derivative {expr, in_terms_of, at} => {
                            let eexpr = latex_recurse(&expr)?;
                            let eat = latex_recurse(&at)?;
                            return Ok(format!("\\frac{{\\partial}}{{\\partial {}}}\\left({}\\right)_{{\\text{{at }}{} = {}}}", in_terms_of, eexpr, in_terms_of, eat));
                        } 
                    }
                }
            } 
        }
    }
}

///describes the type of export done by the [export()] function:
///
///- Pdf: Save as one pdf file.
///- Png: Save as consecutive .png images (one image per page).
///- Tex: Save as the generated .tex file.
pub enum ExportType {
    Pdf,
    Png,
    Tex
}

///exports a history of [Step] to a file named <file_name> with the file type defined
///by export_type (see [ExportType] for further details).
pub fn export<S: Into<String>>(history: Vec<Step>, file_name: S, export_type: ExportType) {
    let file_name = file_name.into();
    let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
    let mut j = 0;
    for s in history {
        match s {
            Step::Calc{term, result, variable_save} => {
                let mut aligner = "&";
                if variable_save.is_some() {
                    output_string += &format!("{} &= ", variable_save.unwrap());
                    aligner = "";
                }
                let expression = match latex_recurse(&term) {
                    Ok(s) => s,
                    Err(_) => return
                };
                let res = result.latex_print();

                if expression != res {
                    output_string += &format!("{} {}= {} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, aligner, res, j+1, j+1);
                } else {
                    output_string += &format!("{} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, j+1, j+1);
                }
            }, 
            Step::Equ{eqs, results, variable_save} => {
                let mut recursed_eq = vec![];
                for i in &eqs {
                    let left = match latex_recurse(&i.0) {
                        Ok(s) => s,
                        Err(_) => return
                    };
                    let right = match latex_recurse(&i.1) {
                        Ok(s) => s,
                        Err(_) => return
                    };

                    recursed_eq.push((left, right));
                }
                for i in recursed_eq {
                    output_string += &format!("{} &= {} \\\\ \n", i.0, i.1);
                }
                output_string += "\\\\ \n";
                if results.len() == 0 {
                    output_string += &format!("&\\text{{No solutions found!}} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", j+1, j+1);
                }
                for i in 0..results.len() {
                    if variable_save.is_some() {
                        output_string += &format!("{}_{{{}}} &= {}", variable_save.clone().unwrap(), i, results[i].latex_print());
                    } else {
                        output_string += &format!("x_{{{}}} &= {}", i, results[i].latex_print());
                    }
                    if i == (results.len() as f32/2.).floor() as usize {
                        output_string += &format!(" \\tag{{{}}}\\label{{eq:{}}} ", j+1, j+1);
                    }
                    if i == results.len()-1{
                        output_string += "\\\\ \\\\ \n";
                    } else {
                        output_string += "\\\\ \n";
                    }
                } 
            },
            Step::Fun{term, inputs, name} => {
                let recursed_fn = match latex_recurse(&term) {
                    Ok(s) => s,
                    Err(_) => return
                }; 

                let mut inputs_str = String::new();
                for (i, inp) in inputs.iter().enumerate() {
                    if i != inputs.len()-1 {
                        inputs_str += &format!("{}, ", inp);
                    } else {
                        inputs_str += &format!("{}", inp);
                    }
                }

                output_string += &format!("{}({}) &= {} \\\\ \n", name, inputs_str, recursed_fn);
            }
        }
        j += 1;
    }
    output_string += "\\end{align*}\n\\end{document}";
    let _ = fs::create_dir(format!("./temp{}", file_name));
    let _ = fs::write(format!("./temp{}/main.tex", file_name), output_string);

    match export_type {
        ExportType::Pdf => {
            let _ = process::Command::new("/usr/bin/pdflatex").arg("./main.tex").current_dir(format!("./temp{}", file_name)).output();
            let _ = process::Command::new("/usr/bin/pdflatex").arg("./main.tex").current_dir(format!("./temp{}", file_name)).output();
            let _ = fs::copy(format!("./temp{}/main.pdf", file_name), format!("./{}.pdf", file_name));
            let _ = process::Command::new("rm").args(["-r", &format!("./temp{}", file_name)]).output();
        },
        ExportType::Tex => {
            let _ = fs::copy(format!("./temp{}/main.tex", file_name), format!("./{}.tex", file_name));
            let _ = process::Command::new("rm").args(["-r", &format!("./temp{}", file_name)]).output();
        },
        ExportType::Png => {
            let _ = process::Command::new("/usr/bin/pdflatex").arg("./main.tex").current_dir(format!("./temp{}", file_name)).output();
            let _ = process::Command::new("/usr/bin/pdflatex").arg("./main.tex").current_dir(format!("./temp{}", file_name)).output();
            let _ = process::Command::new("pdftoppm").args(["./main.pdf", &format!("{}", file_name), "-png"]).current_dir(format!("./temp{}", file_name)).output();
            let read_dir = match fs::read_dir(format!("./temp{}", file_name)) {
                Ok(s) => s,
                Err(_) => {return;}
            };
            for entry in read_dir {
                let path = match entry {
                    Ok(s) => s.path(),
                    Err(_) => {return;}
                };
                if path.to_str().unwrap().split(".").nth(2).unwrap() == "png" {
                    let _ = fs::copy(path.clone(), format!("./{}", path.to_str().unwrap().split("/").nth(2).unwrap()));
                }
            }
            let _ = process::Command::new("rm").args(["-r", &format!("./temp{}", file_name)]).output();
        }
    } 
}
