# CLAUDE.md

这是一个指导 AI 助手理解和操作本项目的文档。

## 项目概述

**pdf-utils-rust** 是一个使用 Rust 编写并编译为 WebAssembly 的 PDF 和图片处理工具库。

- NPM 包名: `@tongshisan/pdf-utils-rust`
- 目标平台: 浏览器 (Web)、Node.js、打包工具 (Bundler)

## 技术栈

- **语言**: Rust (Edition 2021)
- **编译目标**: WebAssembly (wasm32-unknown-unknown)
- **核心工具**:
  - `wasm-pack` - 构建和打包 WASM
  - `wasm-bindgen` - Rust 与 JavaScript 绑定
- **主要依赖**:
  - `lopdf` - PDF 文档处理
  - `image` - 图片处理 (支持 JPEG, PNG, GIF, BMP, WebP)
  - `roxmltree` - SVG/XML 解析
  - `serde` / `serde_json` - 序列化
  - `web-sys` / `js-sys` - Web API 绑定

## 项目结构

```
src/
├── lib.rs              # 入口文件，初始化 panic hook
├── pdf/                # PDF 处理模块
│   ├── mod.rs          # 模块导出
│   └── ...             # 各工具文件
└── image/              # 图片处理模块
    ├── mod.rs          # 模块导出
    └── ...             # 各工具文件

pkg/                    # 构建输出目录 (发布到 npm)
examples/               # 使用示例 (HTML)
```

## 工具列表

**每个 `.rs` 文件对应一个独立的工具**，导出一个或多个 `#[wasm_bindgen]` 函数供 JavaScript 调用。

### PDF 工具 (`src/pdf/`)

| 文件       | 工具名称 | 功能描述                    |
| ---------- | -------- | --------------------------- |
| `info.rs`  | PDF 信息 | 获取 PDF 页数、元数据等信息 |
| `merge.rs` | PDF 合并 | 将多个 PDF 文件合并为一个   |
| `split.rs` | PDF 拆分 | 将 PDF 按页拆分为多个文件   |
| `utils.rs` | 工具函数 | 内部辅助函数 (不导出)       |

### 图片工具 (`src/image/`)

| 文件              | 工具名称   | 功能描述                             |
| ----------------- | ---------- | ------------------------------------ |
| `compress.rs`     | 图片压缩   | 压缩图片体积，支持质量调节           |
| `convert.rs`      | 格式转换   | 图片格式互转 (PNG/JPEG/GIF/BMP/WebP) |
| `crop.rs`         | 图片裁剪   | 按指定区域裁剪图片                   |
| `info.rs`         | 图片信息   | 获取图片尺寸、格式等信息             |
| `resize.rs`       | 图片缩放   | 调整图片尺寸                         |
| `rotate.rs`       | 图片旋转   | 旋转图片 (90°/180°/270°)             |
| `watermark.rs`    | 添加水印   | 为图片添加文字或图片水印             |
| `svg_compress.rs` | SVG 压缩   | 压缩 SVG 文件体积                    |
| `to_pdf.rs`       | 图片转 PDF | 将图片转换为 PDF 文档                |

### 添加新工具

1. 在对应模块目录下创建新的 `.rs` 文件 (如 `src/image/flip.rs`)
2. 在文件中实现功能并使用 `#[wasm_bindgen]` 导出函数
3. 在 `mod.rs` 中添加 `mod flip;` 和 `pub use flip::*;`
4. 添加使用说明文档
5. 重新构建: `wasm-pack build --target web`

## 架构

- **模块化设计**: PDF 和 Image 两个独立模块，通过 `lib.rs` 统一导出
- **WASM 导出**: 使用 `#[wasm_bindgen]` 宏导出函数供 JavaScript 调用
- **数据传输**: 二进制数据通过 `&[u8]` / `Vec<u8>` 在 Rust 和 JS 之间传递
- **错误处理**: 使用 `Result<T, JsValue>` 返回错误信息到 JavaScript

## 常用命令

### 开发构建

```bash
wasm-pack build --target web --dev
```

### 生产构建 (多目标)

```bash
./build.sh
```

生成:

- `pkg/web/` - 浏览器直接使用
- `pkg/nodejs/` - Node.js 环境
- `pkg/bundler/` - Webpack/Rollup 等打包工具

### 发布到 NPM

```bash
./publish.sh
```

注意:

- 发布前需要先登录 npm (`npm login`)
- 版本号在 `Cargo.toml` 中修改
- 包名会自动修正为 `@tongshisan/pdf-utils-rust`

### 本地测试

```bash
python serve.py        # 启动本地服务器
# 然后打开 examples/index.html
```

### 运行测试

```bash
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
```

## 编码规范

1. **函数导出**: 所有需要暴露给 JS 的函数必须使用 `#[wasm_bindgen]` 宏
2. **错误处理**: 使用 `map_err(|e| JsValue::from_str(&e.to_string()))` 转换错误
3. **日志输出**: 使用 `web_sys::console::log_1()` 或 `console_log!` 宏
4. **二进制数据**: 输入使用 `&[u8]`，输出使用 `Vec<u8>` 或 `js_sys::Uint8Array`

## 版本发布流程

1. 修改 `Cargo.toml` 中的 `version` 字段
2. 运行 `./publish.sh` 发布
3. 提交代码: `git add . && git commit -m 'chore: release vX.X.X'`
4. 打标签: `git tag vX.X.X`
5. 推送: `git push origin main --tags`
