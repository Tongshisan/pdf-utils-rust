use wasm_bindgen::prelude::*;
use image::ImageFormat;
use image::codecs::jpeg::JpegEncoder;
use std::io::Cursor;

/// 图片格式转换
#[wasm_bindgen]
pub fn convert_image_format(
    image_bytes: &[u8],
    target_format: &str,
    quality: Option<u8>,
) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    let mut buffer = Vec::new();
    
    // 根据目标格式编码
    match target_format.to_lowercase().as_str() {
        "jpeg" | "jpg" => {
            let quality_val = quality.unwrap_or(85);
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality_val);
            encoder.encode(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            ).map_err(|e| JsValue::from_str(&format!("无法编码 JPEG: {}", e)))?;
        },
        "png" => {
            img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
                .map_err(|e| JsValue::from_str(&format!("无法编码 PNG: {}", e)))?;
        },
        "bmp" => {
            img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Bmp)
                .map_err(|e| JsValue::from_str(&format!("无法编码 BMP: {}", e)))?;
        },
        "gif" => {
            img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Gif)
                .map_err(|e| JsValue::from_str(&format!("无法编码 GIF: {}", e)))?;
        },
        "webp" => {
            return Err(JsValue::from_str("WebP 编码暂不支持"));
        },
        _ => {
            return Err(JsValue::from_str(&format!("不支持的图片格式: {}", target_format)));
        }
    }

    Ok(buffer)
}

