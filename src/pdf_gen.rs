use crate::geometry_def::{PaperSize, LineDef};
use std::path::Path;
use thiserror::Error;
use printpdf::{PdfDocument, Mm, Point, Line, LineCapStyle, Color, Cmyk};

pub fn create_pdf(paper_size: &PaperSize, lines: &[LineDef], pdf_path: &Path) -> Result<(), Error> {
    let (doc, page_idx, layer_idx) = PdfDocument::new("Test page", Mm(paper_size.width), Mm(paper_size.height), "Layer 1");
    let layer = doc.get_page(page_idx).get_layer(layer_idx);

    // Before adding the line, we may need to set the stroke type/color/thickness/etc
    layer.set_line_cap_style(LineCapStyle::Round);

    for line in lines {
        layer.set_outline_thickness(line.thickness); // In pts, 0 is a special value for exactly 1 device px
        layer.set_outline_color(Color::Cmyk(Cmyk::new(
            line.color.0, line.color.1, line.color.2, line.color.3, None))); // 1.0 = 100%
    
        let points = vec![
            (Point::new(Mm(line.start.x), Mm(line.start.y)), false),
            (Point::new(Mm(line.end.x), Mm(line.end.y)), false)
        ];

        let line = Line {
            points: points,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false
        };
 
        layer.add_shape(line);
    }

    let file = std::fs::File::create(pdf_path)?;
    let mut writer = std::io::BufWriter::new(file);
    doc.save(&mut writer)?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error when constructing or saving the PDF.")]
    PdfError(#[from] printpdf::errors::Error),

    #[error("File system I/O error.")]
    IOError(#[from] std::io::Error)
}