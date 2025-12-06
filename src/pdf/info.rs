use wasm_bindgen::prelude::*;
use lopdf::Document;

/// 获取 PDF 页数
#[wasm_bindgen]
pub fn get_pdf_page_count(pdf_bytes: &[u8]) -> Result<usize, JsValue> {
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;
    
    Ok(doc.get_pages().len())
}


