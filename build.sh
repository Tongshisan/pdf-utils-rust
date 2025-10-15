#!/bin/bash

# 构建 WASM 包的脚本

set -e

echo "🔨 开始构建 WASM 包..."

# 检查是否安装了 wasm-pack
if ! command -v wasm-pack &> /dev/null
then
    echo "❌ wasm-pack 未安装"
    echo "📦 正在安装 wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 构建用于 web 的版本
echo "🌐 构建 Web 版本..."
wasm-pack build --target web --out-dir pkg/web

# 构建用于 Node.js 的版本
echo "📦 构建 Node.js 版本..."
wasm-pack build --target nodejs --out-dir pkg/nodejs

# 构建用于打包工具的版本
echo "📦 构建 Bundler 版本..."
wasm-pack build --target bundler --out-dir pkg/bundler

echo "✅ 构建完成！"
echo ""
echo "📦 生成的包位于："
echo "  - pkg/web/     (用于浏览器)"
echo "  - pkg/nodejs/  (用于 Node.js)"
echo "  - pkg/bundler/ (用于 Webpack/Rollup 等)"

