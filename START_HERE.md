# 🚀 从这里开始！

## ✅ 已完成

- ✅ 项目代码已修复（API 兼容性问题已解决）
- ✅ 环境检查脚本已创建

## 📝 需要做的事情（按顺序执行）

### 第 1 步：安装 Rust

```bash
# 运行这个命令安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装过程中：
# - 选择选项 1（默认安装）
# - 等待安装完成（大约 2-5 分钟）

# 安装完成后，运行这个命令让 Rust 生效
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

预期输出：

```
rustc 1.80.0 (或更高版本)
cargo 1.80.0 (或更高版本)
```

### 第 2 步：安装 wasm-pack（推荐使用 npm）

由于你已经安装了 Node.js，最简单的方法是：

```bash
npm install -g wasm-pack
```

或者使用官方安装脚本：

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

验证安装：

```bash
wasm-pack --version
```

### 第 3 步：添加 WASM 编译目标

```bash
rustup target add wasm32-unknown-unknown
```

### 第 4 步：再次检查环境

```bash
./check_env.sh
```

应该看到所有项目都显示 ✅

### 第 5 步：构建项目！

```bash
# 方式 1：使用构建脚本（推荐）
./build.sh

# 方式 2：手动构建 Web 版本
wasm-pack build --target web --out-dir pkg/web

# 方式 3：构建所有版本
wasm-pack build --target web --out-dir pkg/web
wasm-pack build --target nodejs --out-dir pkg/nodejs
wasm-pack build --target bundler --out-dir pkg/bundler
```

### 第 6 步：运行示例

```bash
# 启动本地服务器
python3 -m http.server 8000

# 然后在浏览器中打开
# http://localhost:8000/examples/index.html
```

## 🎉 成功标志

构建成功后，你会看到：

- ✅ `pkg/web/` 目录被创建
- ✅ 包含 `.wasm` 和 `.js` 文件
- ✅ 示例页面可以正常运行

## ⚠️ 如果遇到问题

### 问题 1：Rust 安装失败

**症状**：`curl: command not found` 或网络连接错误

**解决方案**：

1. 检查网络连接
2. 使用 VPN（如果在中国大陆）
3. 或者从 [Rust 官网](https://www.rust-lang.org/tools/install) 下载安装包

### 问题 2：wasm-pack 安装失败

**症状**：编译错误或依赖问题

**解决方案**：

```bash
# 使用 npm 安装（推荐）
npm install -g wasm-pack

# 如果没有 npm，安装 Node.js
brew install node  # macOS
```

### 问题 3：构建时网络错误

**症状**：`error: failed to download` 或 `dns error`

**解决方案**：

```bash
# 配置国内镜像（如果在中国大陆）
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
EOF

# 然后重试构建
```

### 问题 4：`edition2024` 错误

**症状**：`feature edition2024 is required`

**解决方案**：

```bash
# 更新 Rust 到最新版本
rustup update stable

# 清理缓存
cargo clean
rm -rf ~/.cargo/registry/cache
rm -rf ~/.cargo/registry/src

# 重试构建
```

## 📚 更多帮助

- 查看 `INSTALL_GUIDE_CN.md` - 详细安装指南
- 查看 `QUICKSTART.md` - 快速开始指南
- 查看 `README.md` - 完整文档
- 查看 `USAGE_CN.md` - 使用示例

## 🆘 还有问题？

1. 运行 `./check_env.sh` 查看环境状态
2. 查看错误信息
3. 在项目中创建 Issue

---

## 📌 快速命令参考

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. 安装 wasm-pack
npm install -g wasm-pack

# 3. 添加 WASM 目标
rustup target add wasm32-unknown-unknown

# 4. 检查环境
./check_env.sh

# 5. 构建项目
./build.sh

# 6. 运行示例
python3 -m http.server 8000
# 打开 http://localhost:8000/examples/index.html
```

祝你使用愉快！🎉
