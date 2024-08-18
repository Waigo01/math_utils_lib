#[cfg(feature = "output")]
use gdk_pixbuf::{gio::{Cancellable, MemoryInputStream}, glib::Bytes, Pixbuf};
#[cfg(feature = "output")]
use mathjax_svg::convert_to_svg;
#[cfg(feature = "output")]
use usvg::{Options, Tree};

#[cfg(feature = "output")]
use crate::errors::LatexError;

use crate::basetypes::{Value, AST};

#[cfg(feature = "output")]
pub fn png_from_latex<S: Into<String>>(latex: String, scale: f32, line_color: S) -> Result<Vec<u8>, LatexError> {
    let svg = svg_from_latex(latex, line_color)?;

    let tree = Tree::from_str(&svg, &Options::default())?;

    let dest_width = tree.size().width() * scale;
    let dest_height = tree.size().height() * scale;

    let input_stream = MemoryInputStream::from_bytes(&Bytes::from_owned(svg));

    let pixbuf = Pixbuf::from_stream_at_scale(&input_stream, dest_width as i32, dest_height as i32, true, None::<&Cancellable>)?;

    Ok(pixbuf.save_to_bufferv("png", &[])?)
}

#[cfg(feature = "output")]
pub fn svg_from_latex<S: Into<String>>(latex: String, line_color: S) -> Result<String, LatexError> {
    let mut svg = convert_to_svg(latex)?;

    svg = svg.replace("currentColor", &line_color.into());
    
    Ok(svg)
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

impl Step {
    pub fn to_latex_with_tag(&self, equation_number: i32) -> String {
        match self {
            Step::Calc{term, result, variable_save} => {
                let mut aligner = "&";
                let mut latex = "".to_string();
                if variable_save.is_some() {
                    latex += &format!("{} &= ", variable_save.clone().unwrap());
                    aligner = "";
                }
                let expression = term.to_latex();
                let res = result.to_latex();

                if expression != res {
                    latex += &format!("{} {}= {} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, aligner, res, equation_number, equation_number);
                } else {
                    latex += &format!("{} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", expression, equation_number, equation_number);
                }

                return latex;
            }, 
            Step::Equ{eqs, results, variable_save} => {
                let mut recursed_eq = vec![];
                let mut latex = "".to_string();
                for i in eqs {
                    let left = i.0.to_latex();
                    let right = i.1.to_latex();

                    recursed_eq.push((left, right));
                }
                for i in recursed_eq {
                    latex += &format!("{} &= {} \\\\ \n", i.0, i.1);
                }
                latex += "\\\\ \n";
                if results.len() == 0 {
                    latex += &format!("&\\text{{No solutions found!}} \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", equation_number, equation_number);
                }
                for i in 0..results.len() {
                    if variable_save.is_some() {
                        latex += &format!("{}_{{{}}} &= {}", variable_save.clone().unwrap(), i, results[i].to_latex());
                    } else {
                        latex += &format!("x_{{{}}} &= {}", i, results[i].to_latex());
                    }
                    if i == (results.len() as f32/2.).floor() as usize {
                        latex += &format!(" \\tag{{{}}}\\label{{eq:{}}} ", equation_number, equation_number);
                    }
                    if i == results.len()-1{
                        latex += "\\\\ \\\\ \n";
                    } else {
                        latex += "\\\\ \n";
                    }
                }

                return latex;
            },
            Step::Fun{term, inputs, name} => {
                return term.to_latex_at_fun(name, inputs.iter().collect(), true) + &format!(" \\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", equation_number, equation_number);
            }
        }
    }
    pub fn to_latex(&self) -> String {
        match self {
            Step::Calc{term, result, variable_save} => {
                let mut aligner = "&";
                let mut latex = "".to_string();
                if variable_save.is_some() {
                    latex += &format!("{} &= ", variable_save.clone().unwrap());
                    aligner = "";
                }
                let expression = term.to_latex();
                let res = result.to_latex();

                if expression != res {
                    latex += &format!("{} {}= {}", expression, aligner, res);
                } else {
                    latex += &format!("{}", expression);
                }

                return latex;
            }, 
            Step::Equ{eqs, results, variable_save} => {
                let mut recursed_eq = vec![];
                let mut latex = "".to_string();
                for i in eqs {
                    let left = i.0.to_latex();
                    let right = i.1.to_latex();

                    recursed_eq.push((left, right));
                }
                for i in recursed_eq {
                    latex += &format!("{} &= {} \\\\ \n", i.0, i.1);
                }
                latex += "\\\\ \n";
                if results.len() == 0 {
                    latex += &format!("&\\text{{No solutions found!}}");
                }
                for i in 0..results.len() {
                    if variable_save.is_some() {
                        latex += &format!("{}_{{{}}} &= {}", variable_save.clone().unwrap(), i, results[i].to_latex());
                    } else {
                        latex += &format!("x_{{{}}} &= {}", i, results[i].to_latex());
                    }
                    if i == results.len()-1{
                        latex += "\\\\ \\\\ \n";
                    } else {
                        latex += "\\\\ \n";
                    }
                }

                return latex;
            },
            Step::Fun{term, inputs, name} => return term.to_latex_at_fun(name, inputs.iter().collect(), true)
        }
    }
}

///describes the type of export done by the [export()] function:
///
///- Pdf: Save as one pdf file.
///- Png: Save as consecutive .png images (one image per page).
///- Tex: Save as the generated .tex file.
#[cfg(feature = "output")]
pub enum ExportType {
    Pdf,
    Tex
}

///exports a history of [Step] to a file named <file_name> with the file type defined
///by export_type (see [ExportType] for further details).
#[cfg(feature = "output")]
pub fn export_history(history: Vec<Step>, export_type: ExportType) -> Result<Vec<u8>, LatexError> {
    let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
    for (i, s) in history.iter().enumerate() {
        output_string += &s.to_latex_with_tag(i as i32+1);
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
