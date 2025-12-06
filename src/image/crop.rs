use wasm_bindgen::prelude::*;
use std::io::Cursor;

/// 裁剪图片
#[wasm_bindgen]
pub fn crop_image(
    image_bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let mut img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    // 裁剪图片
    let cropped = img.crop(x, y, width, height);

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let mut buffer = Vec::new();
    cropped.write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存裁剪后的图片: {}", e)))?;

    Ok(buffer)
}


