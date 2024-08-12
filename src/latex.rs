use image::ImageFormat;
use mathjax::MathJax;

use crate::{basetypes::{Value, AST}, errors::LatexError};
use std::io::Cursor;

pub fn image_from_latex(latex: String, invert_colors: bool, scale: f32) -> Result<Vec<u8>, LatexError> {
    let renderer = MathJax::new().unwrap();
    let result = renderer.render(latex)?;
    let mut image = result.into_image(scale)?;

    if invert_colors {
        image.invert();
    }

    let mut buffer: Cursor<Vec<u8>> = Cursor::new(vec![]);

    image.write_to(&mut buffer, ImageFormat::Png)?;

    Ok(buffer.into_inner())
}

///provides a way of saving a step. A step can either be a: 
///
///- Calculation, specified by the AST Tree of the calculation, its result and a possible Variable Name in which it is saved.
///- Equation, specified by both the left (left of the =) and the right (right of the =) AST
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
        term: AST,
        result: Value,
        variable_save: Option<String>
    },
    Equ{
        eqs: Vec<(AST, AST)>,
        results: Vec<Value>,
        variable_save: Option<String>
    },
    Fun{
        term: AST,
        inputs: Vec<String>,
        name: String
    }
}

///describes the type of export done by the [export()] function:
///
///- Pdf: Save as one pdf file.
///- Png: Save as consecutive .png images (one image per page).
///- Tex: Save as the generated .tex file.
pub enum ExportType {
    Pdf,
    Tex
}

///exports a history of [Step] to a file named <file_name> with the file type defined
///by export_type (see [ExportType] for further details).
pub fn export_history(history: Vec<Step>, export_type: ExportType) -> Result<Vec<u8>, LatexError> {
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
                let expression = term.to_latex();
                let res = result.to_latex();

                if expression != res {
                    output_string += &format!("{} {}= {} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, aligner, res, j+1, j+1);
                } else {
                    output_string += &format!("{} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, j+1, j+1);
                }
            }, 
            Step::Equ{eqs, results, variable_save} => {
                let mut recursed_eq = vec![];
                for i in &eqs {
                    let left = i.0.to_latex();
                    let right = i.1.to_latex();

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
                        output_string += &format!("{}_{{{}}} &= {}", variable_save.clone().unwrap(), i, results[i].to_latex());
                    } else {
                        output_string += &format!("x_{{{}}} &= {}", i, results[i].to_latex());
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
            Step::Fun{term, inputs, name} => output_string += &term.to_latex_at_fun(name, inputs)
        }
        j += 1;
    }
    output_string += "\\end{align*}\n\\end{document}";

    match export_type {
        ExportType::Pdf => {
            let pdf = tectonic::latex_to_pdf(output_string)?;
            return Ok(pdf.to_vec());
        },
        ExportType::Tex => {
            return Ok(output_string.into_bytes());
        },
    } 
}
