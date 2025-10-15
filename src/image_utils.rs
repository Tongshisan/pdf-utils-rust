use wasm_bindgen::prelude::*;
use image::{ImageFormat, DynamicImage, GenericImageView, imageops::FilterType};
use image::codecs::jpeg::JpegEncoder;
use lopdf::{Document, Object, Dictionary, Stream};
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

/// 图片转 PDF
/// 将多个图片转换为一个 PDF 文件，每个图片占一页
#[wasm_bindgen]
pub fn images_to_pdf(images: Vec<js_sys::Uint8Array>) -> Result<Vec<u8>, JsValue> {
    if images.is_empty() {
        return Err(JsValue::from_str("至少需要一张图片"));
    }

    // 创建新的 PDF 文档
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut page_ids = Vec::new();

    for (idx, image_js) in images.iter().enumerate() {
        let image_bytes = image_js.to_vec();
        
        // 加载图片
        let img = image::load_from_memory(&image_bytes)
            .map_err(|e| JsValue::from_str(&format!("无法加载第 {} 张图片: {}", idx + 1, e)))?;

        let (width, height) = img.dimensions();
        
        // 将图片转换为 JPEG（PDF 中常用）
        let mut jpeg_buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_buffer, 85);
        encoder.encode(
            img.as_bytes(),
            width,
            height,
            img.color().into(),
        ).map_err(|e| JsValue::from_str(&format!("无法转换第 {} 张图片: {}", idx + 1, e)))?;

        // 创建图片对象
        let mut image_dict = Dictionary::new();
        image_dict.set("Type", Object::Name(b"XObject".to_vec()));
        image_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        image_dict.set("Width", Object::Integer(width as i64));
        image_dict.set("Height", Object::Integer(height as i64));
        image_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
        image_dict.set("BitsPerComponent", Object::Integer(8));
        image_dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));

        let image_stream = Stream::new(image_dict, jpeg_buffer);
        let image_id = doc.add_object(Object::Stream(image_stream));

        // 创建资源字典
        let mut resources = Dictionary::new();
        let mut xobject = Dictionary::new();
        xobject.set("Im1", Object::Reference(image_id));
        resources.set("XObject", Object::Dictionary(xobject));

        // 创建内容流：绘制图片以适应页面
        // 使用 A4 页面大小（595 x 842 点）
        let page_width = 595.0;
        let page_height = 842.0;
        
        // 计算缩放比例以适应页面
        let scale_x = page_width / width as f64;
        let scale_y = page_height / height as f64;
        let scale = scale_x.min(scale_y);
        
        let scaled_width = width as f64 * scale;
        let scaled_height = height as f64 * scale;
        
        // 居中图片
        let x = (page_width - scaled_width) / 2.0;
        let y = (page_height - scaled_height) / 2.0;

        let content = format!(
            "q\n{} 0 0 {} {} {} cm\n/Im1 Do\nQ",
            scaled_width, scaled_height, x, y
        );

        let content_stream = Stream::new(Dictionary::new(), content.into_bytes());
        let content_id = doc.add_object(Object::Stream(content_stream));

        // 创建页面对象
        let mut page = Dictionary::new();
        page.set("Type", Object::Name(b"Page".to_vec()));
        page.set("Parent", Object::Reference(pages_id));
        page.set("MediaBox", Object::Array(vec![
            Object::Integer(0),
            Object::Integer(0),
            Object::Integer(page_width as i64),
            Object::Integer(page_height as i64),
        ]));
        page.set("Contents", Object::Reference(content_id));
        page.set("Resources", Object::Dictionary(resources));

        let page_id = doc.add_object(Object::Dictionary(page));
        page_ids.push(Object::Reference(page_id));
    }

    // 创建页面树
    let mut pages = Dictionary::new();
    pages.set("Type", Object::Name(b"Pages".to_vec()));
    pages.set("Kids", Object::Array(page_ids));
    pages.set("Count", Object::Integer(images.len() as i64));
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    // 创建目录
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));
    let catalog_id = doc.add_object(Object::Dictionary(catalog));

    doc.trailer.set("Root", Object::Reference(catalog_id));

    // 保存 PDF
    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| JsValue::from_str(&format!("无法保存 PDF: {}", e)))?;

    Ok(buffer)
}

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

