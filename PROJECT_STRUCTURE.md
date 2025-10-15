# 项目结构说明

## 📁 目录结构

```
pdf-utils-rust/
│
├── .cargo/
│   └── config.toml              # Cargo 构建配置
│
├── .github/
│   └── workflows/
│       └── build.yml            # GitHub Actions CI/CD 配置
│
├── .vscode/
│   └── settings.json            # VS Code 编辑器配置
│
├── examples/
│   ├── index.html               # Web 演示示例（完整的交互界面）
│   └── README.md                # 示例使用说明
│
├── src/
│   ├── lib.rs                   # 主库文件，WASM 入口
│   ├── pdf_utils.rs             # PDF 处理功能模块
│   └── image_utils.rs           # 图片处理功能模块
│
├── tests/
│   └── integration_test.rs      # 集成测试
│
├── pkg/                         # 构建输出目录（.gitignore）
│   ├── web/                     # 浏览器版本
│   ├── nodejs/                  # Node.js 版本
│   └── bundler/                 # 打包工具版本
│
├── .gitignore                   # Git 忽略配置
├── build.sh                     # 构建脚本（可执行）
├── Cargo.toml                   # Rust 项目配置
├── CONTRIBUTING.md              # 贡献指南
├── LICENSE                      # MIT 许可证
├── package.json                 # NPM 包配置
├── PROJECT_STRUCTURE.md         # 本文件
├── QUICKSTART.md                # 快速开始指南
├── README.md                    # 项目主文档（英文）
├── rust-toolchain.toml          # Rust 工具链配置
└── USAGE_CN.md                  # 详细使用指南（中文）
```

## 📄 文件说明

### 核心源代码

#### `src/lib.rs`

- **作用**: 库的入口文件
- **功能**:
  - WASM 初始化
  - 模块导出
  - panic hook 设置

#### `src/pdf_utils.rs`

- **作用**: PDF 处理功能模块
- **功能**:
  - `merge_pdfs()` - 合并多个 PDF
  - `split_pdf()` - 分割 PDF 为单页
  - `split_pdf_by_range()` - 按范围分割 PDF
  - `get_pdf_page_count()` - 获取页数

#### `src/image_utils.rs`

- **作用**: 图片处理功能模块
- **功能**:
  - `images_to_pdf()` - 图片转 PDF
  - `convert_image_format()` - 格式转换
  - `resize_image()` - 调整大小
  - `compress_image()` - 压缩图片
  - `rotate_image()` - 旋转图片
  - `crop_image()` - 裁剪图片
  - `get_image_info()` - 获取图片信息

### 配置文件

#### `Cargo.toml`

- **作用**: Rust 项目配置
- **内容**:
  - 项目元数据
  - 依赖声明
  - 构建配置
  - 优化配置

#### `rust-toolchain.toml`

- **作用**: 指定 Rust 工具链版本
- **内容**:
  - 使用 stable 版本
  - 包含 rustfmt 和 clippy
  - 添加 wasm32 目标

#### `.cargo/config.toml`

- **作用**: Cargo 构建配置
- **内容**:
  - 默认编译目标为 wasm32
  - 链接器参数优化

#### `package.json`

- **作用**: NPM 包配置
- **内容**:
  - 包元数据
  - 构建脚本
  - 发布配置

### 构建和部署

#### `build.sh`

- **作用**: 一键构建脚本
- **功能**:
  - 检查 wasm-pack
  - 构建三个目标版本
  - 输出构建结果

#### `.github/workflows/build.yml`

- **作用**: CI/CD 自动化
- **功能**:
  - 自动构建和测试
  - 生成构件上传

### 文档

#### `README.md`

- **作用**: 项目主文档
- **内容**:
  - 功能介绍
  - 快速开始
  - API 文档
  - 使用示例

#### `QUICKSTART.md`

- **作用**: 快速开始指南
- **内容**:
  - 环境安装
  - 构建步骤
  - 常见问题

#### `USAGE_CN.md`

- **作用**: 详细使用指南
- **内容**:
  - 各种使用场景
  - 代码示例
  - 最佳实践

#### `CONTRIBUTING.md`

- **作用**: 贡献指南
- **内容**:
  - 贡献流程
  - 代码规范
  - 提交规范

### 示例

#### `examples/index.html`

- **作用**: 完整的 Web 演示
- **功能**:
  - 所有功能的交互界面
  - 美观的 UI 设计
  - 实时处理和下载

## 🔄 工作流程

### 开发流程

```
编写代码 → 格式化 → 检查 → 测试 → 构建 → 部署
   ↓         ↓        ↓      ↓      ↓       ↓
src/*.rs   cargo fmt  cargo  cargo  wasm-   pkg/
                      clippy test   pack
```

### 构建流程

```
源代码 (src/)
    ↓
cargo build
    ↓
wasm-pack build
    ↓
WASM 包 (pkg/)
    ├── web/      (浏览器)
    ├── nodejs/   (Node.js)
    └── bundler/  (打包工具)
```

### 使用流程

```
用户上传文件
    ↓
JavaScript 读取文件
    ↓
转换为 Uint8Array
    ↓
调用 WASM 函数
    ↓
Rust 处理数据
    ↓
返回处理结果
    ↓
JavaScript 接收结果
    ↓
触发下载或显示
```

## 📦 依赖说明

### Rust 依赖

#### 核心依赖

- `wasm-bindgen` - Rust 和 JavaScript 互操作
- `js-sys` - JavaScript 标准对象绑定
- `web-sys` - Web API 绑定

#### 功能依赖

- `lopdf` - PDF 文件处理
- `image` - 图片处理

#### 辅助依赖

- `serde` / `serde_json` - 序列化和反序列化
- `console_error_panic_hook` - 错误处理

### 开发依赖

- `wasm-bindgen-test` - WASM 测试框架

## 🎯 编译目标

### wasm32-unknown-unknown

- **用途**: WebAssembly 编译目标
- **特点**:
  - 无操作系统依赖
  - 跨平台
  - 浏览器可执行

## 🔧 扩展建议

### 添加新功能

1. **创建新模块**

   ```
   src/new_module.rs
   ```

2. **在 lib.rs 中导入**

   ```rust
   mod new_module;
   pub use new_module::*;
   ```

3. **实现功能**

   ```rust
   #[wasm_bindgen]
   pub fn new_function(...) -> Result<...> {
       // 实现
   }
   ```

4. **更新文档**

   - README.md
   - API 文档

5. **添加测试**
   - 单元测试
   - 集成测试

### 优化性能

1. **使用 release 构建**

   ```toml
   [profile.release]
   opt-level = "z"
   lto = true
   ```

2. **减小 WASM 大小**

   - 使用 wasm-opt
   - 移除未使用的依赖
   - 使用 features 条件编译

3. **优化算法**
   - 使用高效数据结构
   - 减少内存分配
   - 并行处理（如果可能）

## 📊 项目统计

- **源代码文件**: 3
- **文档文件**: 7
- **配置文件**: 5
- **示例文件**: 2
- **总代码行数**: ~1000+ 行

## 🎓 学习资源

### Rust

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

### WebAssembly

- [WebAssembly 官网](https://webassembly.org/)
- [wasm-bindgen 文档](https://rustwasm.github.io/wasm-bindgen/)

### 相关库

- [lopdf 文档](https://docs.rs/lopdf/)
- [image 文档](https://docs.rs/image/)

---

如有问题或建议，欢迎提交 Issue！
