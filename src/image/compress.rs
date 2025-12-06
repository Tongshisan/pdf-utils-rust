use wasm_bindgen::prelude::*;
use image::codecs::jpeg::JpegEncoder;

/// 压缩图片（通过调整质量）
#[wasm_bindgen]
pub fn compress_image(
    image_bytes: &[u8],
    quality: u8,
) -> Result<Vec<u8>, JsValue> {
    if quality > 100 {
        return Err(JsValue::from_str("质量参数必须在 0-100 之间"));
    }

    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    // 保存为 JPEG 格式以应用压缩
    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
    
    encoder.encode(
        img.as_bytes(),
        img.width(),
        img.height(),
        img.color().into(),
    ).map_err(|e| JsValue::from_str(&format!("无法压缩图片: {}", e)))?;

    Ok(buffer)
}


