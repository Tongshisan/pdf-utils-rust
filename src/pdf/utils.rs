use lopdf::{Document, Object, Stream, Dictionary, ObjectId};
use std::collections::HashMap;

/// 深度复制对象及其引用（带ID映射，防止循环引用）
pub fn deep_copy_object_with_map(
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
pub fn parse_page_ranges(ranges: &str, max_pages: usize) -> Result<Vec<usize>, String> {
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


