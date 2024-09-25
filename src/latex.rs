#[cfg(feature = "output")]
use crate::errors::LatexError;

use crate::{basetypes::AST, Values};

#[cfg(feature = "output")]
pub fn png_from_latex<S: Into<String>>(latex: String, scale: f32, line_color: S) -> Result<Vec<u8>, LatexError> {
    use resvg::{render, tiny_skia::Pixmap, usvg::{Options, Transform, Tree}};

    let svg = svg_from_latex(latex, line_color)?;

    let tree = Tree::from_str(&svg, &Options::default())?;

    let dest_width = (tree.size().width() * scale).ceil();
    let dest_height = (tree.size().height() * scale).ceil();

    let mut pixmap = Pixmap::new(dest_width as u32, dest_height as u32).unwrap();

    render(&tree, Transform::from_row(scale, 0., 0., scale, 0., 0.), &mut pixmap.as_mut());

    Ok(pixmap.encode_png().ok().unwrap())
}

#[cfg(feature = "output")]
pub fn svg_from_latex<S: Into<String>>(latex: String, line_color: S) -> Result<String, LatexError> {
    use mathjax_svg::convert_to_svg;

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
        result: Values,
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
            Step::Fun{term, inputs, name} => return term.to_latex_at_fun(name, inputs.iter().collect(), true)
        }
    }
    pub fn to_latex_inline(&self) -> String {
        match self {
            Step::Calc{term, result, variable_save} => {
                let mut latex = "".to_string();
                if variable_save.is_some() {
                    latex += &format!("{} = ", variable_save.clone().unwrap());
                }
                let expression = term.to_latex();
                let res = result.to_latex();

                if expression != res {
                    latex += &format!("{} = {}", expression, res);
                } else {
                    latex += &format!("{}", expression);
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
