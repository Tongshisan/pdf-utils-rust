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

    /// 创建安全选项（保守优化，不会破坏SVG）
    #[wasm_bindgen]
    pub fn safe() -> Self {
        Self {
            remove_comments: true,
            remove_metadata: false,  // 不移除元数据
            remove_editor_attrs: true,
            minify_whitespace: true,
            precision: 0,  // 不优化数字
            remove_default_attrs: false,  // 不移除默认属性
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

/// 需要移除的元数据元素（不包括 defs，defs 需要特殊处理）
const METADATA_ELEMENTS: &[&str] = &[
    "metadata",
    "title",
    "desc",
];

/// 编辑器相关的命名空间前缀
const EDITOR_PREFIXES: &[&str] = &[
    "inkscape:",
    "sodipodi:",
    "sketch:",
    "illustrator:",
    "adobe:",
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
    ("font-style", "normal"),
    ("font-weight", "normal"),
    ("text-decoration", "none"),
    ("visibility", "visible"),
    ("display", "inline"),
    ("overflow", "visible"),
];

/// 可以安全优化数字的属性名
const NUMERIC_ATTRS: &[&str] = &[
    "d",
    "points",
    "viewBox",
    "x",
    "y",
    "x1",
    "y1",
    "x2",
    "y2",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "width",
    "height",
    "dx",
    "dy",
    "offset",
    "stdDeviation",
    "baseFrequency",
];

/// 不应该转义内容的元素
const RAW_TEXT_ELEMENTS: &[&str] = &[
    "style",
    "script",
];

/// 压缩 SVG（使用默认选项）
#[wasm_bindgen]
pub fn compress_svg(svg_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let options = SvgCompressOptions::new();
    compress_svg_with_options(svg_bytes, &options)
}

/// 压缩 SVG（使用安全选项，推荐用于可能有问题的 SVG）
#[wasm_bindgen]
pub fn compress_svg_safe(svg_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let options = SvgCompressOptions::safe();
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
    build_optimized_svg(&doc.root(), &mut output, options, 0, false);

    Ok(output.into_bytes())
}

/// 递归构建优化后的 SVG
fn build_optimized_svg(
    node: &roxmltree::Node,
    output: &mut String,
    options: &SvgCompressOptions,
    depth: usize,
    in_raw_text_element: bool,
) {
    match node.node_type() {
        roxmltree::NodeType::Root => {
            for child in node.children() {
                build_optimized_svg(&child, output, options, depth, false);
            }
        }
        roxmltree::NodeType::Element => {
            let tag_name = node.tag_name().name();
            
            // 检查是否应该跳过此元素
            if should_skip_element(node, options) {
                return;
            }

            // 检查是否是原始文本元素（style, script）
            let is_raw_text = RAW_TEXT_ELEMENTS.contains(&tag_name);

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
                
                // 属性名（包含前缀）
                let attr_name = attr.name();
                output.push_str(attr_name);
                output.push_str("=\"");
                
                // 属性值（可能需要优化数字精度）
                let value = if options.precision > 0 && should_optimize_numbers(attr_name) {
                    optimize_numbers(attr.value(), options.precision)
                } else {
                    attr.value().to_string()
                };
                output.push_str(&escape_xml_attr(&value));
                output.push('"');
            }

            // 处理命名空间声明
            // 在根元素或 svg 元素上输出所有命名空间
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
                        output.push_str(&escape_xml_attr(ns.uri()));
                        output.push('"');
                    } else {
                        // 默认命名空间
                        output.push_str(" xmlns=\"");
                        output.push_str(&escape_xml_attr(ns.uri()));
                        output.push('"');
                    }
                }
            }

            // 检查是否有子节点
            let has_children = node.children().any(|c| !is_ignorable_node(&c, options));

            if has_children {
                output.push('>');
                
                for child in node.children() {
                    build_optimized_svg(&child, output, options, depth + 1, is_raw_text);
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
            
            // 对于 style/script 元素内的文本，不进行转义
            if in_raw_text_element {
                if options.minify_whitespace {
                    // 对 CSS/JS 只做基本的空白压缩
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        output.push_str(trimmed);
                    }
                } else {
                    output.push_str(text);
                }
            } else {
                if options.minify_whitespace {
                    // 压缩空白，但保留非空白文本
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        output.push_str(&escape_xml_text(trimmed));
                    }
                } else {
                    output.push_str(&escape_xml_text(text));
                }
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
            // 保留处理指令（如 <?xml-stylesheet ...?>）
            // roxmltree 的 PI 节点 target 和 content 需要通过其他方式获取
            // 由于 roxmltree 限制，我们跳过非 xml 声明的 PI
            // 这里简化处理，大多数 SVG 不需要 PI
        }
    }
}

/// 检查是否应该跳过元素
fn should_skip_element(node: &roxmltree::Node, options: &SvgCompressOptions) -> bool {
    let tag_name = node.tag_name().name();
    
    // 检查元数据元素
    if options.remove_metadata && METADATA_ELEMENTS.contains(&tag_name) {
        return true;
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
        // 也检查 data- 属性（通常是编辑器添加的）
        if name.starts_with("data-") {
            return true;
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

/// 检查属性是否应该优化数字
fn should_optimize_numbers(attr_name: &str) -> bool {
    NUMERIC_ATTRS.contains(&attr_name)
}

/// 优化数字精度（只对确定是数字的值进行优化）
fn optimize_numbers(value: &str, precision: u8) -> String {
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    
    while let Some(c) = chars.next() {
        // 只有当遇到数字开头或者负号后跟数字时才尝试解析
        let is_number_start = c.is_ascii_digit() 
            || (c == '-' && chars.peek().map_or(false, |next| next.is_ascii_digit() || *next == '.'))
            || (c == '.' && chars.peek().map_or(false, |next| next.is_ascii_digit()));
        
        if is_number_start {
            // 收集数字
            let mut num_str = String::new();
            num_str.push(c);
            
            let mut has_dot = c == '.';
            let mut has_e = false;
            
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    num_str.push(chars.next().unwrap());
                } else if next == '.' && !has_dot && !has_e {
                    has_dot = true;
                    num_str.push(chars.next().unwrap());
                } else if (next == 'e' || next == 'E') && !has_e {
                    has_e = true;
                    num_str.push(chars.next().unwrap());
                    // e 后面可能有 + 或 -
                    if let Some(&sign) = chars.peek() {
                        if sign == '+' || sign == '-' {
                            num_str.push(chars.next().unwrap());
                        }
                    }
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

/// 转义 XML 属性值中的特殊字符
fn escape_xml_attr(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(c),
        }
    }
    result
}

/// 转义 XML 文本内容中的特殊字符
fn escape_xml_text(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
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
