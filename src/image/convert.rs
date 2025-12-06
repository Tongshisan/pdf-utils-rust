use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::webp::WebPEncoder;
use std::io::Cursor;

/// 支持的图片格式枚举
#[wasm_bindgen]
pub fn get_supported_formats() -> Vec<JsValue> {
    vec![
        JsValue::from_str("jpeg"),
        JsValue::from_str("jpg"),
        JsValue::from_str("png"),
        JsValue::from_str("gif"),
        JsValue::from_str("bmp"),
        JsValue::from_str("webp"),
    ]
}

/// 获取图片的当前格式
#[wasm_bindgen]
pub fn detect_image_format(image_bytes: &[u8]) -> Result<String, JsValue> {
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;
    
    let format_str = match format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpeg",
        ImageFormat::Gif => "gif",
        ImageFormat::WebP => "webp",
        ImageFormat::Bmp => "bmp",
        ImageFormat::Tiff => "tiff",
        ImageFormat::Ico => "ico",
        _ => "unknown",
    };
    
    Ok(format_str.to_string())
}

/// 内部编码函数
fn encode_image(
    img: &DynamicImage,
    target_format: &str,
    quality: Option<u8>,
) -> Result<Vec<u8>, JsValue> {
    let mut buffer = Vec::new();
    
    match target_format.to_lowercase().as_str() {
        "jpeg" | "jpg" => {
            let quality_val = quality.unwrap_or(85);
            // 确保图片是 RGB 格式（JPEG 不支持 alpha 通道）
            let rgb_img = img.to_rgb8();
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality_val);
            encoder.encode(
                rgb_img.as_raw(),
                rgb_img.width(),
                rgb_img.height(),
                image::ExtendedColorType::Rgb8,
            ).map_err(|e| JsValue::from_str(&format!("无法编码 JPEG: {}", e)))?;
        },
        "png" => {
            img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
                .map_err(|e| JsValue::from_str(&format!("无法编码 PNG: {}", e)))?;
        },
        "bmp" => {
            // BMP 不支持 alpha 通道，转换为 RGB
            let rgb_img = DynamicImage::ImageRgb8(img.to_rgb8());
            rgb_img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Bmp)
                .map_err(|e| JsValue::from_str(&format!("无法编码 BMP: {}", e)))?;
        },
        "gif" => {
            img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Gif)
                .map_err(|e| JsValue::from_str(&format!("无法编码 GIF: {}", e)))?;
        },
        "webp" => {
            // 当前 image crate 版本仅支持无损 WebP 编码
            let rgba_img = img.to_rgba8();
            let encoder = WebPEncoder::new_lossless(&mut buffer);
            encoder.encode(
                rgba_img.as_raw(),
                rgba_img.width(),
                rgba_img.height(),
                image::ExtendedColorType::Rgba8,
            ).map_err(|e| JsValue::from_str(&format!("无法编码 WebP: {}", e)))?;
        },
        _ => {
            return Err(JsValue::from_str(&format!("不支持的图片格式: {}", target_format)));
        }
    }

    Ok(buffer)
}

/// 图片格式转换
/// 
/// # 参数
/// - `image_bytes`: 原始图片二进制数据
/// - `target_format`: 目标格式 (jpeg/jpg/png/gif/bmp/webp)
/// - `quality`: 压缩质量 (0-100)，仅对 JPEG 和 WebP 有效。100 表示无损
/// 
/// # 返回
/// 转换后的图片二进制数据
#[wasm_bindgen]
pub fn convert_image_format(
    image_bytes: &[u8],
    target_format: &str,
    quality: Option<u8>,
) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;

    encode_image(&img, target_format, quality)
}

/// 批量图片格式转换
/// 
/// # 参数
/// - `images_data`: 包含多个图片的 JS 数组，每个元素是 Uint8Array
/// - `target_format`: 目标格式 (jpeg/jpg/png/gif/bmp/webp)
/// - `quality`: 压缩质量 (0-100)，仅对 JPEG 和 WebP 有效
/// 
/// # 返回
/// 包含转换后图片的 JS 数组
#[wasm_bindgen]
pub fn batch_convert_image_format(
    images_data: js_sys::Array,
    target_format: &str,
    quality: Option<u8>,
) -> Result<js_sys::Array, JsValue> {
    let result = js_sys::Array::new();
    
    for i in 0..images_data.length() {
        let item = images_data.get(i);
        let uint8_array = js_sys::Uint8Array::new(&item);
        let bytes = uint8_array.to_vec();
        
        match convert_image_format(&bytes, target_format, quality) {
            Ok(converted) => {
                let converted_array = js_sys::Uint8Array::new_with_length(converted.len() as u32);
                converted_array.copy_from(&converted);
                result.push(&converted_array);
            },
            Err(e) => {
                // 如果单个图片转换失败，将错误信息作为字符串添加
                result.push(&JsValue::from_str(&format!("转换失败 (索引 {}): {:?}", i, e)));
            }
        }
    }
    
    Ok(result)
}

/// 图片格式转换，带详细选项
/// 
/// # 参数
/// - `image_bytes`: 原始图片二进制数据
/// - `target_format`: 目标格式
/// - `options_json`: JSON 格式的选项字符串
///   - `quality`: 压缩质量 (0-100)
///   - `preserve_metadata`: 是否保留元数据（暂未实现）
/// 
/// # 返回
/// 转换后的图片二进制数据
#[wasm_bindgen]
pub fn convert_image_with_options(
    image_bytes: &[u8],
    target_format: &str,
    options_json: &str,
) -> Result<Vec<u8>, JsValue> {
    // 解析选项
    let options: serde_json::Value = serde_json::from_str(options_json)
        .map_err(|e| JsValue::from_str(&format!("无效的选项 JSON: {}", e)))?;
    
    let quality = options.get("quality")
        .and_then(|v| v.as_u64())
        .map(|v| v.min(100) as u8);
    
    convert_image_format(image_bytes, target_format, quality)
}

/// 检查是否可以转换到指定格式
#[wasm_bindgen]
pub fn can_convert_to(target_format: &str) -> bool {
    matches!(
        target_format.to_lowercase().as_str(),
        "jpeg" | "jpg" | "png" | "gif" | "bmp" | "webp"
    )
}

/// 获取转换格式的推荐用途说明
#[wasm_bindgen]
pub fn get_format_info(format: &str) -> Result<String, JsValue> {
    let info = match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => serde_json::json!({
            "name": "JPEG",
            "extension": "jpg",
            "mimeType": "image/jpeg",
            "supportsTransparency": false,
            "supportsAnimation": false,
            "lossless": false,
            "description": "适合照片和复杂图像，文件体积小",
            "recommendedUse": ["照片", "复杂色彩图像", "网页图片"]
        }),
        "png" => serde_json::json!({
            "name": "PNG",
            "extension": "png",
            "mimeType": "image/png",
            "supportsTransparency": true,
            "supportsAnimation": false,
            "lossless": true,
            "description": "无损压缩，支持透明度",
            "recommendedUse": ["Logo", "图标", "截图", "需要透明背景的图片"]
        }),
        "gif" => serde_json::json!({
            "name": "GIF",
            "extension": "gif",
            "mimeType": "image/gif",
            "supportsTransparency": true,
            "supportsAnimation": true,
            "lossless": true,
            "description": "支持动画和透明，最多256色",
            "recommendedUse": ["简单动画", "低色彩图像"]
        }),
        "bmp" => serde_json::json!({
            "name": "BMP",
            "extension": "bmp",
            "mimeType": "image/bmp",
            "supportsTransparency": false,
            "supportsAnimation": false,
            "lossless": true,
            "description": "无压缩位图格式，文件体积大",
            "recommendedUse": ["Windows系统图标", "无损存储"]
        }),
        "webp" => serde_json::json!({
            "name": "WebP",
            "extension": "webp",
            "mimeType": "image/webp",
            "supportsTransparency": true,
            "supportsAnimation": true,
            "lossless": "both",
            "description": "现代格式，支持有损和无损压缩，体积小",
            "recommendedUse": ["网页图片", "需要小体积的场景"]
        }),
        _ => return Err(JsValue::from_str(&format!("不支持的格式: {}", format))),
    };
    
    serde_json::to_string(&info)
        .map_err(|e| JsValue::from_str(&format!("JSON 序列化失败: {}", e)))
}
