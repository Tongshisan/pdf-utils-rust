use wasm_bindgen::prelude::*;

/// SVG 压缩选项
#[wasm_bindgen]
pub struct SvgCompressOptions {
    /// 是否移除注释
    remove_comments: bool,
    /// 是否移除元数据元素（metadata, title, desc 等）
    remove_metadata: bool,
    /// 是否移除编辑器属性（如 inkscape:*, sodipodi:* 等）
    remove_editor_attrs: bool,
    /// 是否移除空白/格式化
    minify_whitespace: bool,
    /// 数字精度（小数位数，0 表示不处理）
    precision: u8,
    /// 是否移除默认值属性
    remove_default_attrs: bool,
}

#[wasm_bindgen]
impl SvgCompressOptions {
    /// 创建默认选项（启用所有优化）
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            remove_comments: true,
            remove_metadata: true,
            remove_editor_attrs: true,
            minify_whitespace: true,
            precision: 3,
            remove_default_attrs: true,
        }
    }

    /// 创建最小优化选项（只移除空白）
    #[wasm_bindgen]
    pub fn minimal() -> Self {
        Self {
            remove_comments: false,
            remove_metadata: false,
            remove_editor_attrs: false,
            minify_whitespace: true,
            precision: 0,
            remove_default_attrs: false,
        }
    }

    /// 设置是否移除注释
    #[wasm_bindgen]
    pub fn set_remove_comments(&mut self, value: bool) {
        self.remove_comments = value;
    }

    /// 设置是否移除元数据
    #[wasm_bindgen]
    pub fn set_remove_metadata(&mut self, value: bool) {
        self.remove_metadata = value;
    }

    /// 设置是否移除编辑器属性
    #[wasm_bindgen]
    pub fn set_remove_editor_attrs(&mut self, value: bool) {
        self.remove_editor_attrs = value;
    }

    /// 设置是否压缩空白
    #[wasm_bindgen]
    pub fn set_minify_whitespace(&mut self, value: bool) {
        self.minify_whitespace = value;
    }

    /// 设置数字精度
    #[wasm_bindgen]
    pub fn set_precision(&mut self, value: u8) {
        self.precision = value;
    }

    /// 设置是否移除默认值属性
    #[wasm_bindgen]
    pub fn set_remove_default_attrs(&mut self, value: bool) {
        self.remove_default_attrs = value;
    }
}

impl Default for SvgCompressOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// 需要移除的元数据元素
const METADATA_ELEMENTS: &[&str] = &[
    "metadata",
    "title",
    "desc",
    "defs", // 如果为空则移除
];

/// 编辑器相关的命名空间前缀
const EDITOR_PREFIXES: &[&str] = &[
    "inkscape:",
    "sodipodi:",
    "sketch:",
    "illustrator:",
    "adobe:",
    "data-",
];

/// 默认值属性（可以安全移除）
const DEFAULT_ATTRS: &[(&str, &str)] = &[
    ("fill-opacity", "1"),
    ("stroke-opacity", "1"),
    ("opacity", "1"),
    ("fill-rule", "nonzero"),
    ("clip-rule", "nonzero"),
    ("stroke-linecap", "butt"),
    ("stroke-linejoin", "miter"),
    ("stroke-miterlimit", "4"),
    ("stroke-dashoffset", "0"),
    ("stroke-width", "1"),
    ("font-style", "normal"),
    ("font-weight", "normal"),
    ("text-decoration", "none"),
    ("visibility", "visible"),
    ("display", "inline"),
    ("overflow", "visible"),
];

/// 压缩 SVG（使用默认选项）
#[wasm_bindgen]
pub fn compress_svg(svg_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let options = SvgCompressOptions::new();
    compress_svg_with_options(svg_bytes, &options)
}

/// 压缩 SVG（使用自定义选项）
#[wasm_bindgen]
pub fn compress_svg_with_options(
    svg_bytes: &[u8],
    options: &SvgCompressOptions,
) -> Result<Vec<u8>, JsValue> {
    // 将字节转换为字符串
    let svg_str = std::str::from_utf8(svg_bytes)
        .map_err(|e| JsValue::from_str(&format!("无法解析 SVG 字符串: {}", e)))?;

    // 解析 SVG
    let doc = roxmltree::Document::parse(svg_str)
        .map_err(|e| JsValue::from_str(&format!("无法解析 SVG 文档: {}", e)))?;

    // 重新构建优化后的 SVG
    let mut output = String::with_capacity(svg_str.len());
    
    // 添加 XML 声明（如果原文件有的话）
    if svg_str.trim().starts_with("<?xml") {
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    }

    // 遍历并重建 SVG
    build_optimized_svg(&doc.root(), &mut output, options, 0);

    Ok(output.into_bytes())
}

/// 递归构建优化后的 SVG
fn build_optimized_svg(
    node: &roxmltree::Node,
    output: &mut String,
    options: &SvgCompressOptions,
    depth: usize,
) {
    match node.node_type() {
        roxmltree::NodeType::Root => {
            for child in node.children() {
                build_optimized_svg(&child, output, options, depth);
            }
        }
        roxmltree::NodeType::Element => {
            let tag_name = node.tag_name().name();
            
            // 检查是否应该跳过此元素
            if should_skip_element(node, options) {
                return;
            }

            // 开始标签
            output.push('<');
            
            // 处理命名空间前缀
            if let Some(prefix) = node.tag_name().namespace() {
                if let Some(p) = get_namespace_prefix(node, prefix) {
                    output.push_str(p);
                    output.push(':');
                }
            }
            output.push_str(tag_name);

            // 处理属性
            for attr in node.attributes() {
                if should_skip_attribute(&attr, options) {
                    continue;
                }

                output.push(' ');
                
                // 属性名
                if let Some(prefix) = attr.namespace() {
                    if let Some(p) = get_namespace_prefix(node, prefix) {
                        output.push_str(p);
                        output.push(':');
                    }
                }
                output.push_str(attr.name());
                output.push_str("=\"");
                
                // 属性值（可能需要优化数字精度）
                let value = if options.precision > 0 {
                    optimize_numbers(attr.value(), options.precision)
                } else {
                    attr.value().to_string()
                };
                output.push_str(&escape_xml(&value));
                output.push('"');
            }

            // 处理命名空间声明（只在根元素上）
            if depth == 0 || tag_name == "svg" {
                for ns in node.namespaces() {
                    if let Some(prefix) = ns.name() {
                        // 检查是否应该跳过编辑器命名空间
                        if options.remove_editor_attrs {
                            let is_editor_ns = EDITOR_PREFIXES.iter()
                                .any(|p| p.trim_end_matches(':') == prefix);
                            if is_editor_ns {
                                continue;
                            }
                        }
                        output.push_str(" xmlns:");
                        output.push_str(prefix);
                        output.push_str("=\"");
                        output.push_str(ns.uri());
                        output.push('"');
                    } else {
                        // 默认命名空间
                        output.push_str(" xmlns=\"");
                        output.push_str(ns.uri());
                        output.push('"');
                    }
                }
            }

            // 检查是否有子节点
            let has_children = node.children().any(|c| !is_ignorable_node(&c, options));

            if has_children {
                output.push('>');
                
                for child in node.children() {
                    build_optimized_svg(&child, output, options, depth + 1);
                }

                output.push_str("</");
                if let Some(prefix) = node.tag_name().namespace() {
                    if let Some(p) = get_namespace_prefix(node, prefix) {
                        output.push_str(p);
                        output.push(':');
                    }
                }
                output.push_str(tag_name);
                output.push('>');
            } else {
                output.push_str("/>");
            }
        }
        roxmltree::NodeType::Text => {
            let text = node.text().unwrap_or("");
            if options.minify_whitespace {
                // 压缩空白，但保留非空白文本
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    output.push_str(&escape_xml(trimmed));
                }
            } else {
                output.push_str(&escape_xml(text));
            }
        }
        roxmltree::NodeType::Comment => {
            if !options.remove_comments {
                output.push_str("<!--");
                output.push_str(node.text().unwrap_or(""));
                output.push_str("-->");
            }
        }
        roxmltree::NodeType::PI => {
            // 处理指令（如 <?xml-stylesheet ...?>）
            // 保留这些，因为它们可能是必要的
        }
    }
}

/// 检查是否应该跳过元素
fn should_skip_element(node: &roxmltree::Node, options: &SvgCompressOptions) -> bool {
    let tag_name = node.tag_name().name();
    
    // 检查元数据元素
    if options.remove_metadata {
        if METADATA_ELEMENTS.contains(&tag_name) {
            // 对于 defs，只有当它为空时才跳过
            if tag_name == "defs" {
                return !node.has_children();
            }
            return true;
        }
    }

    // 检查编辑器特定元素
    if options.remove_editor_attrs {
        if let Some(prefix) = node.tag_name().namespace() {
            // 检查是否是编辑器命名空间
            let prefixes: Vec<&str> = node.namespaces()
                .filter_map(|ns| {
                    if ns.uri() == prefix {
                        ns.name()
                    } else {
                        None
                    }
                })
                .collect();
            
            for p in prefixes {
                let full_prefix = format!("{}:", p);
                if EDITOR_PREFIXES.contains(&full_prefix.as_str()) {
                    return true;
                }
            }
        }
    }

    false
}

/// 检查是否应该跳过属性
fn should_skip_attribute(attr: &roxmltree::Attribute, options: &SvgCompressOptions) -> bool {
    let name = attr.name();
    let value = attr.value();

    // 检查编辑器属性
    if options.remove_editor_attrs {
        for prefix in EDITOR_PREFIXES {
            if name.starts_with(prefix) {
                return true;
            }
        }
    }

    // 检查默认值属性
    if options.remove_default_attrs {
        for (attr_name, default_value) in DEFAULT_ATTRS {
            if name == *attr_name && value == *default_value {
                return true;
            }
        }
    }

    false
}

/// 检查是否是可忽略的节点
fn is_ignorable_node(node: &roxmltree::Node, options: &SvgCompressOptions) -> bool {
    match node.node_type() {
        roxmltree::NodeType::Comment => options.remove_comments,
        roxmltree::NodeType::Text => {
            if options.minify_whitespace {
                node.text().map_or(true, |t| t.trim().is_empty())
            } else {
                false
            }
        }
        roxmltree::NodeType::Element => should_skip_element(node, options),
        _ => false,
    }
}

/// 获取命名空间前缀
fn get_namespace_prefix<'a>(node: &'a roxmltree::Node, uri: &str) -> Option<&'a str> {
    node.namespaces()
        .find(|ns| ns.uri() == uri)
        .and_then(|ns| ns.name())
}

/// 优化数字精度
fn optimize_numbers(value: &str, precision: u8) -> String {
    // 使用正则表达式替换数字（简化实现）
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() || c == '-' || c == '.' {
            // 收集数字
            let mut num_str = String::new();
            num_str.push(c);
            
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() || next == '.' || next == 'e' || next == 'E' || next == '-' || next == '+' {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            
            // 尝试解析和格式化数字
            if let Ok(num) = num_str.parse::<f64>() {
                let formatted = format!("{:.prec$}", num, prec = precision as usize);
                // 移除尾部的零
                let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
                result.push_str(if trimmed.is_empty() { "0" } else { trimmed });
            } else {
                result.push_str(&num_str);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

/// 转义 XML 特殊字符
fn escape_xml(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            _ => result.push(c),
        }
    }
    result
}

/// 获取 SVG 压缩统计信息
#[wasm_bindgen]
pub fn get_svg_compress_stats(original: &[u8], compressed: &[u8]) -> Result<String, JsValue> {
    let original_size = original.len();
    let compressed_size = compressed.len();
    let saved = if original_size > compressed_size {
        original_size - compressed_size
    } else {
        0
    };
    let ratio = if original_size > 0 {
        (saved as f64 / original_size as f64) * 100.0
    } else {
        0.0
    };

    Ok(format!(
        r#"{{"original_size":{},"compressed_size":{},"saved_bytes":{},"compression_ratio":{:.2}}}"#,
        original_size, compressed_size, saved, ratio
    ))
}

