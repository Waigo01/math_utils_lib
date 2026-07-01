use crate::{Values, basetypes::{AST, Operation, SimpleOpType}, errors::LatexError, maths::num_traits::Number};

#[cfg(feature = "output")]
#[cfg_attr(docsrs, doc(cfg(feature = "output")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "output")))]
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
/// let step: Step<f64> = Step::new(parsed_expr, res);
/// # Ok::<(), MathLibError>(())
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Step<N: Number> {
    pub term: AST<N>,
    pub result: Values<N>
}

impl<N: Number> Step<N> {
    /// creates a new step based on a term and the associated results.
    pub fn new(term: AST<N>, result: Values<N>) -> Step<N> {
        return Step { term, result };
    }
    fn as_latex_base(&self, mut with_align: bool, with_equation_number: Option<i32>) -> String {

        let tag_with_label = if let Some(equation_number) = with_equation_number {format!("\\tag{{{}}}\\label{{eq:{}}} \\\\ \\\\ \n", equation_number, equation_number)} else {String::new()};

        let expression = if with_align {self.term.as_latex()} else {self.term.as_latex_inline()};

        let result_expression = if let AST::Operation(ref op) = self.term && let Operation::SimpleOperation{ref op_type, ref right, ..} = **op && *op_type == SimpleOpType::Assign {
            if with_align {with_align = false}
            right.as_latex()
        } else {
            expression.clone()
        };
        
        let aligner = if with_align {"&"} else {""};

        let res = self.result.as_latex();

        let latex = if self.result.len() != 0 && result_expression != res {
            format!("{} {}= {} {}", expression, aligner, res, tag_with_label)
        } else {
            format!("{} {}", expression, tag_with_label)
        };

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
    /// converts a step to a string using basic formatting.
    pub fn as_string(&self) -> String {
        let expression = self.term.as_string();

        let result_expression = if let AST::Operation(ref op) = self.term && let Operation::SimpleOperation{ref op_type, ref right, ..} = **op && *op_type == SimpleOpType::Assign {
            right.as_string()
        } else {
            expression.clone()
        };

        let res = self.result.as_string();

        let output = if self.result.len() != 0 && result_expression != res {
            format!("{} = {}", expression, res)
        } else {
            format!("{}", expression)
        };

        return output;
    }
    /// converts a step to a different number type.
    pub fn into<B: Number + From<N>>(self) -> Step<B> {
        Step { term: self.term.into(), result: self.result.into() }
    }
}

/// describes the type of export done by the [export_history()] function:
///
/// - Pdf: Save as a pdf file (only available with output feature).
/// - Png: Save as the generated .png file (only available with output feature).
/// - Tex: Save as the generated .tex file.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExportType {
    #[cfg(feature = "output")]
    #[cfg_attr(docsrs, doc(cfg(feature = "output")))]
    Pdf,
    #[cfg(feature = "output")]
    #[cfg_attr(docsrs, doc(cfg(feature = "output")))]
    Png,
    Tex,
}

/// exports a history of [Step] to a file named <file_name> with the file type defined
/// by export_type (see [ExportType] for further details).
pub fn export_history<N: Number>(history: Vec<Step<N>>, export_type: ExportType) -> Result<Vec<u8>, LatexError> {
    match export_type {
        #[cfg(feature = "output")]
        ExportType::Pdf => {
            let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage{mathtools}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
            for (i, s) in history.iter().enumerate() {
                output_string += &s.as_latex_with_tag(i as i32+1);
            }
            output_string += "\\end{align*}\n\\end{document}";

            let pdf = tectonic::latex_to_pdf(output_string)?;
            return Ok(pdf.to_vec());
        },
        #[cfg(feature = "output")]
        ExportType::Png => {
            let mut output_string = "\\begin{align*}\n".to_string();
            for (i, s) in history.iter().enumerate() {
                output_string += &s.as_latex_with_tag(i as i32+1);
            }
            output_string += "\\end{align*}";

            let png = png_from_latex(output_string, 1080, "#FFFFFF")?;

            return Ok(png)
        },
        ExportType::Tex => {
            let mut output_string = "\\documentclass[12pt, letterpaper]{article}\n\\usepackage{amsmath}\n\\usepackage{mathtools}\n\\usepackage[margin=1in]{geometry}\n\\allowdisplaybreaks\n\\begin{document}\n\\begin{align*}\n".to_string();
            for (i, s) in history.iter().enumerate() {
                output_string += &s.as_latex_with_tag(i as i32+1);
            }
            output_string += "\\end{align*}\n\\end{document}";

            return Ok(output_string.into_bytes());
        },
    } 
}
