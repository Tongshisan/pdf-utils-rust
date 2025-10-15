# 快速开始指南

## 📋 系统要求

- macOS / Linux / Windows
- 稳定的网络连接（用于下载依赖）
- 至少 2GB 可用磁盘空间

## 🔧 安装开发环境

### 第一步：安装 Rust

在终端运行以下命令：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重启终端或运行：

```bash
source $HOME/.cargo/env
```

验证安装：

```bash
rustc --version
cargo --version
```

### 第二步：安装 wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

验证安装：

```bash
wasm-pack --version
```

### 第三步：添加 WASM 目标

```bash
rustup target add wasm32-unknown-unknown
```

## 🚀 构建项目

### 方式一：使用构建脚本（推荐）

```bash
cd /Users/lizhi/Documents/lizhi/github/pdf-utils-rust
chmod +x build.sh
./build.sh
```

### 方式二：手动构建

```bash
# 检查代码（可选）
cargo check

# 构建 WASM 包
wasm-pack build --target web --out-dir pkg/web
```

## 🌐 运行示例

### 1. 启动本地服务器

**使用 Python 3（推荐）：**

```bash
python3 -m http.server 8000
```

**使用 Node.js：**

```bash
npx http-server -p 8000
```

**使用 PHP：**

```bash
php -S localhost:8000
```

### 2. 打开浏览器

访问：`http://localhost:8000/examples/index.html`

## 📦 使用在你的项目中

### 在 HTML 中使用

```html
<script type="module">
  import init, * as wasm from "./pkg/web/pdf_utils_rust.js";

  async function main() {
    await init();
    // 现在可以使用 wasm 的所有功能了
    console.log("WASM 加载成功！");
  }

  main();
</script>
```

### 在 Node.js 中使用

```javascript
const wasm = require("./pkg/nodejs/pdf_utils_rust");

// 直接使用
const merged = wasm.merge_pdfs([pdf1, pdf2]);
```

## ⚠️ 常见问题

### 问题 1: 找不到 cargo 命令

**解决方法：**

```bash
# 确保 Rust 已正确安装
source $HOME/.cargo/env

# 或重启终端
```

### 问题 2: wasm-pack 构建失败

**解决方法：**

```bash
# 确保已添加 wasm32 目标
rustup target add wasm32-unknown-unknown

# 清理并重新构建
cargo clean
wasm-pack build --target web
```

### 问题 3: CORS 错误

**解决方法：**

- 不要直接打开 HTML 文件（file://）
- 必须通过 HTTP 服务器访问（http://localhost）

### 问题 4: 依赖下载慢

**解决方法：**

```bash
# 使用国内镜像源
# 编辑 ~/.cargo/config.toml，添加：
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

## 🎯 验证安装

运行以下命令来验证一切正常：

```bash
# 检查项目
cd /Users/lizhi/Documents/lizhi/github/pdf-utils-rust
cargo check

# 运行测试
cargo test

# 构建 WASM
wasm-pack build --target web --out-dir pkg/web
```

如果所有命令都成功执行，说明环境配置完成！

## 📚 下一步

- 查看 [README.md](README.md) 了解完整功能
- 查看 [USAGE_CN.md](USAGE_CN.md) 学习详细用法
- 运行示例并修改代码进行实验
- 将功能集成到你的项目中

## 💡 开发建议

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 查看文档

```bash
cargo doc --open
```

### 监听文件变化自动构建

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 监听并自动构建
cargo watch -x 'build'
```

## 🆘 获取帮助

- 📖 查看文档：[README.md](README.md)
- 🐛 报告问题：创建 GitHub Issue
- 💬 讨论交流：GitHub Discussions
- 📧 直接联系：提交 Pull Request

---

祝你使用愉快！如果有任何问题，随时寻求帮助。🎉
