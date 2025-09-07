use std::fs;
use std::error::Error;
use comrak::{markdown_to_html, ComrakOptions};
use printpdf::*;
use std::io::BufWriter;

fn convert_markdown_to_pdf(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read markdown content
    let markdown_content = fs::read_to_string(input_path)?;

    // Convert markdown to HTML
    let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());

    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new("Markdown PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Create text object
    let font = doc.add_external_font(std::io::Cursor::new(include_bytes!("../assets/fonts/MonaspaceRadonFrozen.ttf")))?;
    let text = html_content.replace("<p>", "").replace("</p>", "\n");

    current_layer.use_text(text, 14.0, Mm(10.0), Mm(287.0), &font);

    // Save PDF
    let file = fs::File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;

    Ok(())
}

fn main() {}