mod cmd_line;
mod pdf_gen;
mod geometry_def;
mod slant_lines_gen;
mod seyes_lines_gen;
mod horizontal_lines_gen;
mod vertical_lines_gen;

use thiserror::Error;
use geometry_def::{LineDef, GeometryDef, LineSet};

fn main() -> Result<(), Error> {
    let opts = cmd_line::parse_cmd_line()?;

    let gdef = std::fs::File::open(opts.input_yaml)?;
    let gdef: GeometryDef = serde_yaml::from_reader(gdef)?;

    let mut lines: Vec<LineDef> = Vec::new();

    for line_set in gdef.line_sets {
        match line_set {
            LineSet::SingleLine(line) =>
                lines.push(line),
            LineSet::Slant(slant_lines) =>
                slant_lines_gen::create_slant_lines(&slant_lines, &gdef.paper_size, &mut lines)?,
            LineSet::Seyes(seyes_lines) =>
                seyes_lines_gen::create_seyes_lines(&seyes_lines, &gdef.paper_size, &mut lines)?,
            LineSet::HorizontalLines(horiz_lines) =>
                horizontal_lines_gen::create_horizontal_lines(&horiz_lines, &gdef.paper_size, &mut lines)?,
            LineSet::VerticalLines(vert_lines) =>
                vertical_lines_gen::create_vertical_lines(&vert_lines, &gdef.paper_size, &mut lines)?
        }
    }

    pdf_gen::create_pdf(&gdef.paper_size, &lines, &opts.output_pdf)?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid command line parameters.")]
    CmdLineError(#[from] cmd_line::Error),

    #[error("Geometry definition reading error.")]
    GeometryDefReadError(#[from] std::io::Error),

    #[error("Geometry definition parsing error.")]
    GeometryDefParseError(#[from] serde_yaml::Error),

    #[error("Slant line generation error.")]
    SlantLinesGenError(#[from] slant_lines_gen::Error),

    #[error("Seyes line generation error.")]
    SeyesLinesGenError(#[from] seyes_lines_gen::Error),

    #[error("Horizontal line generation error.")]
    HorizontalLinesGenError(#[from] horizontal_lines_gen::Error),

    #[error("Vertical line generation error.")]
    VerticalLinesGenError(#[from] vertical_lines_gen::Error),

    #[error("Error when generating the PDF from the lines.")]
    PdfGenError(#[from] pdf_gen::Error)
}