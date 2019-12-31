use serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PointDef {
    #[serde(rename = "x mm")]
    pub x: f64,

    #[serde(rename = "y mm")]
    pub y: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineDef {
    pub start: PointDef,
    pub end: PointDef,
    #[serde(rename = "thickness pt")]
    pub thickness: f64,
    #[serde(rename = "color cmyk")]
    pub color: CmykDef
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaperSize {
    #[serde(rename = "width mm")]
    pub width: f64,

    #[serde(rename = "height mm")]
    pub height: f64
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CmykDef (pub f64, pub f64, pub f64, pub f64);

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum LineSet {
    #[serde(rename = "slant")]
    Slant (SlantLineSet),

    #[serde(rename = "seyes")]
    Seyes (SeyesLineSet),

    #[serde(rename = "single line")]
    SingleLine(LineDef)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeometryDef {
    #[serde(rename = "paper size")]
    pub paper_size: PaperSize,
    #[serde(rename = "line sets")]
    pub line_sets: Vec<LineSet>
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
            assert_eq!(slant_lines.thickness, 0.4);
            assert_eq!(slant_lines.color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The first line set is supposed to be the slant lines.");
        }
    
        if let LineSet::Seyes(seyes_lines) = &gdef.line_sets[1] {
            assert_eq!(seyes_lines.y_spacing, 2.0);
            assert_eq!(seyes_lines.top_margin, 30.0);
            assert_eq!(seyes_lines.bottom_margin, 20.0);
            assert_eq!(seyes_lines.base_thickness, 0.5);
            assert_eq!(seyes_lines.base_color, CmykDef(0.02, 0.34, 0.0, 0.12));
            assert_eq!(seyes_lines.aux_thickness, 0.1);
            assert_eq!(seyes_lines.aux_color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The second line set is supposed to be the seyes (French ruled) lines.");
        }

        if let LineSet::SingleLine(line) = &gdef.line_sets[2] {
            assert_eq!(line.start.x, 30.0);
            assert_eq!(line.start.y, PaperSize::LETTER_PORTRAIT.height);
            assert_eq!(line.end.x, 30.0);
            assert_eq!(line.end.y, 0.0);
            assert_eq!(line.thickness, 0.4);
            assert_eq!(line.color, CmykDef(0.02, 0.34, 0.0, 0.12));
        } else {
            panic!("The third line set is supposed to be a single vertical line.");
        }
    }

    impl PartialEq for CmykDef {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
        }
    }

    impl Eq for CmykDef {}
}