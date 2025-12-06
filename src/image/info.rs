use wasm_bindgen::prelude::*;
use image::GenericImageView;

/// 获取图片信息
#[wasm_bindgen]
pub fn get_image_info(image_bytes: &[u8]) -> Result<String, JsValue> {
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let (width, height) = img.dimensions();
    let color_type = img.color();

    let info = serde_json::json!({
        "width": width,
        "height": height,
        "format": format!("{:?}", format),
        "color_type": format!("{:?}", color_type),
    });

    Ok(info.to_string())
}


