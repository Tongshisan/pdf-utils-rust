use wasm_bindgen::prelude::*;
use std::io::Cursor;

/// 旋转图片
#[wasm_bindgen]
pub fn rotate_image(image_bytes: &[u8], degrees: i32) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    // 旋转图片
    let rotated = match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => return Err(JsValue::from_str("仅支持 90、180、270 度旋转")),
    };

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let mut buffer = Vec::new();
    rotated.write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存旋转后的图片: {}", e)))?;

    Ok(buffer)
}


