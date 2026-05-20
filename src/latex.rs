#[cfg(feature = "output")]
use crate::errors::LatexError;

use crate::{Values, basetypes::{AST, Operation, SimpleOpType}};

#[cfg(feature = "output")]
/// converts the given latex string to a png image with the given height in pixels, returned as its raw bytes. 
/// This function allows for a change of line color. The line color is defined by a hex string
/// e.g. "#FFFFFF". The background is always transparent.
pub fn png_from_latex<S: Into<String>>(latex: String, height: u32, line_color: S) -> Result<Vec<u8>, LatexError> {
    use resvg::{render, tiny_skia::Pixmap, usvg::{Options, Transform, Tree}};

    let svg = svg_from_latex(latex, line_color)?;

    let tree = Tree::from_str(&svg, &Options::default())?;

    let dest_width = ((tree.size().width()/tree.size().height()) * height as f32).ceil();
    let width_scale = dest_width/tree.size().width();
    let height_scale = height as f32/tree.size().height();

    let mut pixmap = Pixmap::new(dest_width as u32, height as u32).unwrap();

    render(&tree, Transform::from_row(width_scale, 0., 0., height_scale, 0., 0.), &mut pixmap.as_mut());

    Ok(pixmap.encode_png().ok().unwrap())
}

#[cfg(feature = "output")]
/// converts the given latex string to an svg string. The function also takes a line color, which
/// is given as a hex string e.g. "#FFFFFF".
pub fn svg_from_latex<S: Into<String>>(latex: String, line_color: S) -> Result<String, LatexError> {
    use mathjax_svg::convert_to_svg;

    let mut svg = convert_to_svg(latex)?;

    svg = svg.replace("currentColor", &line_color.into());
    
    Ok(svg)
}

/// provides a way of saving a step. A step can either be a: 
///
/// # Example
/// ```
/// # use math_utils_lib::{parse, eval, Step, Context, MathLibError, Value};
/// let parsed_expr = parse("3*3+6^5")?;
/// let res = eval(&parsed_expr, &mut Context::empty())?;
/// let step = Step::new(parsed_expr, res);
/// # Ok::<(), MathLibError>(())
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Step {
    term: AST,
    result: Values
}

impl Step {
    /// creates a new step based on a term and the associated results.
    pub fn new(term: AST, result: Values) -> Step {
        return Step { term, result };
    }
    fn as_latex_base(&self, with_align: bool, with_equation_number: Option<i32>) -> String {
        let aligner = if with_align {"&"} else {""};

        let tag_with_label = if let Some(equation_number) = with_equation_number {format!("\\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", equation_number, equation_number)} else {String::new()};

        let mut latex = "".to_string();
        
        let expression = self.term.as_latex();

        let result_expression = if let AST::Operation(ref op) = self.term && let Operation::SimpleOperation{ref op_type, ref right, ..} = **op && *op_type == SimpleOpType::Assign {
            right.as_latex()
        } else {
            expression.clone()
        };

        let res = self.result.as_latex();

        if self.result.len() != 0 && result_expression != res {
            latex += &format!("{} {}= {} {}", expression, aligner, res, tag_with_label);
        } else {
            latex += &format!("{} {}", expression, tag_with_label);
        }

        return latex;
    }
    /// converts a step to latex with an added equation tag, which number is given by the equation
    /// number. This function also adds a "&" aligner before the "=".
    pub fn as_latex_with_tag(&self, equation_number: i32) -> String {
        return self.as_latex_base(true, Some(equation_number));
    }
    /// converts a step to latex. This function also adds a "&" aligner before the "=".
    pub fn as_latex(&self) -> String {
        return self.as_latex_base(true, None);
    }
    /// converts a step to inline latex (without the "&" aligner).
    pub fn as_latex_inline(&self) -> String {
        return self.as_latex_base(false, None);
    }
}

/// describes the type of export done by the [export()] function:
///
/// - Pdf: Save as a pdf file.
/// - Tex: Save as the generated .tex file.
#[cfg(feature = "output")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExportType {
    Pdf,
    Tex
}

/// exports a history of [Step] to a file named <file_name> with the file type defined
/// by export_type (see [ExportType] for further details).
#[cfg(feature = "output")]
pub fn export_history(history: Vec<Step>, export_type: ExportType) -> Result<Vec<u8>, LatexError> {
    let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
    for (i, s) in history.iter().enumerate() {
        output_string += &s.as_latex_with_tag(i as i32+1);
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
