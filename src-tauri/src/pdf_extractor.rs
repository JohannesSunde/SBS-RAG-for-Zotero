use std::path::Path;
use std::error::Error;

#[derive(Debug)]
pub struct PageText {
    pub page_num: u32,
    pub text: String,
}

pub struct PdfExtractor;

impl PdfExtractor {
    pub fn extract_text<P: AsRef<Path>>(pdf_path: P) -> Result<Vec<PageText>, Box<dyn Error>> {
        let text = pdf_extract::extract_text(pdf_path)?;
        
        // pdf-extract returns the whole text. For page-aware extraction, we would need a more complex setup.
        // For now, we'll return the whole text as "Page 1" to get things running.
        Ok(vec![PageText {
            page_num: 1,
            text,
        }])
    }
}
