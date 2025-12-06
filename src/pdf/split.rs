use wasm_bindgen::prelude::*;
use lopdf::{Document, Object, Dictionary};
use std::collections::HashMap;
use super::utils::{deep_copy_object_with_map, parse_page_ranges};

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
        let pages_id = single_page_doc.new_object_id();
        
        // 为每一页创建单独的ID映射表
        let mut id_map: HashMap<_,_> = HashMap::new();
        
        // 深度复制页面对象及其所有引用的资源
        if let Ok(page_obj) = doc.get_object(page_id) {
            let new_page_obj = deep_copy_object_with_map(&doc, &mut single_page_doc, page_obj, &mut id_map);
            let new_page_id = single_page_doc.add_object(new_page_obj);
            
            // 记录页面ID映射
            id_map.insert(page_id, new_page_id);
            
            // 更新页面的父引用
            if let Ok(Object::Dictionary(ref mut page_dict)) = single_page_doc.get_object_mut(new_page_id) {
                page_dict.set("Parent", Object::Reference(pages_id));
            }
            
            // 创建页面树
            let mut pages_dict = Dictionary::new();
            pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
            pages_dict.set("Count", Object::Integer(1));
            pages_dict.set("Kids", Object::Array(vec![Object::Reference(new_page_id)]));
            single_page_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
            
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

    if pages_to_extract.is_empty() {
        return Err(JsValue::from_str("没有可提取的页面"));
    }

    // 创建新文档
    let mut new_doc = Document::with_version("1.5");
    let pages_id = new_doc.new_object_id();
    let mut page_ids = Vec::new();
    
    let all_pages: Vec<_> = doc.get_pages().into_iter().collect();

    // 创建ID映射表
    let mut id_map: HashMap<_,_> = HashMap::new();
    
    // 复制选定的页面及其资源
    for page_idx in &pages_to_extract {
        if *page_idx < all_pages.len() {
            let (_page_num, page_id) = all_pages[*page_idx];
            
            // 递归复制页面对象及其所有引用的资源
            if let Ok(page_obj) = doc.get_object(page_id) {
                // 深度复制页面对象，使用ID映射
                let new_page_obj = deep_copy_object_with_map(&doc, &mut new_doc, page_obj, &mut id_map);
                let new_page_id = new_doc.add_object(new_page_obj);
                
                // 记录页面ID映射
                id_map.insert(page_id, new_page_id);
                
                // 更新页面的父引用
                if let Ok(Object::Dictionary(ref mut page_dict)) = new_doc.get_object_mut(new_page_id) {
                    page_dict.set("Parent", Object::Reference(pages_id));
                }
                
                page_ids.push(Object::Reference(new_page_id));
            }
        }
    }

    // 创建页面树
    let mut pages = Dictionary::new();
    pages.set("Type", Object::Name(b"Pages".to_vec()));
    pages.set("Kids", Object::Array(page_ids.clone()));
    pages.set("Count", Object::Integer(page_ids.len() as i64));
    new_doc.objects.insert(pages_id, Object::Dictionary(pages));

    // 创建目录
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));
    let catalog_id = new_doc.add_object(Object::Dictionary(catalog));

    new_doc.trailer.set("Root", Object::Reference(catalog_id));

    // 保存文档
    let mut buffer = Vec::new();
    new_doc.save_to(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("无法保存 PDF: {}", e)))?;

    Ok(buffer)
}

/// 按页码范围分割成多个 PDF 文件
/// page_ranges: 格式如 "1,3,5" 或 "1-2,4-5" 
/// 返回多个独立的 PDF 文件数组
#[wasm_bindgen]
pub fn split_pdf_by_pages(pdf_bytes: &[u8], page_ranges: &str) -> Result<js_sys::Array, JsValue> {
    // 加载 PDF 文档
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;

    // 解析页码范围
    let pages_to_extract = parse_page_ranges(page_ranges, doc.get_pages().len())
        .map_err(|e| JsValue::from_str(&e))?;

    if pages_to_extract.is_empty() {
        return Err(JsValue::from_str("没有可提取的页面"));
    }

    let result = js_sys::Array::new_with_length(pages_to_extract.len() as u32);
    let all_pages: Vec<_> = doc.get_pages().into_iter().collect();

    // 为每个页面创建独立的 PDF
    for (output_idx, &page_idx) in pages_to_extract.iter().enumerate() {
        if page_idx < all_pages.len() {
            let (_page_num, page_id) = all_pages[page_idx];
            
            // 创建单页文档
            let mut single_page_doc = Document::with_version("1.5");
            let pages_id = single_page_doc.new_object_id();
            let mut id_map: HashMap<_,_> = HashMap::new();
            
            // 深度复制页面对象
            if let Ok(page_obj) = doc.get_object(page_id) {
                let new_page_obj = deep_copy_object_with_map(&doc, &mut single_page_doc, page_obj, &mut id_map);
                let new_page_id = single_page_doc.add_object(new_page_obj);
                
                id_map.insert(page_id, new_page_id);
                
                // 更新页面的父引用
                if let Ok(Object::Dictionary(ref mut page_dict)) = single_page_doc.get_object_mut(new_page_id) {
                    page_dict.set("Parent", Object::Reference(pages_id));
                }
                
                // 创建页面树
                let mut pages_dict = Dictionary::new();
                pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
                pages_dict.set("Count", Object::Integer(1));
                pages_dict.set("Kids", Object::Array(vec![Object::Reference(new_page_id)]));
                single_page_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
                
                // 创建目录
                let mut catalog = Dictionary::new();
                catalog.set("Type", Object::Name(b"Catalog".to_vec()));
                catalog.set("Pages", Object::Reference(pages_id));
                
                let catalog_id = single_page_doc.add_object(Object::Dictionary(catalog));
                single_page_doc.trailer.set("Root", Object::Reference(catalog_id));
                
                // 保存单页文档
                let mut buffer = Vec::new();
                single_page_doc.save_to(&mut buffer)
                    .map_err(|e| JsValue::from_str(&format!("无法保存第 {} 页: {}", page_idx + 1, e)))?;
                
                let uint8_array = js_sys::Uint8Array::from(&buffer[..]);
                result.set(output_idx as u32, JsValue::from(uint8_array));
            }
        }
    }

    Ok(result)
}


