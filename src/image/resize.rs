use wasm_bindgen::prelude::*;
use image::imageops::FilterType;
use std::io::Cursor;

/// 调整图片大小
#[wasm_bindgen]
pub fn resize_image(
    image_bytes: &[u8],
    width: u32,
    height: u32,
    maintain_aspect_ratio: bool,
) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    // 调整大小
    let resized = if maintain_aspect_ratio {
        img.resize(width, height, FilterType::Lanczos3)
    } else {
        img.resize_exact(width, height, FilterType::Lanczos3)
    };

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let mut buffer = Vec::new();
    
    resized.write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存调整后的图片: {}", e)))?;

    Ok(buffer)
}


