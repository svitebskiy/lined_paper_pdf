pub mod coord;

use serde::{self, Deserialize};
use coord::Coord;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct PointDef {
    #[serde(rename = "x mm")]
    pub x: Coord,

    #[serde(rename = "y mm")]
    pub y: Coord
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct DashPatternDef {
    pub dash: i64,
    pub gap: Option<i64>
}

#[derive(Deserialize, Debug)]
pub struct LineDef {
    pub start: PointDef,
    pub end: PointDef,
    #[serde(rename = "thickness pt")]
    pub thickness: f64,
    #[serde(rename = "color cmyk")]
    pub color: CmykDef,
    #[serde(rename = "dash pattern")]
    pub dash_pattern: Option<DashPatternDef>
}

#[derive(Deserialize, Debug)]
pub struct PaperSize {
    #[serde(rename = "width mm")]
    pub width: f64,

    #[serde(rename = "height mm")]
    pub height: f64
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct CmykDef (pub f64, pub f64, pub f64, pub f64);

#[derive(Deserialize, Debug)]
pub struct SlantLineSet {
    #[serde(rename = "x spacing mm")]
    pub x_spacing: f64,
    #[serde(rename = "slant angle deg")]
    pub slant_angle: f64,
    #[serde(rename = "thickness pt")]
    pub thickness: f64,
    #[serde(rename = "color cmyk")]
    pub color: CmykDef
}

#[derive(Deserialize, Debug)]
pub struct SeyesLineSet {
    #[serde(rename = "y spacing mm")]
    pub y_spacing: f64,
    #[serde(rename = "top margin mm")]
    pub top_margin: f64,
    #[serde(rename = "bottom margin mm")]
    pub bottom_margin: f64,
    #[serde(rename = "base thickness pt")]
    pub base_thickness: f64,
    #[serde(rename = "base color cmyk")]
    pub base_color: CmykDef,
    #[serde(rename = "aux thickness pt")]
    pub aux_thickness: f64,
    #[serde(rename = "aux color cmyk")]
    pub aux_color: CmykDef
}

#[derive(Deserialize, Debug)]
pub struct HorizontalLineSet {
    #[serde(rename = "y spacing mm")]
    pub y_spacing: f64,
    #[serde(rename = "top margin mm")]
    pub top_margin: f64,
    #[serde(rename = "bottom margin mm")]
    pub bottom_margin: f64,
    #[serde(rename = "thickness pt")]
    pub thickness: f64,
    #[serde(rename = "color cmyk")]
    pub color: CmykDef,
    #[serde(rename = "dash pattern")]
    pub dash_pattern: Option<DashPatternDef>  
}

#[derive(Deserialize, Debug)]
pub struct VerticalLineSet {
    #[serde(rename = "x spacing mm")]
    pub x_spacing: f64,
    #[serde(rename = "left margin mm")]
    pub left_margin: f64,
    #[serde(rename = "right margin mm")]
    pub right_margin: f64,
    #[serde(rename = "thickness pt")]
    pub thickness: f64,
    #[serde(rename = "color cmyk")]
    pub color: CmykDef,
    #[serde(rename = "dash pattern")]
    pub dash_pattern: Option<DashPatternDef>
}

#[derive(Deserialize, Debug)]
pub enum LineSet {
    #[serde(rename = "slant")]
    Slant (SlantLineSet),

    #[serde(rename = "seyes")]
    Seyes (SeyesLineSet),

    #[serde(rename = "horizontal lines")]
    HorizontalLines (HorizontalLineSet),

    #[serde(rename = "vertical lines")]
    VerticalLines (VerticalLineSet),

    #[serde(rename = "single line")]
    SingleLine (LineDef)
}

#[derive(Deserialize, Debug)]
pub struct GeometryDef {
    #[serde(rename = "paper size")]
    pub paper_size: PaperSize,
    #[serde(rename = "line sets")]
    pub line_sets: Vec<LineSet>
}

impl PointDef {
    pub fn x_coord(&self, paper_size: &PaperSize) -> f64 {
        match self.x {
            Coord::OffZero(v) => v,
            Coord::OffFarEdge(v) => paper_size.width - v
        }
    }

    pub fn y_coord(&self, paper_size: &PaperSize) -> f64 {
        match self.y {
            Coord::OffZero(v) => v,
            Coord::OffFarEdge(v) => paper_size.height - v
        }
    }
}

#[cfg(test)]
impl PaperSize {
    pub const LETTER_PORTRAIT: PaperSize = PaperSize { width: 215.9, height: 279.4 };
    pub const LETTER_LANDSCAPE: PaperSize = PaperSize { width: 215.9, height: 279.4 };
}

#[cfg(test)]
impl CmykDef {
    pub const BLACK: CmykDef = CmykDef(0.0, 0.0, 0.0, 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_seyes_slant52_yml() {
        let yml = fs::read_to_string("test_line_defs/letter_seyes_slant52.yml").unwrap();
        let gdef: GeometryDef = serde_yaml::from_str(&yml).unwrap();

        assert_eq!(gdef.paper_size.width, PaperSize::LETTER_PORTRAIT.width);
        assert_eq!(gdef.paper_size.height, PaperSize::LETTER_PORTRAIT.height);

        assert_eq!(gdef.line_sets.len(), 3);

        if let LineSet::Slant(slant_lines) = &gdef.line_sets[0] {
            assert_eq!(slant_lines.x_spacing, 10.0);
            assert_eq!(slant_lines.slant_angle, 52.0);
            assert_eq!(slant_lines.thickness, 0.1);
            assert_eq!(slant_lines.color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The first line set is supposed to be the slant lines.");
        }
    
        if let LineSet::Seyes(seyes_lines) = &gdef.line_sets[1] {
            assert_eq!(seyes_lines.y_spacing, 2.0);
            assert_eq!(seyes_lines.top_margin, 30.0);
            assert_eq!(seyes_lines.bottom_margin, 20.0);
            assert_eq!(seyes_lines.base_thickness, 0.4);
            assert_eq!(seyes_lines.base_color, CmykDef(0.02, 0.34, 0.0, 0.12));
            assert_eq!(seyes_lines.aux_thickness, 0.1);
            assert_eq!(seyes_lines.aux_color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The second line set is supposed to be the seyes (French ruled) lines.");
        }

        if let LineSet::SingleLine(line) = &gdef.line_sets[2] {
            assert_eq!(line.start.x_coord(&gdef.paper_size), 30.0);
            assert_eq!(line.start.y_coord(&gdef.paper_size), PaperSize::LETTER_PORTRAIT.height);
            assert_eq!(line.end.x_coord(&gdef.paper_size), 30.0);
            assert_eq!(line.end.y_coord(&gdef.paper_size), 0.0);
            assert_eq!(line.thickness, 0.4);
            assert_eq!(line.color, CmykDef(0.0, 0.36, 0.26, 0.04));
        } else {
            panic!("The third line set is supposed to be a single vertical line.");
        }
    }

    #[test]
    fn parse_5mm_square_yml() {
        let yml = fs::read_to_string("test_line_defs/letter_5mm_square.yml").unwrap();
        let gdef: GeometryDef = serde_yaml::from_str(&yml).unwrap();

        assert_eq!(gdef.paper_size.width, PaperSize::LETTER_PORTRAIT.width);
        assert_eq!(gdef.paper_size.height, PaperSize::LETTER_PORTRAIT.height);

        assert_eq!(gdef.line_sets.len(), 3);

        if let LineSet::HorizontalLines(h_lines) = &gdef.line_sets[0] {
            assert_eq!(h_lines.y_spacing, 5.0);
            assert_eq!(h_lines.thickness, 0.1);
            assert_eq!(h_lines.top_margin, 30.0);
            assert_eq!(h_lines.bottom_margin, 20.0);
            assert_eq!(h_lines.color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The first line set is supposed to be the horizontal lines.");
        }

        if let LineSet::VerticalLines(v_lines) = &gdef.line_sets[1] {
            assert_eq!(v_lines.x_spacing, 5.0);
            assert_eq!(v_lines.thickness, 0.1);
            assert_eq!(v_lines.left_margin, 30.0);
            assert_eq!(v_lines.right_margin, 20.0);
            assert_eq!(v_lines.color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The second line set is supposed to be the horizontal lines.");
        }
 
        if let LineSet::SingleLine(line) = &gdef.line_sets[2] {
            assert_eq!(line.start.x_coord(&gdef.paper_size), 30.0);
            assert_eq!(line.start.y_coord(&gdef.paper_size), PaperSize::LETTER_PORTRAIT.height);
            assert_eq!(line.end.x_coord(&gdef.paper_size), 30.0);
            assert_eq!(line.end.y_coord(&gdef.paper_size), 0.0);
            assert_eq!(line.thickness, 0.4);
            assert_eq!(line.color, CmykDef(0.0, 0.36, 0.26, 0.04));
        } else {
            panic!("The third line set is supposed to be a single vertical line.");
        }
   }

   #[test]
   fn parse_letter_dashed_6mm() {
       let yml = fs::read_to_string("test_line_defs/letter_dashed_6mm.yml").unwrap();
       let gdef: GeometryDef = serde_yaml::from_str(&yml).unwrap();

       assert_eq!(gdef.paper_size.width, PaperSize::LETTER_PORTRAIT.width);
       assert_eq!(gdef.paper_size.height, PaperSize::LETTER_PORTRAIT.height);

       if let LineSet::HorizontalLines(h_lines) = &gdef.line_sets[0] {
            assert_eq!(h_lines.y_spacing, 6.0);
            assert_eq!(h_lines.thickness, 0.8);
            assert_eq!(h_lines.top_margin, 30.0);
            assert_eq!(h_lines.bottom_margin, 20.0);
            assert_eq!(h_lines.color, CmykDef(0.02, 0.34, 0.0, 0.12));

            match h_lines.dash_pattern {
                Some(dp) => {
                    assert_eq!(dp.dash, 0);
                    assert_eq!(dp.gap.unwrap(), 4);
                },
                None => panic!("Expecting dash pattern in the horizontal lines.")
            }
        } else {
            panic!("The first line set is supposed to be the horizontal lines.");
        }

        if let LineSet::SingleLine(s_line) = &gdef.line_sets[1] {
            match s_line.dash_pattern {
                Some(dp) => {
                    assert_eq!(dp.dash, 2);
                    assert_eq!(dp.gap.unwrap(), 2);
                },
                None => panic!("Expecting dash pattern in the horizontal lines.")
            }
        } else {
            panic!("The second line set is supposed to be a single line.");
        }
    }

    impl PartialEq for CmykDef {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
        }
    }

    impl Eq for CmykDef {}
}