# 🚀 完整安装指南（中文）

## ⚠️ 遇到的问题

如果你看到以下错误：

- `cargo: command not found` - Rust 未安装
- `wasm-pack` 编译失败 - 需要特殊处理

## 📦 方案一：使用预编译的 wasm-pack（推荐）

### 步骤 1: 安装 Rust

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 按提示选择默认安装（选项 1）
# 安装完成后，运行：
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 步骤 2: 安装 wasm-pack（使用预编译版本）

```bash
# macOS/Linux - 使用官方安装脚本
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 如果上面的方法失败，使用 npm 安装（需要 Node.js）
npm install -g wasm-pack

# 或者使用 Homebrew（仅 macOS）
brew install wasm-pack
```

### 步骤 3: 添加 WASM 目标

```bash
rustup target add wasm32-unknown-unknown
```

### 步骤 4: 构建项目

```bash
cd /Users/lizhi/Documents/lizhi/github/pdf-utils-rust

# 方式 1: 使用构建脚本
chmod +x build.sh
./build.sh

# 方式 2: 手动构建（如果脚本失败）
wasm-pack build --target web --out-dir pkg/web
```

## 📦 方案二：手动编译 wasm-pack（如果方案一失败）

如果预编译版本不工作，可以尝试手动编译：

```bash
# 安装 wasm-pack 的依赖
cargo install wasm-pack --locked

# 如果还是失败，尝试安装特定版本
cargo install wasm-pack --version 0.12.1
```

## 📦 方案三：不使用 wasm-pack（手动构建）

如果 wasm-pack 无法安装，可以手动构建：

```bash
# 1. 安装 wasm-bindgen-cli
cargo install wasm-bindgen-cli

# 2. 构建 WASM
cargo build --target wasm32-unknown-unknown --release

# 3. 生成绑定
wasm-bindgen target/wasm32-unknown-unknown/release/pdf_utils_rust.wasm \
  --out-dir pkg/web \
  --target web
```

## 🔧 常见问题修复

### 问题 1: SSL/TLS 错误

```bash
# macOS
brew install openssl

# 设置环境变量
export OPENSSL_DIR=/usr/local/opt/openssl
```

### 问题 2: 链接器错误

```bash
# macOS
xcode-select --install

# Linux
sudo apt-get install build-essential
```

### 问题 3: 权限问题

```bash
# 使用用户目录安装
cargo install wasm-pack --root ~/.cargo
```

### 问题 4: 网络问题（中国大陆）

```bash
# 配置 Rust 国内镜像
cat >> ~/.cargo/config.toml << EOF
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

[net]
git-fetch-with-cli = true
EOF
```

## ✅ 验证安装

运行以下命令检查是否安装成功：

```bash
# 检查 Rust
rustc --version
cargo --version

# 检查 wasm-pack
wasm-pack --version

# 检查 WASM 目标
rustup target list | grep wasm32-unknown-unknown
```

预期输出：

```
rustc 1.75.0 (或更高版本)
cargo 1.75.0 (或更高版本)
wasm-pack 0.12.0 (或更高版本)
wasm32-unknown-unknown (installed)
```

## 🎯 完整构建流程

确认所有工具安装成功后：

```bash
# 1. 进入项目目录
cd /Users/lizhi/Documents/lizhi/github/pdf-utils-rust

# 2. 清理之前的构建（可选）
cargo clean
rm -rf pkg/

# 3. 检查代码（确保没有错误）
cargo check --target wasm32-unknown-unknown

# 4. 构建 Web 版本
wasm-pack build --target web --out-dir pkg/web

# 5. 如果需要其他版本
wasm-pack build --target nodejs --out-dir pkg/nodejs
wasm-pack build --target bundler --out-dir pkg/bundler
```

## 🌐 运行示例

构建成功后：

```bash
# 启动本地服务器
python3 -m http.server 8000

# 在浏览器中打开
# http://localhost:8000/examples/index.html
```

## 💡 快速诊断脚本

创建一个诊断脚本来检查环境：

```bash
cat > check_env.sh << 'EOF'
#!/bin/bash

echo "🔍 检查开发环境..."
echo ""

# 检查 Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "❌ Rust 未安装"
    echo "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

# 检查 Cargo
if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo 未安装"
fi

# 检查 wasm-pack
if command -v wasm-pack &> /dev/null; then
    echo "✅ wasm-pack: $(wasm-pack --version)"
else
    echo "❌ wasm-pack 未安装"
    echo "   安装命令: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    echo "   或: npm install -g wasm-pack"
fi

# 检查 WASM 目标
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "✅ wasm32-unknown-unknown 目标已安装"
else
    echo "❌ wasm32-unknown-unknown 目标未安装"
    echo "   安装命令: rustup target add wasm32-unknown-unknown"
fi

# 检查 Python（用于运行示例服务器）
if command -v python3 &> /dev/null; then
    echo "✅ Python3: $(python3 --version)"
else
    echo "⚠️  Python3 未安装（可选，用于运行示例）"
fi

echo ""
echo "📝 总结："
if command -v rustc &> /dev/null && command -v wasm-pack &> /dev/null; then
    echo "✅ 环境配置完成！可以开始构建项目。"
    echo "   运行: ./build.sh 或 wasm-pack build --target web"
else
    echo "❌ 环境未完全配置，请按照上面的提示安装缺失的工具。"
fi
EOF

chmod +x check_env.sh
./check_env.sh
```

## 🆘 仍然遇到问题？

### 最小化测试

创建一个最小的测试项目：

```bash
# 创建测试目录
mkdir -p ~/test-wasm && cd ~/test-wasm

# 创建最小项目
cargo new --lib test-wasm-lib
cd test-wasm-lib

# 修改 Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "test-wasm-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
EOF

# 修改 src/lib.rs
cat > src/lib.rs << 'EOF'
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
EOF

# 尝试构建
wasm-pack build --target web

# 如果成功，说明环境没问题，可能是主项目的依赖问题
```

### 联系支持

如果以上方法都不行，可以：

1. 查看 [wasm-pack Issues](https://github.com/rustwasm/wasm-pack/issues)
2. 查看 [Rust Issues](https://github.com/rust-lang/rust/issues)
3. 在项目中创建 Issue 描述你的问题

---

**提示**: 最简单的方法是使用 npm 安装 wasm-pack（如果你已经安装了 Node.js）：

```bash
npm install -g wasm-pack
```

这通常比从源码编译更可靠。
