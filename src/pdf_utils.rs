use wasm_bindgen::prelude::*;
use lopdf::{Document, Object, Stream, Dictionary};
use std::collections::BTreeMap;

/// PDF 合并功能
/// 接收多个 PDF 文件的字节数组，返回合并后的 PDF
#[wasm_bindgen]
pub fn merge_pdfs(pdf_files: Vec<js_sys::Uint8Array>) -> Result<Vec<u8>, JsValue> {
    // 创建新的 PDF 文档
    let mut merged_doc = Document::with_version("1.5");
    let mut max_id = 1;

    for pdf_bytes_js in pdf_files {
        let pdf_bytes = pdf_bytes_js.to_vec();
        
        // 加载 PDF 文档
        let mut doc = Document::load_mem(&pdf_bytes)
            .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;

        // 重新编号对象 ID 以避免冲突
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        // 合并文档
        merged_doc.objects.append(&mut doc.objects);

        // 获取页面
        let pages = doc.get_pages();
        for (page_num, page_id) in pages {
            if let Ok(page_obj) = doc.get_object(page_id) {
                let new_page_id = merged_doc.add_object(page_obj.clone());
                
                // 添加到合并文档的页面树
                if merged_doc.catalog().is_err() {
                    merged_doc.catalog_mut()
                        .map_err(|e| JsValue::from_str(&format!("无法创建目录: {}", e)))?;
                }
            }
        }
    }

    // 保存合并后的文档
    let mut buffer = Vec::new();
    merged_doc.save_to(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("无法保存合并的 PDF: {}", e)))?;

    Ok(buffer)
}

/// PDF 分割功能
/// 将一个 PDF 文件分割成多个单页 PDF
#[wasm_bindgen]
pub fn split_pdf(pdf_bytes: &[u8]) -> Result<js_sys::Array, JsValue> {
    // 加载 PDF 文档
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;

    let page_count = doc.get_pages().len();
    let result = js_sys::Array::new_with_length(page_count as u32);

    // 获取所有页面
    let pages: Vec<_> = doc.get_pages().into_iter().collect();

    for (idx, (_page_num, page_id)) in pages.into_iter().enumerate() {
        // 为每一页创建新文档
        let mut single_page_doc = Document::with_version("1.5");
        
        // 复制必要的资源和页面对象
        if let Ok(page_obj) = doc.get_object(page_id) {
            let new_page_id = single_page_doc.add_object(page_obj.clone());
            
            // 创建页面树
            let mut pages_dict = Dictionary::new();
            pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
            pages_dict.set("Count", Object::Integer(1));
            pages_dict.set("Kids", Object::Array(vec![Object::Reference(new_page_id)]));
            
            let pages_id = single_page_doc.add_object(Object::Dictionary(pages_dict));
            
            // 创建目录
            let mut catalog = Dictionary::new();
            catalog.set("Type", Object::Name(b"Catalog".to_vec()));
            catalog.set("Pages", Object::Reference(pages_id));
            
            let catalog_id = single_page_doc.add_object(Object::Dictionary(catalog));
            single_page_doc.trailer.set("Root", Object::Reference(catalog_id));
            
            // 保存单页文档
            let mut buffer = Vec::new();
            single_page_doc.save_to(&mut buffer)
                .map_err(|e| JsValue::from_str(&format!("无法保存第 {} 页: {}", idx + 1, e)))?;
            
            let uint8_array = js_sys::Uint8Array::from(&buffer[..]);
            result.set(idx as u32, JsValue::from(uint8_array));
        }
    }

    Ok(result)
}

/// 按页码范围分割 PDF
/// page_ranges: 格式如 "1-3,5,7-9" 表示提取第1-3页、第5页和第7-9页
#[wasm_bindgen]
pub fn split_pdf_by_range(pdf_bytes: &[u8], page_ranges: &str) -> Result<Vec<u8>, JsValue> {
    // 加载 PDF 文档
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;

    // 解析页码范围
    let pages_to_extract = parse_page_ranges(page_ranges, doc.get_pages().len())
        .map_err(|e| JsValue::from_str(&e))?;

    // 创建新文档
    let mut new_doc = Document::with_version("1.5");
    let all_pages: Vec<_> = doc.get_pages().into_iter().collect();

    for page_idx in pages_to_extract {
        if page_idx < all_pages.len() {
            let (_page_num, page_id) = all_pages[page_idx];
            if let Ok(page_obj) = doc.get_object(page_id) {
                new_doc.add_object(page_obj.clone());
            }
        }
    }

    // 保存文档
    let mut buffer = Vec::new();
    new_doc.save_to(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("无法保存 PDF: {}", e)))?;

    Ok(buffer)
}

/// 解析页码范围字符串
fn parse_page_ranges(ranges: &str, max_pages: usize) -> Result<Vec<usize>, String> {
    let mut pages = Vec::new();
    
    for range in ranges.split(',') {
        let range = range.trim();
        if range.contains('-') {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("无效的页码范围: {}", range));
            }
            
            let start: usize = parts[0].trim().parse()
                .map_err(|_| format!("无效的起始页码: {}", parts[0]))?;
            let end: usize = parts[1].trim().parse()
                .map_err(|_| format!("无效的结束页码: {}", parts[1]))?;
            
            if start < 1 || end > max_pages || start > end {
                return Err(format!("页码范围超出有效范围: {}", range));
            }
            
            for page in start..=end {
                pages.push(page - 1); // 转换为 0 索引
            }
        } else {
            let page: usize = range.parse()
                .map_err(|_| format!("无效的页码: {}", range))?;
            
            if page < 1 || page > max_pages {
                return Err(format!("页码超出范围: {}", page));
            }
            
            pages.push(page - 1); // 转换为 0 索引
        }
    }
    
    pages.sort_unstable();
    pages.dedup();
    Ok(pages)
}

/// 获取 PDF 页数
#[wasm_bindgen]
pub fn get_pdf_page_count(pdf_bytes: &[u8]) -> Result<usize, JsValue> {
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;
    
    Ok(doc.get_pages().len())
}

