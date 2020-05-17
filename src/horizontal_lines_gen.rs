use crate::geometry_def::{LineDef, PointDef, PaperSize, HorizontalLineSet};
use crate::geometry_def::coord::Coord;
use thiserror::Error;

pub fn create_horizontal_lines(line_set: &HorizontalLineSet, paper_size: &PaperSize,
    result: &mut Vec<LineDef>)
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

    let mut y = paper_size.height - line_set.top_margin;

    while y >= line_set.bottom_margin {
        result.push(LineDef {
            start: PointDef { x: Coord::OffZero(0.0), y: Coord::OffZero(y) },
            end: PointDef { x: Coord::OffFarEdge(0.0), y: Coord::OffZero(y) },
            thickness: line_set.thickness,
            color: line_set.color,
            dash_pattern: line_set.dash_pattern
        });
        y -= line_set.y_spacing;
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Paper width of {0} is not a positive number.")]
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