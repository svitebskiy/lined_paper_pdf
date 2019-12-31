use crate::geometry_def::{LineDef, PointDef, PaperSize, SeyesLineSet, CmykDef};
use thiserror::Error;

pub fn create_seyes_lines(line_set: &SeyesLineSet, paper_size: &PaperSize, result: &mut Vec<LineDef>)
    -> Result<(), Error>
{
    if paper_size.width <= 0.0 {
        return Err(Error::PaperWidthIsNotPositive(paper_size.width));
    }

    if paper_size.height <= 0.0 {
        return Err(Error::PaperHeightIsNotPositive(paper_size.height));
    }

    if line_set.y_spacing <= 0.0 {
        return Err(Error::YSpacingIsNotPositive(line_set.y_spacing));
    }

    if line_set.top_margin <= 0.0 {
        return Err(Error::TopMarginIsNotPositive(line_set.top_margin));
    }

    if line_set.bottom_margin <= 0.0 {
        return Err(Error::BottomMarginIsNotPositive(line_set.bottom_margin));
    }

    let mut add_line = |y, thickness, color: &CmykDef| {
        result.push(LineDef {
            start: PointDef { x: 0.0, y: y },
            end: PointDef { x: paper_size.width, y: y },
            thickness: thickness,
            color: *color
        });
    };

    let mut y = paper_size.height - line_set.top_margin;
    if y >= line_set.bottom_margin {
        add_line(y, line_set.aux_thickness, &line_set.aux_color);
    }
    y -= line_set.y_spacing;
    if y >= line_set.bottom_margin {
        add_line(y, line_set.aux_thickness, &line_set.aux_color);
    }

    while y + (4.0 * line_set.y_spacing) >= line_set.bottom_margin {
        y -= line_set.y_spacing;
        add_line(y, line_set.aux_thickness, &line_set.aux_color);
        y -= line_set.y_spacing;
        add_line(y, line_set.base_thickness, &line_set.base_color);
        y -= line_set.y_spacing;
        add_line(y, line_set.aux_thickness, &line_set.aux_color);
        y -= line_set.y_spacing;
        add_line(y, line_set.aux_thickness, &line_set.aux_color);
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Paper witdth of {0} is not a positive number.")]
    PaperWidthIsNotPositive(f64),

    #[error("Paper height of {0} is not a positive number.")]
    PaperHeightIsNotPositive(f64),

    #[error("Y spacing of {0} is not a positive number.")]
    YSpacingIsNotPositive(f64),

    #[error("Top margin of {0} is not a positive number.")]
    TopMarginIsNotPositive(f64),

    #[error("Bottom margin of {0} is not a positive number.")]
    BottomMarginIsNotPositive(f64),
}