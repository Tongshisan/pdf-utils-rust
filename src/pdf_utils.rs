use wasm_bindgen::prelude::*;
use lopdf::{Document, Object, Stream, Dictionary, ObjectId};
use std::collections::HashMap;

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
        let mut id_map: HashMap<ObjectId, ObjectId> = HashMap::new();

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
        let mut id_map: HashMap<ObjectId, ObjectId> = HashMap::new();
        
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
    let mut id_map: HashMap<ObjectId, ObjectId> = HashMap::new();
    
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

/// 深度复制对象及其引用（带ID映射，防止循环引用）
fn deep_copy_object_with_map(
    src_doc: &Document,
    dst_doc: &mut Document,
    obj: &Object,
    id_map: &mut HashMap<ObjectId, ObjectId>,
) -> Object {
    match obj {
        Object::Reference(id) => {
            // 检查ID映射表中是否已经有这个对象
            if let Some(&new_id) = id_map.get(id) {
                return Object::Reference(new_id);
            }
            
            // 复制引用的对象
            if let Ok(referenced_obj) = src_doc.get_object(*id) {
                // 先创建占位符，防止循环引用导致无限递归
                let new_id = dst_doc.new_object_id();
                id_map.insert(*id, new_id);
                
                // 然后复制对象内容
                let copied_obj = deep_copy_object_with_map(src_doc, dst_doc, referenced_obj, id_map);
                
                // 插入复制的对象
                dst_doc.objects.insert(new_id, copied_obj);
                
                return Object::Reference(new_id);
            }
            
            // 如果无法获取对象，返回null
            Object::Null
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in dict.iter() {
                let copied_value = deep_copy_object_with_map(src_doc, dst_doc, value, id_map);
                new_dict.set(key.clone(), copied_value);
            }
            Object::Dictionary(new_dict)
        }
        Object::Array(arr) => {
            let new_arr: Vec<Object> = arr.iter()
                .map(|item| deep_copy_object_with_map(src_doc, dst_doc, item, id_map))
                .collect();
            Object::Array(new_arr)
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in stream.dict.iter() {
                let copied_value = deep_copy_object_with_map(src_doc, dst_doc, value, id_map);
                new_dict.set(key.clone(), copied_value);
            }
            Object::Stream(Stream::new(new_dict, stream.content.clone()))
        }
        // 对于其他类型（字符串、数字等），直接克隆
        _ => obj.clone()
    }
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
            let mut id_map: HashMap<ObjectId, ObjectId> = HashMap::new();
            
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

/// 获取 PDF 页数
#[wasm_bindgen]
pub fn get_pdf_page_count(pdf_bytes: &[u8]) -> Result<usize, JsValue> {
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载 PDF: {}", e)))?;
    
    Ok(doc.get_pages().len())
}

