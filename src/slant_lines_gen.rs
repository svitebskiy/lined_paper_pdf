use crate::geometry_def::{LineDef, PointDef, PaperSize, SlantLineSet};
use thiserror::Error;

pub fn create_slant_lines(line_set: &SlantLineSet, paper_size: &PaperSize, result: &mut Vec<LineDef>)
    -> Result<(), Error>
{

    if line_set.slant_angle < 45.0 && line_set.slant_angle > 90.0 {
        return Err(Error::SlantAngleIsOutOfRange {actual: line_set.slant_angle, min: 45.0, max: 90.0});
    }

    if line_set.x_spacing <= 0.0 {
        return Err(Error::LineSpacingIsNotPositive(line_set.x_spacing));
    }

    if paper_size.width <= 0.0 {
        return Err(Error::PaperWidthIsNotPositive(paper_size.width));
    }

    if paper_size.height <= 0.0 {
        return Err(Error::PaperHeightIsNotPositive(paper_size.height));
    }

    let slant_angle = line_set.slant_angle.to_radians();
    let tan_a = slant_angle.tan();

    // First line starts at the left size, on upper edge
    let mut x0 = line_set.x_spacing;
    let mut y0 = paper_size.height;

    let mut add_line_to_result = |x0, y0, x1, y1| {
        result.push(LineDef {
            start: PointDef {x: x0, y: y0},
            end: PointDef {x: x1, y: y1},
            thickness: line_set.thickness,
            color: line_set.color,
            dash_pattern: None
        });
    };

    // Advance the starting point right along the upper edge until past the right edge:
    loop {
        if x0 > paper_size.width {
            y0 = paper_size.height - (x0 - paper_size.width) * tan_a;
            x0 = paper_size.width;
            break;
        }

        let mut x1 = x0 - paper_size.height / tan_a;
        let mut y1 = 0.0;
        if x1 < 0.0 {
            x1 = 0.0;
            y1 = paper_size.height - x0 * tan_a;
        }
  
        add_line_to_result(x0, y0, x1, y1);
        x0 += line_set.x_spacing;     
    }

    let line_y_spacing = line_set.x_spacing * tan_a;
    assert!(line_y_spacing > 0.0);

    // Advance the starting point down the right edge until past the lower edge:
    loop {
        if y0 < 0.0 {
            break;
        }

        let mut x1 = x0 - (y0 / tan_a);
        let mut y1 = 0.0;
    
        if x1 < 0.0 {
            x1 = 0.0;
            y1 = y0 - paper_size.width * tan_a;
        }

        add_line_to_result(x0, y0, x1, y1);
        y0 -= line_y_spacing;
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Slant angle of {actual} is out of range. It must be between {min} and {max}.")]
    SlantAngleIsOutOfRange {actual: f64, min: f64, max: f64},

    #[error("Line spacing of {0} is not a positive number.")]
    LineSpacingIsNotPositive(f64),

    #[error("Paper witdth of {0} is not a positive number.")]
    PaperWidthIsNotPositive(f64),

    #[error("Paper height of {0} is not a positive number.")]
    PaperHeightIsNotPositive(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry_def::CmykDef;

    const LINE_X_SPACING: f64 = 5.0;


    #[test]
    fn line_up_high_slant_in_portrait_mode() {
        let paper_size = &PaperSize::LETTER_PORTRAIT;
        let mut result: Vec<LineDef> = Vec::new();
        create_slant_lines(
            &SlantLineSet { slant_angle: 46.0, x_spacing: LINE_X_SPACING, thickness: 0.4, color: CmykDef::BLACK},
            paper_size, &mut result)
            .unwrap();
        assert!(result.len() > 2);
        assert_eq!(result[0].end.x, 0.0);
        assert_eq!(result.last().unwrap().end.y, 0.0);
        for line in result.iter() {
            check_line(line, paper_size);
        }
    }

    #[test]
    fn line_up_low_slant_in_landscape_mode() {
        let paper_size = &PaperSize::LETTER_LANDSCAPE;
        let mut result: Vec<LineDef> = Vec::new();
        create_slant_lines(
            &SlantLineSet { slant_angle: 60.0, x_spacing: LINE_X_SPACING, thickness: 0.4, color: CmykDef::BLACK},
            paper_size, &mut result)
            .unwrap();
        assert!(result.len() > 2);
        assert_eq!(result[0].end.x, 0.0);
        assert_eq!(result[result.len() / 2].start.y, paper_size.height);
        assert_eq!(result[result.len() / 2].end.y, 0.0);
        assert!(result[result.len() / 2].start.x > 0.0);
        assert!(result[result.len() / 2].end.x > 0.0);
        assert!(result[result.len() / 2].end.x < result[result.len() / 2].start.x);
        assert_eq!(result.last().unwrap().end.y, 0.0);
        for line in result.iter() {
            check_line(line, paper_size);
        }
    }

    fn check_line(line: &LineDef, paper_size: &PaperSize) {
        let x0 = line.start.x;
        let y0 = line.start.y;
        let x1 = line.end.x;
        let y1 = line.end.y;
        assert!(0.0 <= x0);
        assert!(x0 <= paper_size.width);
        assert!(0.0 <= x1);
        assert!(x1 <= paper_size.width);
        assert!(0.0 <= y0);
        assert!(y0 <= paper_size.height, "y0 {} <= paper height {}", y0, paper_size.height);
        assert!(0.0 <= y1, "0.0 <= y1 {}", y1);
        assert!(y1 <= paper_size.height);
    }
}