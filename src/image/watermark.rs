use wasm_bindgen::prelude::*;
use image::{Rgba, DynamicImage, ImageFormat};
use std::io::Cursor;

/// 去除图片水印 - 基于区域修复
/// 通过指定水印区域，使用周围像素的平均值填充该区域
#[wasm_bindgen]
pub fn remove_watermark_by_region(
    image_bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    method: &str,
) -> Result<Vec<u8>, JsValue> {
    use image::GenericImage;
    
    // 加载图片
    let mut img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?
        .to_rgba8();

    let (img_width, img_height) = img.dimensions();

    // 验证区域是否在图片范围内
    if x + width > img_width || y + height > img_height {
        return Err(JsValue::from_str("指定区域超出图片范围"));
    }

    match method {
        "average" => {
            // 方法1: 使用周围像素的平均值填充
            let border_size = 5u32;
            let mut sum_r = 0u64;
            let mut sum_g = 0u64;
            let mut sum_b = 0u64;
            let mut count = 0u64;

            // 收集水印区域周围边界的像素
            for dy in 0..height {
                for dx in 0..width {
                    let px = x + dx;
                    let py = y + dy;
                    
                    // 检查是否在边界区域
                    if dx < border_size || dx >= width - border_size || 
                       dy < border_size || dy >= height - border_size {
                        let pixel = img.get_pixel(px, py);
                        sum_r += pixel[0] as u64;
                        sum_g += pixel[1] as u64;
                        sum_b += pixel[2] as u64;
                        count += 1;
                    }
                }
            }

            // 计算平均颜色
            let avg_color = if count > 0 {
                Rgba([
                    (sum_r / count) as u8,
                    (sum_g / count) as u8,
                    (sum_b / count) as u8,
                    255u8,
                ])
            } else {
                Rgba([255u8, 255u8, 255u8, 255u8]) // 默认白色
            };

            // 填充水印区域
            for dy in 0..height {
                for dx in 0..width {
                    img.put_pixel(x + dx, y + dy, avg_color);
                }
            }
        },
        "blur" => {
            // 方法2: 使用模糊效果
            use image::imageops::blur;
            let sub_img = img.sub_image(x, y, width, height).to_image();
            let blurred = blur(&sub_img, 10.0);
            
            for dy in 0..height {
                for dx in 0..width {
                    let pixel = blurred.get_pixel(dx, dy);
                    img.put_pixel(x + dx, y + dy, *pixel);
                }
            }
        },
        "median" => {
            // 方法3: 中值滤波
            for dy in 0..height {
                for dx in 0..width {
                    let px = x + dx;
                    let py = y + dy;
                    
                    let mut r_values = Vec::new();
                    let mut g_values = Vec::new();
                    let mut b_values = Vec::new();
                    
                    // 收集3x3区域的像素
                    for ny in py.saturating_sub(1)..=(py + 1).min(img_height - 1) {
                        for nx in px.saturating_sub(1)..=(px + 1).min(img_width - 1) {
                            let pixel = img.get_pixel(nx, ny);
                            r_values.push(pixel[0]);
                            g_values.push(pixel[1]);
                            b_values.push(pixel[2]);
                        }
                    }
                    
                    r_values.sort();
                    g_values.sort();
                    b_values.sort();
                    
                    let mid = r_values.len() / 2;
                    let median_color = Rgba([r_values[mid], g_values[mid], b_values[mid], 255u8]);
                    img.put_pixel(px, py, median_color);
                }
            }
        },
        _ => {
            return Err(JsValue::from_str("不支持的方法，请使用 'average', 'blur' 或 'median'"));
        }
    }

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let mut buffer = Vec::new();
    DynamicImage::ImageRgba8(img).write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存处理后的图片: {}", e)))?;

    Ok(buffer)
}

/// 去除图片水印 - 基于颜色过滤
/// 将指定颜色范围的像素替换为透明或背景色
#[wasm_bindgen]
pub fn remove_watermark_by_color(
    image_bytes: &[u8],
    target_r: u8,
    target_g: u8,
    target_b: u8,
    tolerance: u8,
    replace_with_white: bool,
) -> Result<Vec<u8>, JsValue> {
    use image::Rgba;
    
    // 加载图片
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?;
    
    let mut rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    // 遍历所有像素
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // 计算颜色差异
            let r_diff = (r as i32 - target_r as i32).abs() as u8;
            let g_diff = (g as i32 - target_g as i32).abs() as u8;
            let b_diff = (b as i32 - target_b as i32).abs() as u8;

            // 如果颜色在容差范围内，则替换
            if r_diff <= tolerance && g_diff <= tolerance && b_diff <= tolerance {
                if replace_with_white {
                    // 替换为白色
                    rgba_img.put_pixel(x, y, Rgba([255u8, 255u8, 255u8, 255u8]));
                } else {
                    // 使用周围像素的平均值替换
                    let mut sum_r = 0u32;
                    let mut sum_g = 0u32;
                    let mut sum_b = 0u32;
                    let mut count = 0u32;

                    for ny in y.saturating_sub(2)..=(y + 2).min(height - 1) {
                        for nx in x.saturating_sub(2)..=(x + 2).min(width - 1) {
                            if nx == x && ny == y {
                                continue;
                            }
                            let neighbor = rgba_img.get_pixel(nx, ny);
                            let nr_diff = (neighbor[0] as i32 - target_r as i32).abs() as u8;
                            let ng_diff = (neighbor[1] as i32 - target_g as i32).abs() as u8;
                            let nb_diff = (neighbor[2] as i32 - target_b as i32).abs() as u8;
                            
                            // 只使用不是水印颜色的像素
                            if nr_diff > tolerance || ng_diff > tolerance || nb_diff > tolerance {
                                sum_r += neighbor[0] as u32;
                                sum_g += neighbor[1] as u32;
                                sum_b += neighbor[2] as u32;
                                count += 1;
                            }
                        }
                    }

                    if count > 0 {
                        rgba_img.put_pixel(x, y, Rgba([
                            (sum_r / count) as u8,
                            (sum_g / count) as u8,
                            (sum_b / count) as u8,
                            255u8,
                        ]));
                    } else {
                        rgba_img.put_pixel(x, y, Rgba([255u8, 255u8, 255u8, 255u8]));
                    }
                }
            }
        }
    }

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .unwrap_or(ImageFormat::Png);

    let mut buffer = Vec::new();
    DynamicImage::ImageRgba8(rgba_img).write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存处理后的图片: {}", e)))?;

    Ok(buffer)
}

/// 减淡水印 - 使整个图片的特定区域变淡
#[wasm_bindgen]
pub fn fade_watermark(
    image_bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    brightness_increase: i32,
) -> Result<Vec<u8>, JsValue> {
    use image::Rgba;
    
    // 加载图片
    let mut img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法加载图片: {}", e)))?
        .to_rgba8();

    let (img_width, img_height) = img.dimensions();

    // 验证区域是否在图片范围内
    if x + width > img_width || y + height > img_height {
        return Err(JsValue::from_str("指定区域超出图片范围"));
    }

    // 增加指定区域的亮度
    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;
            let pixel = img.get_pixel(px, py);
            
            let new_r = ((pixel[0] as i32 + brightness_increase).max(0).min(255)) as u8;
            let new_g = ((pixel[1] as i32 + brightness_increase).max(0).min(255)) as u8;
            let new_b = ((pixel[2] as i32 + brightness_increase).max(0).min(255)) as u8;
            
            img.put_pixel(px, py, Rgba([new_r, new_g, new_b, pixel[3]]));
        }
    }

    // 保存为原格式
    let format = image::guess_format(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法识别图片格式: {}", e)))?;

    let mut buffer = Vec::new();
    DynamicImage::ImageRgba8(img).write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| JsValue::from_str(&format!("无法保存处理后的图片: {}", e)))?;

    Ok(buffer)
}

