use crate::{basetypes::Value, parser::{Binary, OpType}};
use std::{fs, process, usize};

///provides a way of saving a step. A step can either be a: 
///
///- Calculation, specified by the Binary Tree of the calculation, its result and a possible Variable Name in which it is saved.
///- Equation, specified by both the left (left of the =) and the right (right of the =) Binary
///Trees, its results and a possible Variable Name in which the results are saved.
///
///# Example
///```
///let steps: Vec<StepType> = vec![
///     StepType::Calc((BinaryTree, Result, Some("A".to_string())))
///];
///```
#[derive(Debug, Clone)]
pub enum StepType {
    Calc((Binary, Value, Option<String>)),
    Equ((Binary, Binary, Vec<Value>, Option<String>))
}

fn latex_recurse(b: &Binary) -> Result<String, String> {
    match b {
        Binary::Value(b) => return Ok(b.latex_print()),
        Binary::Variable(v) => {
            if v == "pi" {
                return Ok("\\pi".to_string());
            }
            return Ok(v.to_string())
        },
        Binary::Operation(o) => {
            let lv = latex_recurse(&o.left)?;
            let rv = latex_recurse(&o.right)?; 
            match o.op_type {
                OpType::Get => return Ok(format!("{}_{{{}}}", lv, rv)),
                OpType::Add => return Ok(format!("{}+{}", lv, rv)),
                OpType::Sub => return Ok(format!("{}-{}", lv, rv)),
                OpType::Mult => return Ok(format!("{}\\cdot {}", lv, rv)),
                OpType::Neg => return Ok(format!("-{}", lv)),
                OpType::Div => return Ok(format!("\\frac{{{}}}{{{}}}", lv, rv)),
                OpType::HiddenMult => return Ok(format!("{}{}", lv, rv)),
                OpType::Pow => return Ok(format!("{}^{{{}}}", lv, rv)),
                OpType::Cross => return Ok(format!("{}\\times {}", lv, rv)),
                OpType::Abs => return Ok(format!("|{}|", lv)),
                OpType::Sin => return Ok(format!("\\sin{{({})}}", lv)),
                OpType::Cos => return Ok(format!("\\cos{{({})}}", lv)),
                OpType::Tan => return Ok(format!("\\tan{{({})}}", lv)),
                OpType::Sqrt => return Ok(format!("\\sqrt{{{}}}", lv)),
                OpType::Ln => return Ok(format!("\\ln{{({})}}", lv)),
                OpType::Arcsin => return Ok(format!("\\arcsin{{({})}}", lv)),
                OpType::Arccos => return Ok(format!("\\arccos{{({})}}", lv)),
                OpType::Arctan => return Ok(format!("\\arctan{{({})}}", lv)),
                OpType::Parenths => return Ok(format!("\\left({}\\right)", lv))
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

///exports a history of [StepType] to a file named <file_name> with the file type defined
///by export_type (see [ExportType] for further details).
pub fn export(history: Vec<StepType>, file_name: String, export_type: ExportType) {
    let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
    let mut j = 0;
    for s in history {
        match s {
            StepType::Calc(i) => {
                let mut aligner = "&";
                if i.2.is_some() {
                    output_string += &format!("{} &= ", i.2.unwrap());
                    aligner = "";
                }
                let expression = match latex_recurse(&i.0) {
                    Ok(s) => s,
                    Err(_) => return
                };
                let res = i.1.latex_print();
                if expression != res {
                    output_string += &format!("{} {}= {} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, aligner, res, j+1, j+1);
                } else {
                    output_string += &format!("{} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, j+1, j+1);
                }
            },
            StepType::Equ(e) => {
                let left = match latex_recurse(&e.0) {
                    Ok(s) => s,
                    Err(_) => return
                };
                let right = match latex_recurse(&e.1) {
                    Ok(s) => s,
                    Err(_) => return
                };
                output_string += &format!("{} &= {} \\\\\n", left, right);
                if e.2.len() == 0 {
                    output_string += &format!("&\\text{{No solutions found!}} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", j+1, j+1);
                }
                for i in 0..e.2.len() {
                    if e.3.is_some() {
                        output_string += &format!("{}_{{{}}} &= {}", e.3.clone().unwrap(), i, e.2[i].latex_print());
                    } else {
                        output_string += &format!("x_{{{}}} &= {}", i, e.2[i].latex_print());
                    }
                    if i == (e.2.len() as f32/2.).floor() as usize {
                        output_string += &format!(" \\tag{{{}}}\\label{{eq:{}}} ", j+1, j+1);
                    }
                    if i == e.2.len()-1{
                        output_string += "\\\\ \\\\ \n";
                    } else {
                        output_string += "\\\\ \n";
                    }
                } 
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
