use wasm_bindgen::prelude::*;
use lopdf::{Document, Object, Dictionary};
use std::collections::HashMap;
use super::utils::deep_copy_object_with_map;

/// PDF 合并功能
/// 接收多个 PDF 文件的字节数组，返回合并后的 PDF
#[wasm_bindgen]
pub fn merge_pdfs(pdf_files: Vec<js_sys::Uint8Array>) -> Result<Vec<u8>, JsValue> {
    if pdf_files.is_empty() {
        return Err(JsValue::from_str("至少需要一个 PDF 文件"));
    }

    // 创建新的 PDF 文档
    let mut merged_doc = Document::with_version("1.5");
    let pages_id = merged_doc.new_object_id();
    let mut all_page_ids = Vec::new();

    // 逐个处理每个 PDF 文件
    for (idx, pdf_bytes_js) in pdf_files.iter().enumerate() {
        let pdf_bytes = pdf_bytes_js.to_vec();
        
        // 加载 PDF 文档
        let doc = Document::load_mem(&pdf_bytes)
            .map_err(|e| JsValue::from_str(&format!("无法加载第 {} 个 PDF: {}", idx + 1, e)))?;

        // 为每个文档创建单独的ID映射表
        let mut id_map: HashMap<_,_> = HashMap::new();

        // 获取该文档的所有页面
        let pages: Vec<_> = doc.get_pages().into_iter().collect();
        
        // 复制每一页及其资源
        for (_page_num, page_id) in pages {
            if let Ok(page_obj) = doc.get_object(page_id) {
                // 深度复制页面对象及其所有引用的资源
                let new_page_obj = deep_copy_object_with_map(&doc, &mut merged_doc, page_obj, &mut id_map);
                let new_page_id = merged_doc.add_object(new_page_obj);
                
                // 记录页面ID映射
                id_map.insert(page_id, new_page_id);
                
                // 更新页面的父引用
                if let Ok(Object::Dictionary(ref mut page_dict)) = merged_doc.get_object_mut(new_page_id) {
                    page_dict.set("Parent", Object::Reference(pages_id));
                }
                
                all_page_ids.push(Object::Reference(new_page_id));
            }
        }
    }

    // 创建页面树
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(all_page_ids.clone()));
    pages_dict.set("Count", Object::Integer(all_page_ids.len() as i64));
    merged_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    // 创建目录
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));
    let catalog_id = merged_doc.add_object(Object::Dictionary(catalog));

    merged_doc.trailer.set("Root", Object::Reference(catalog_id));

    // 保存合并后的文档
    let mut buffer = Vec::new();
    merged_doc.save_to(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("无法保存合并的 PDF: {}", e)))?;

    Ok(buffer)
}


