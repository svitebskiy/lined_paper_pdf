use crate::geometry_def::{LineDef, PointDef, PaperSize, VerticalLineSet};
use thiserror::Error;

pub fn create_vertical_lines(line_set: &VerticalLineSet, paper_size: &PaperSize, result: &mut Vec<LineDef>)
    -> Result<(), Error>
{
    if paper_size.width <= 0.0 {
        return Err(Error::PaperWidthIsNotPositive(paper_size.width));
    }

    if paper_size.height <= 0.0 {
        return Err(Error::PaperHeightIsNotPositive(paper_size.height));
    }

    if line_set.x_spacing <= 0.0 {
        return Err(Error::XSpacingIsNotPositive(line_set.x_spacing));
    }

    if line_set.left_margin <= 0.0 {
        return Err(Error::LeftMarginIsNotPositive(line_set.left_margin));
    }

    if line_set.right_margin <= 0.0 {
        return Err(Error::RightMarginIsNotPositive(line_set.right_margin));
    }

    let mut x = line_set.left_margin;

    while x <= paper_size.width - line_set.right_margin {
        result.push(LineDef {
            start: PointDef { x: x, y: 0.0 },
            end: PointDef { x: x, y: paper_size.height },
            thickness: line_set.thickness,
            color: line_set.color,
            dash_pattern: line_set.dash_pattern
        });
        x += line_set.x_spacing;
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Paper witdth of {0} is not a positive number.")]
    PaperWidthIsNotPositive(f64),

    #[error("Paper height of {0} is not a positive number.")]
    PaperHeightIsNotPositive(f64),

    #[error("X spacing of {0} is not a positive number.")]
    XSpacingIsNotPositive(f64),

    #[error("Left margin of {0} is not a positive number.")]
    LeftMarginIsNotPositive(f64),

    #[error("Right margin of {0} is not a positive number.")]
    RightMarginIsNotPositive(f64),
}